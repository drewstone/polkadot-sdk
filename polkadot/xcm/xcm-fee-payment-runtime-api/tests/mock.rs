// Copyright (C) Parity Technologies (UK) Ltd.
// This file is part of Polkadot.

// Substrate is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Substrate is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Polkadot.  If not, see <http://www.gnu.org/licenses/>.

//! Mock runtime for tests.
//! Implements both runtime APIs for fee estimation and getting the messages for transfers.

use codec::Encode;
use frame_system::{EnsureRoot, RawOrigin as SystemRawOrigin};
use frame_support::{
    construct_runtime, derive_impl, parameter_types, assert_ok,
    traits::{Nothing, ConstU32, ConstU128, OriginTrait, ContainsPair, Everything, Equals},
    weights::WeightToFee as WeightToFeeT,
};
use pallet_xcm::TestWeightInfo;
use sp_runtime::{traits::{IdentityLookup, Block as BlockT, TryConvert, Get}, SaturatedConversion, BuildStorage};
use sp_std::{cell::RefCell, marker::PhantomData};
use xcm::{prelude::*, Version as XcmVersion};
use xcm_builder::{
    EnsureXcmOrigin, FixedWeightBounds, IsConcrete, FungibleAdapter, MintLocation,
    AllowTopLevelPaidExecutionFrom, TakeWeightCredit,
};
use xcm_executor::{XcmExecutor, traits::ConvertLocation};

use xcm_fee_payment_runtime_api::{XcmDryRunApi, XcmPaymentApi, XcmPaymentApiError, XcmDryRunEffects};

construct_runtime! {
    pub enum TestRuntime {
        System: frame_system,
        Balances: pallet_balances,
        XcmPallet: pallet_xcm,
    }
}

pub type SignedExtra = (
	// frame_system::CheckEra<TestRuntime>,
	// frame_system::CheckNonce<TestRuntime>,
    frame_system::CheckWeight<TestRuntime>,
);
pub type TestXt = sp_runtime::testing::TestXt<RuntimeCall, SignedExtra>;
type Block = sp_runtime::testing::Block<TestXt>;
type Balance = u128;
type AccountId = u64;

pub(crate) fn extra() -> SignedExtra {
    (
        frame_system::CheckWeight::new(),
    )
}

type Executive = frame_executive::Executive<
    TestRuntime,
    Block,
    frame_system::ChainContext<TestRuntime>,
    TestRuntime,
    AllPalletsWithSystem,
    (),
>;

#[derive_impl(frame_system::config_preludes::TestDefaultConfig)]
impl frame_system::Config for TestRuntime {
    type Block = Block;
    type AccountId = AccountId;
    type AccountData = pallet_balances::AccountData<Balance>;
    type Lookup = IdentityLookup<AccountId>;
}

#[derive_impl(pallet_balances::config_preludes::TestDefaultConfig)]
impl pallet_balances::Config for TestRuntime {
    type AccountStore = System;
    type Balance = Balance;
    type ExistentialDeposit = ExistentialDeposit;
}

thread_local! {
    pub static SENT_XCM: RefCell<Vec<(Location, Xcm<()>)>> = RefCell::new(Vec::new());
}

pub(crate) fn sent_xcm() -> Vec<(Location, Xcm<()>)> {
    SENT_XCM.with(|q| (*q.borrow()).clone())
}

pub struct TestXcmSender;
impl SendXcm for TestXcmSender {
    type Ticket = (Location, Xcm<()>);
    fn validate(
        dest: &mut Option<Location>,
        msg: &mut Option<Xcm<()>>,
    ) -> SendResult<Self::Ticket> {
        let ticket = (dest.take().unwrap(), msg.take().unwrap());
        let fees: Assets = (HereLocation::get(), DeliveryFees::get()).into();
        Ok((ticket, fees))
    }
    fn deliver(ticket: Self::Ticket) -> Result<XcmHash, SendError> {
        let hash = fake_message_hash(&ticket.1);
        SENT_XCM.with(|q| q.borrow_mut().push(ticket));
        Ok(hash)
    }
}

