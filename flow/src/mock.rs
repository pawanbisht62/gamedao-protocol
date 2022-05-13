#![cfg(test)]

pub use super::*;
use frame_support::{
	construct_runtime, parameter_types,
	traits::{Everything, GenesisBuild, Nothing},
};
use frame_support_test::TestRandomness;
use frame_system::EnsureRoot;
use sp_core::H256;
use sp_runtime::{traits::IdentityLookup, Permill};

use orml_traits::parameter_type_with_key;

// Types:
pub type AccountId = u32;
pub type BlockNumber = u64;
pub type Hash = H256;
pub type Moment = u64;
pub type Balance = u128;
pub type Amount = i128;
pub type CurrencyId = u32;
// pub type Header = generic::Header<BlockNumber, BlakeTwo256>;
type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;

// Constants:
pub const MILLICENTS: Balance = 1_000_000_000;
pub const CENTS: Balance = 1_000 * MILLICENTS;
pub const DOLLARS: Balance = 100 * CENTS;
pub const MILLISECS_PER_BLOCK: u64 = 6000;
pub const MINUTES: BlockNumber = 60_000 / (MILLISECS_PER_BLOCK as BlockNumber);
pub const HOURS: BlockNumber = MINUTES * 60;
pub const DAYS: BlockNumber = HOURS * 24;
pub const PROTOCOL_TOKEN_ID: CurrencyId = 1;
pub const PAYMENT_TOKEN_ID: CurrencyId = 2;

// Accounts:
pub const ALICE: AccountId = 1;
pub const BOB: AccountId = 2;
pub const BOGDANA: AccountId = 3;
pub const GAMEDAO_TREASURY: AccountId = 5;

mod gamedao_flow {
	pub use super::super::*;
}

parameter_types! {
	pub const BlockHashCount: u32 = 250;
}

impl frame_system::Config for Test {
	type Origin = Origin;
	type Index = u64;
	type BlockNumber = BlockNumber;
	type Call = Call;
	type Hash = H256;
	type Hashing = ::sp_runtime::traits::BlakeTwo256;
	type AccountId = AccountId;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = sp_runtime::testing::Header;
	type Event = Event;
	type BlockHashCount = BlockHashCount;
	type BlockWeights = ();
	type BlockLength = ();
	type Version = ();
	type PalletInfo = PalletInfo;
	type AccountData = pallet_balances::AccountData<Balance>;
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type DbWeight = ();
	type BaseCallFilter = Everything;
	type SystemWeightInfo = ();
	type SS58Prefix = ();
	type OnSetCode = ();
}

parameter_type_with_key! {
	pub ExistentialDeposits: |_currency_id: CurrencyId| -> Balance {
		Default::default()
	};
}

impl orml_tokens::Config for Test {
	type Event = Event;
	type Balance = Balance;
	type Amount = Amount;
	type CurrencyId = CurrencyId;
	type WeightInfo = ();
	type ExistentialDeposits = ExistentialDeposits;
	type OnDust = ();
	type MaxLocks = ();
	type DustRemovalWhitelist = Nothing;
}

parameter_types! {
	pub const ExistentialDeposit: Balance = 1;
}

impl pallet_balances::Config for Test {
	type Balance = Balance;
	type DustRemoval = ();
	type Event = Event;
	type ExistentialDeposit = ExistentialDeposit;
	type AccountStore = frame_system::Pallet<Test>;
	type MaxLocks = ();
	type MaxReserves = ();
	type ReserveIdentifier = [u8; 8];
	type WeightInfo = ();
}
pub type AdaptedBasicCurrency = orml_currencies::BasicCurrencyAdapter<Test, PalletBalances, Amount, BlockNumber>;

impl orml_currencies::Config for Test {
	type Event = Event;
	type MultiCurrency = Tokens;
	type NativeCurrency = AdaptedBasicCurrency;
	type GetNativeCurrencyId = ();
	type WeightInfo = ();
}

parameter_types! {
	pub const MinimumPeriod: Moment = 1000;
}

impl pallet_timestamp::Config for Test {
	type Moment = Moment;
	type OnTimestampSet = ();
	type MinimumPeriod = MinimumPeriod;
	type WeightInfo = ();
}

frame_support::parameter_types! {
	pub const MaxDAOsPerAccount: u32 = 2;
	pub const MaxMembersPerDAO: u32 = 2;
	pub const MaxCreationsPerBlock: u32 = 2;
	pub const ProtocolTokenId: u32 = PROTOCOL_TOKEN_ID;
	pub const PaymentTokenId: CurrencyId = PAYMENT_TOKEN_ID;
	pub const InitialDeposit: Balance = 1 * DOLLARS;
}

