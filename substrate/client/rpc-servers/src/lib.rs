// This file is part of Substrate.

// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! Substrate RPC servers.

#![warn(missing_docs)]

pub mod middleware;

use std::{
	convert::Infallible,
	error::Error as StdError,
	net::{IpAddr, SocketAddr, ToSocketAddrs},
	num::NonZeroU32,
	time::Duration,
};

use http::header::HeaderValue;
use hyper::{
	server::conn::AddrStream,
	service::{make_service_fn, service_fn},
};
use jsonrpsee::{
	server::{
		middleware::http::{HostFilterLayer, ProxyGetRequestLayer},
		stop_channel, ws, PingConfig, StopHandle, TowerServiceBuilder,
	},
	Methods, RpcModule,
};
use tokio::net::TcpListener;
use tower::Service;
use tower_http::cors::{AllowOrigin, CorsLayer};

pub use jsonrpsee::{
	core::{
		id_providers::{RandomIntegerIdProvider, RandomStringIdProvider},
		traits::IdProvider,
	},
	server::{middleware::rpc::RpcServiceBuilder, BatchRequestConfig},
};
pub use middleware::{Metrics, MiddlewareLayer, RpcMetrics};

const MEGABYTE: u32 = 1024 * 1024;

/// Type alias for the JSON-RPC server.
pub type Server = jsonrpsee::server::ServerHandle;

/// RPC server configuration.
#[derive(Debug)]
pub struct Config<'a, M: Send + Sync + 'static> {
	/// Socket addresses.
	pub addrs: [SocketAddr; 2],
	/// CORS.
	pub cors: Option<&'a Vec<String>>,
	/// Maximum connections.
	pub max_connections: u32,
	/// Maximum subscriptions per connection.
	pub max_subs_per_conn: u32,
	/// Maximum rpc request payload size.
	pub max_payload_in_mb: u32,
	/// Maximum rpc response payload size.
	pub max_payload_out_mb: u32,
	/// Metrics.
	pub metrics: Option<RpcMetrics>,
	/// Message buffer size
	pub message_buffer_capacity: u32,
	/// RPC API.
	pub rpc_api: RpcModule<M>,
	/// Subscription ID provider.
	pub id_provider: Option<Box<dyn IdProvider>>,
	/// Tokio runtime handle.
	pub tokio_handle: tokio::runtime::Handle,
	/// Batch request config.
	pub batch_config: BatchRequestConfig,
	/// Rate limit calls per minute.
	pub rate_limit: Option<NonZeroU32>,
	/// Disable rate limit for hosts.
	pub rate_limit_whitelisted_hosts: &'a [String],
}

#[derive(Debug, Clone)]
struct PerConnection<RpcMiddleware, HttpMiddleware> {
	methods: Methods,
	stop_handle: StopHandle,
	metrics: Option<RpcMetrics>,
	tokio_handle: tokio::runtime::Handle,
	service_builder: TowerServiceBuilder<RpcMiddleware, HttpMiddleware>,
}

/// Start RPC server listening on given address.
pub async fn start_server<M>(
	config: Config<'_, M>,
) -> Result<Server, Box<dyn StdError + Send + Sync>>
where
	M: Send + Sync,
{
	let Config {
		addrs,
		batch_config,
		cors,
		max_payload_in_mb,
		max_payload_out_mb,
		max_connections,
		max_subs_per_conn,
		metrics,
		message_buffer_capacity,
		id_provider,
		tokio_handle,
		rpc_api,
		rate_limit,
		rate_limit_whitelisted_hosts,
	} = config;

	let std_listener = TcpListener::bind(addrs.as_slice()).await?.into_std()?;
	let local_addr = std_listener.local_addr().ok();
	let host_filter = hosts_filtering(cors.is_some(), local_addr);
	let rate_limit_whitelisted_ip_addrs = hosts_to_ip_addrs(rate_limit_whitelisted_hosts)?;

	let http_middleware = tower::ServiceBuilder::new()
		.option_layer(host_filter)
		// Proxy `GET /health` requests to internal `system_health` method.
		.layer(ProxyGetRequestLayer::new("/health", "system_health")?)
		.layer(try_into_cors(cors)?);

	let mut builder = jsonrpsee::server::Server::builder()
		.max_request_body_size(max_payload_in_mb.saturating_mul(MEGABYTE))
		.max_response_body_size(max_payload_out_mb.saturating_mul(MEGABYTE))
		.max_connections(max_connections)
		.max_subscriptions_per_connection(max_subs_per_conn)
		.enable_ws_ping(
			PingConfig::new()
				.ping_interval(Duration::from_secs(30))
				.inactive_limit(Duration::from_secs(60))
				.max_failures(3),
		)
		.set_http_middleware(http_middleware)
		.set_message_buffer_capacity(message_buffer_capacity)
		.set_batch_request_config(batch_config)
		.custom_tokio_runtime(tokio_handle.clone());

	if let Some(provider) = id_provider {
		builder = builder.set_id_provider(provider);
	} else {
		builder = builder.set_id_provider(RandomStringIdProvider::new(16));
	};

	let (stop_handle, server_handle) = stop_channel();
	let cfg = PerConnection {
		methods: build_rpc_api(rpc_api).into(),
		service_builder: builder.to_service_builder(),
		metrics,
		tokio_handle,
		stop_handle: stop_handle.clone(),
	};

	let make_service = make_service_fn(move |conn: &AddrStream| {
		let cfg = cfg.clone();
		let conn_ip = conn.remote_addr().ip();
		let rate_limit_whitelisted_ip_addrs = rate_limit_whitelisted_ip_addrs.clone();

		async move {
			let cfg = cfg.clone();
			let rate_limit_whitelisted_ip_addrs = rate_limit_whitelisted_ip_addrs.clone();

			Ok::<_, Infallible>(service_fn(move |req| {
				let ip = read_ip(conn_ip, &req);

				let rate_limit_cfg = if rate_limit_whitelisted_ip_addrs.iter().any(|ip2| ip2 == &ip)
				{
					None
				} else {
					rate_limit
				};

				let PerConnection { service_builder, metrics, tokio_handle, stop_handle, methods } =
					cfg.clone();

				let is_websocket = ws::is_upgrade_request(&req);
				let transport_label = if is_websocket { "ws" } else { "http" };

				let middleware_layer = match (metrics, rate_limit_cfg) {
					(None, None) => None,
					(Some(metrics), None) => Some(
						MiddlewareLayer::new().with_metrics(Metrics::new(metrics, transport_label)),
					),
					(None, Some(rate_limit)) =>
						Some(MiddlewareLayer::new().with_rate_limit_per_minute(rate_limit)),
					(Some(metrics), Some(rate_limit)) => Some(
						MiddlewareLayer::new()
							.with_metrics(Metrics::new(metrics, transport_label))
							.with_rate_limit_per_minute(rate_limit),
					),
				};

				let rpc_middleware =
					RpcServiceBuilder::new().option_layer(middleware_layer.clone());

				let mut svc =
					service_builder.set_rpc_middleware(rpc_middleware).build(methods, stop_handle);

				async move {
					if is_websocket {
						let on_disconnect = svc.on_session_closed();

						// Spawn a task to handle when the connection is closed.
						tokio_handle.spawn(async move {
							let now = std::time::Instant::now();
							middleware_layer.as_ref().map(|m| m.ws_connect());
							on_disconnect.await;
							middleware_layer.as_ref().map(|m| m.ws_disconnect(now));
						});
					}

					svc.call(req).await
				}
			}))
		}
	});

	let server = hyper::Server::from_tcp(std_listener)?.serve(make_service);

	tokio::spawn(async move {
		let graceful = server.with_graceful_shutdown(async move { stop_handle.shutdown().await });
		let _ = graceful.await;
	});

	log::info!(
		"Running JSON-RPC server: addr={}, allowed origins={}",
		local_addr.map_or_else(|| "unknown".to_string(), |a| a.to_string()),
		format_cors(cors)
	);

	Ok(server_handle)
}