fn fake_message_hash<Call>(message: &Xcm<Call>) -> XcmHash {
    message.using_encoded(sp_io::hashing::blake2_256)
}

pub type XcmRouter = TestXcmSender;

parameter_types! {
    pub const DeliveryFees: u128 = 20; // Random value.
    pub const ExistentialDeposit: u128 = 1; // Random value.
    pub const BaseXcmWeight: Weight = Weight::from_parts(100, 10); // Random value.
    pub const MaxInstructions: u32 = 100;
    pub UniversalLocation: InteriorLocation = [GlobalConsensus(NetworkId::Westend), Parachain(2000)].into();
    pub static AdvertisedXcmVersion: XcmVersion = 4;
    pub const HereLocation: Location = Location::here();
    pub const RelayLocation: Location = Location::parent();
    pub const MaxAssetsIntoHolding: u32 = 64;
    pub CheckAccount: AccountId = XcmPallet::check_account();
    pub LocalCheckAccount: (AccountId, MintLocation) = (CheckAccount::get(), MintLocation::Local);
    pub const AnyNetwork: Option<NetworkId> = None;
}

/// Simple `WeightToFee` implementation that adds the ref_time by the proof_size.
pub struct WeightToFee;
impl WeightToFeeT for WeightToFee {
    type Balance = Balance;
    fn weight_to_fee(weight: &Weight) -> Self::Balance {
        Self::Balance::saturated_from(weight.ref_time())
            .saturating_add(Self::Balance::saturated_from(weight.proof_size()))
    }
}

type Weigher = FixedWeightBounds<BaseXcmWeight, RuntimeCall, MaxInstructions>;

/// Matches the pair (NativeToken, AssetHub).
/// This is used in the `IsTeleporter` configuration item, meaning we accept our native token
/// coming from AssetHub as a teleport.
pub struct NativeTokenToAssetHub;
impl ContainsPair<Asset, Location> for NativeTokenToAssetHub {
    fn contains(asset: &Asset, origin: &Location) -> bool {
        matches!(asset.id.0.unpack(), (0, []))
            && matches!(origin.unpack(), (1, [Parachain(1000)]))
    }
}

/// Matches the pair (RelayToken, AssetHub).
/// This is used in the `IsReserve` configuration item, meaning we accept the relay token
/// coming from AssetHub as a reserve asset transfer.
pub struct RelayTokenToAssetHub;
impl ContainsPair<Asset, Location> for RelayTokenToAssetHub {
    fn contains(asset: &Asset, origin: &Location) -> bool {
        matches!(asset.id.0.unpack(), (1, []))
            && matches!(origin.unpack(), (1, [Parachain(1000)]))
    }
}

/// Converts locations that are only the `AccountIndex64` junction into local u64 accounts.
pub struct AccountIndex64Aliases<Network, AccountId>(PhantomData<(Network, AccountId)>);
impl<Network: Get<Option<NetworkId>>, AccountId: From<u64>>
    ConvertLocation<AccountId> for AccountIndex64Aliases<Network, AccountId>
{
    fn convert_location(location: &Location) -> Option<AccountId> {
        let index = match location.unpack() {
            (0, [AccountIndex64 { index, network: None }]) => index,
            (0, [AccountIndex64 { index, network }]) if *network == Network::get() => index,
            _ => return None,
        };
        Some((*index).into())
    }
}

/// We only alias local account locations to actual local accounts.
/// We don't allow sovereign accounts for locations outside our chain.
pub type LocationToAccountId = AccountIndex64Aliases<AnyNetwork, u64>;

pub type FungibleTransactor = FungibleAdapter<
    // We use pallet-balances for handling this fungible asset.
    Balances,
    // The fungible asset handled by this transactor is the native token of the chain.
    IsConcrete<HereLocation>,
    // How we convert locations to accounts.
    LocationToAccountId,
    // We need to specify the AccountId type.
    AccountId,
    // We mint the native tokens locally, so we track how many we've sent away via teleports.
    LocalCheckAccount,