impl gamedao_control::Config for Test {
	type Balance = Balance;
	// type Moment = Moment;
	type CurrencyId = CurrencyId;
	type WeightInfo = ();
	type Event = Event;
	type Currency = Currencies;
	type Randomness = TestRandomness<Self>;
	type ForceOrigin = frame_system::EnsureRoot<Self::AccountId>;
	type MaxDAOsPerAccount = MaxDAOsPerAccount;
	type MaxMembersPerDAO = MaxMembersPerDAO;
	type MaxCreationsPerBlock = MaxCreationsPerBlock;
	type ProtocolTokenId = ProtocolTokenId;
	type PaymentTokenId = ProtocolTokenId;
	type InitialDeposit = InitialDeposit;
}

parameter_types! {
	pub const MinNameLength: u32 = 2;
	pub const MaxNameLength: u32 = 4;
	pub const MaxCampaignsPerAddress: u32 = 3;
	pub const MaxCampaignsPerBlock: u32 = 1;
	pub const MaxContributionsPerBlock: u32 = 3;
	pub const MinCampaignDuration: BlockNumber = 1 * DAYS;
	pub const MaxCampaignDuration: BlockNumber = 100 * DAYS;
	pub const MinCreatorDeposit: Balance = 1 * DOLLARS;
	pub const MinContribution: Balance = 1 * DOLLARS;
	pub CampaignFee: Permill = Permill::from_rational(1u32, 10u32); // 10%
	// pub const CampaignFee: Balance = 25 * CENTS;
	pub const GameDAOTreasury: AccountId = GAMEDAO_TREASURY;
}

impl Config for Test {
	type Balance = Balance;
	// type Moment = Moment;
	type CurrencyId = CurrencyId;
	type WeightInfo = ();
	type Event = Event;
	type Currency = Currencies;
	type ProtocolTokenId = ProtocolTokenId;
	type PaymentTokenId = PaymentTokenId;
	type UnixTime = PalletTimestamp;
	type Randomness = TestRandomness<Self>;
	type Control = Control;
	type GameDAOAdminOrigin = EnsureRoot<Self::AccountId>;
	type GameDAOTreasury = GameDAOTreasury;
	type MinNameLength = MinNameLength;
	type MaxNameLength = MaxNameLength;
	type MaxCampaignsPerAddress = MaxCampaignsPerAddress;
	type MaxCampaignsPerBlock = MaxCampaignsPerBlock;
	type MaxContributionsPerBlock = MaxContributionsPerBlock;
	type MinCampaignDuration = MinCampaignDuration;
	type MaxCampaignDuration = MaxCampaignDuration;
	type MinCreatorDeposit = MinCreatorDeposit;
	type MinContribution = MinContribution;
	type CampaignFee = CampaignFee;
}

construct_runtime!(
	pub enum Test where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
	{
		System: frame_system::{Pallet, Call, Storage, Config, Event<T>},
		Currencies: orml_currencies::{Pallet, Call, Event<T>},
		Tokens: orml_tokens::{Pallet, Storage, Event<T>, Config<T>},
		PalletBalances: pallet_balances::{Pallet, Call, Storage, Event<T>},
		PalletTimestamp: pallet_timestamp::{Pallet, Call, Storage, Inherent},
		Flow: gamedao_flow,
		Control: gamedao_control,
	}
);

pub fn new_test_ext() -> sp_io::TestExternalities {
	let mut t = frame_system::GenesisConfig::default().build_storage::<Test>().unwrap();
	orml_tokens::GenesisConfig::<Test> {
		balances: vec![
			(ALICE, PROTOCOL_TOKEN_ID, 100 * DOLLARS),
			(ALICE, PAYMENT_TOKEN_ID, 100 * DOLLARS),
			(BOB, PROTOCOL_TOKEN_ID, 100 * DOLLARS),
			(BOB, PAYMENT_TOKEN_ID, 100 * DOLLARS),
			(BOGDANA, PROTOCOL_TOKEN_ID, 100 * DOLLARS),
			(BOGDANA, PAYMENT_TOKEN_ID, 100 * DOLLARS),
			(GAMEDAO_TREASURY, PROTOCOL_TOKEN_ID, 0),
			(GAMEDAO_TREASURY, PAYMENT_TOKEN_ID, 0),
		],
	}
	.assimilate_storage(&mut t)
	.unwrap();
	t.into()
}