fn hosts_filtering(enabled: bool, addr: Option<SocketAddr>) -> Option<HostFilterLayer> {
	// If the local_addr failed, fallback to wildcard.
	let port = addr.map_or("*".to_string(), |p| p.port().to_string());

	if enabled {
		// NOTE: The listening addresses are whitelisted by default.
		let hosts =
			[format!("localhost:{port}"), format!("127.0.0.1:{port}"), format!("[::1]:{port}")];
		Some(HostFilterLayer::new(hosts).expect("Valid hosts; qed"))
	} else {
		None
	}
}

fn build_rpc_api<M: Send + Sync + 'static>(mut rpc_api: RpcModule<M>) -> RpcModule<M> {
	let mut available_methods = rpc_api.method_names().collect::<Vec<_>>();
	// The "rpc_methods" is defined below and we want it to be part of the reported methods.
	available_methods.push("rpc_methods");
	available_methods.sort();

	rpc_api
		.register_method("rpc_methods", move |_, _| {
			serde_json::json!({
				"methods": available_methods,
			})
		})
		.expect("infallible all other methods have their own address space; qed");

	rpc_api
}

fn try_into_cors(
	maybe_cors: Option<&Vec<String>>,
) -> Result<CorsLayer, Box<dyn StdError + Send + Sync>> {
	if let Some(cors) = maybe_cors {
		let mut list = Vec::new();
		for origin in cors {
			list.push(HeaderValue::from_str(origin)?);
		}
		Ok(CorsLayer::new().allow_origin(AllowOrigin::list(list)))
	} else {
		// allow all cors
		Ok(CorsLayer::permissive())
	}
}

fn format_cors(maybe_cors: Option<&Vec<String>>) -> String {
	if let Some(cors) = maybe_cors {
		format!("{:?}", cors)
	} else {
		format!("{:?}", ["*"])
	}
}

/// Helper function that tries to read the ip addr from "X-Real-IP" header
/// which is only set if the connection was made via a reverse-proxy
///
/// If that header is missing then remote addr from the socket is used.
fn read_ip(remote_addr: IpAddr, req: &hyper::Request<hyper::Body>) -> IpAddr {
	if let Some(ip) = req
		.headers()
		.get("X-Real-IP")
		.and_then(|v| v.to_str().ok())
		.and_then(|s| s.parse().ok())
	{
		ip
	} else {
		remote_addr
	}
}

fn hosts_to_ip_addrs(hosts: &[String]) -> Result<Vec<IpAddr>, Box<dyn StdError + Send + Sync>> {
	let mut ip_list = Vec::new();

	for host in hosts {
		// The host may contain a port such as `hostname:8080`
		// and we don't care about the port to lookup the IP addr.
		//
		// to_socket_addr without the port will fail though
		let host_no_port = if let Some((h, _port)) = host.split_once(":") { h } else { host };

		let sockaddrs = (host_no_port, 0).to_socket_addrs()?;

		for sockaddr in sockaddrs {
			ip_list.push(sockaddr.ip());
		}
	}

	Ok(ip_list)
}