>;

// TODO: Handle the relay chain asset so we can also test with
// reserve asset transfers.
pub type AssetTransactor = FungibleTransactor;

pub type Barrier = (
    TakeWeightCredit, // We need this for pallet-xcm's extrinsics to work.
    AllowTopLevelPaidExecutionFrom<Equals<HereLocation>>, // TODO: Technically, we should allow messages from "AssetHub".
);

pub struct XcmConfig;
impl xcm_executor::Config for XcmConfig {
    type RuntimeCall = RuntimeCall;
    type XcmSender = XcmRouter;
    type AssetTransactor = AssetTransactor;
    type OriginConverter = ();
    type IsReserve = ();
    type IsTeleporter = NativeTokenToAssetHub;
    type UniversalLocation = UniversalLocation;
    type Barrier = Barrier;
    type Weigher = Weigher;
    type Trader = ();
    type ResponseHandler = ();
    type AssetTrap = ();
    type AssetLocker = ();
    type AssetExchanger = ();
    type AssetClaims = ();
    type SubscriptionService = ();
    type PalletInstancesInfo = AllPalletsWithSystem;
    type MaxAssetsIntoHolding = MaxAssetsIntoHolding;
    type FeeManager = ();
    type MessageExporter = ();
    type UniversalAliases = ();
    type CallDispatcher = RuntimeCall;
    type SafeCallFilter = Nothing;
    type Aliasers = Nothing;
    type TransactionalProcessor = ();
    type HrmpNewChannelOpenRequestHandler = ();
	type HrmpChannelAcceptedHandler = ();
	type HrmpChannelClosingHandler = ();
}

/// Converts a signed origin of a u64 account into a location with only the `AccountIndex64` junction.
pub struct SignedToAccountIndex64<RuntimeOrigin, AccountId>(PhantomData<(RuntimeOrigin, AccountId)>);
impl<
    RuntimeOrigin: OriginTrait + Clone,
    AccountId: Into<u64>,
> TryConvert<RuntimeOrigin, Location> for SignedToAccountIndex64<RuntimeOrigin, AccountId>
where
	RuntimeOrigin::PalletsOrigin: From<SystemRawOrigin<AccountId>>
		+ TryInto<SystemRawOrigin<AccountId>, Error = RuntimeOrigin::PalletsOrigin>,
{
    fn try_convert(origin: RuntimeOrigin) -> Result<Location, RuntimeOrigin> {
        origin.try_with_caller(|caller| match caller.try_into() {
            Ok(SystemRawOrigin::Signed(who)) =>
                Ok(Junction::AccountIndex64 { network: None, index: who.into() }.into()),
            Ok(other) => Err(other.into()),
            Err(other) => Err(other),
        })
    }
}

pub type LocalOriginToLocation = SignedToAccountIndex64<RuntimeOrigin, AccountId>;

impl pallet_xcm::Config for TestRuntime {
    type RuntimeEvent = RuntimeEvent;
    type SendXcmOrigin = EnsureXcmOrigin<RuntimeOrigin, ()>;
    type XcmRouter = XcmRouter;
    type ExecuteXcmOrigin = EnsureXcmOrigin<RuntimeOrigin, LocalOriginToLocation>;
    type XcmExecuteFilter = Nothing;
    type XcmExecutor = XcmExecutor<XcmConfig>;
    type XcmTeleportFilter = Everything; // Put everything instead of something more restricted.
    type XcmReserveTransferFilter = Everything; // Same.
    type Weigher = Weigher;
    type UniversalLocation = UniversalLocation;
    type RuntimeOrigin = RuntimeOrigin;
    type RuntimeCall = RuntimeCall;
    const VERSION_DISCOVERY_QUEUE_SIZE: u32 = 100;
    type AdvertisedXcmVersion = AdvertisedXcmVersion;
    type AdminOrigin = EnsureRoot<AccountId>;
    type TrustedLockers = ();
    type SovereignAccountOf = ();
    type Currency = Balances;
    type CurrencyMatcher = IsConcrete<HereLocation>;
    type MaxLockers = ConstU32<0>;
    type MaxRemoteLockConsumers = ConstU32<0>;
    type RemoteLockConsumerIdentifier = ();
    type WeightInfo = TestWeightInfo;
}

pub fn new_test_ext_with_balances(balances: Vec<(AccountId, Balance)>) -> sp_io::TestExternalities {
    let mut t = frame_system::GenesisConfig::<TestRuntime>::default().build_storage().unwrap();

    pallet_balances::GenesisConfig::<TestRuntime> { balances }
        .assimilate_storage(&mut t)
        .unwrap();

    let mut ext = sp_io::TestExternalities::new(t);
    ext.execute_with(|| System::set_block_number(1));
    ext
}

#[derive(Clone)]
pub(crate) struct TestClient;

pub(crate) struct RuntimeApi {
    _inner: TestClient,
}

impl sp_api::ProvideRuntimeApi<Block> for TestClient {
    type Api = RuntimeApi;
    fn runtime_api(&self) -> sp_api::ApiRef<Self::Api> {
        RuntimeApi { _inner: self.clone() }.into()
    }
}

sp_api::mock_impl_runtime_apis! {
    impl XcmPaymentApi<Block> for RuntimeApi {
        fn query_acceptable_payment_assets(xcm_version: XcmVersion) -> Result<Vec<VersionedAssetId>, XcmPaymentApiError> {
            todo!()
        }

        fn query_xcm_weight(message: VersionedXcm<()>) -> Result<Weight, XcmPaymentApiError> {
            XcmPallet::query_xcm_weight(message)
        }

        fn query_weight_to_asset_fee(weight: Weight, asset: VersionedAssetId) -> Result<u128, XcmPaymentApiError> {
            let local_asset = VersionedAssetId::V4(HereLocation::get().into());
            let asset = asset
                .into_version(4)
                .map_err(|_| XcmPaymentApiError::VersionedConversionFailed)?;

            if asset != local_asset { return Err(XcmPaymentApiError::AssetNotFound); }

            Ok(WeightToFee::weight_to_fee(&weight))
        }

        fn query_delivery_fees(destination: VersionedLocation, message: VersionedXcm<()>) -> Result<VersionedAssets, XcmPaymentApiError> {
            XcmPallet::query_delivery_fees(destination, message)
        }
    }

    impl XcmDryRunApi<Block> for RuntimeApi {
        fn dry_run_extrinsic(extrinsic: <Block as BlockT>::Extrinsic) -> Result<XcmDryRunEffects, ()> {
            // First we execute the extrinsic to check the queue.
            match &extrinsic.call {
                RuntimeCall::XcmPallet(pallet_xcm::Call::transfer_assets {
                    dest: _dest,
                    beneficiary: _beneficiary,
                    assets,
                    fee_asset_item: _fee_asset_item,
                    weight_limit: _weight_limit,
                }) => {
                    let assets: Assets = (**assets).clone().try_into()?;
                    assert_ok!(Executive::apply_extrinsic(extrinsic)); // Asserting just because it's for tests.
                    let forwarded_messages = sent_xcm()
                        .into_iter()
                        .map(|(location, message)| (
                            VersionedLocation::V4(location),
                            VersionedXcm::V4(message)
                        )).collect();
                    Ok(XcmDryRunEffects {
                        local_program: VersionedXcm::V4(
                            Xcm::builder_unsafe()
                                .withdraw_asset(assets.clone())
                                .burn_asset(assets)
                                .build()
                            ),
                        forwarded_messages,
                    })
                },
                _ => Err(()),
            }
        }

        fn dry_run_xcm(xcm: Xcm<()>) -> Result<XcmDryRunEffects, ()> {
            todo!()
        }
    }
}
