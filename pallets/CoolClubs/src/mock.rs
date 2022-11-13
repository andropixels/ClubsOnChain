

use crate as pallet_clubs;

use frame_support::{traits::{ConstU16, ConstU64}, parameter_types, ord_parameter_types};

use sp_core::{H256, ConstU128};
use sp_runtime::{
	testing::Header,
	traits::{BlakeTwo256, IdentityLookup},
};
// use sp_core::{crypto::AccountId32};
use frame_support::{PalletId};
use frame_system::{EnsureRoot};
pub use crate::weights::SubstrateWeight;

pub type Balance = u128;
pub type BlockNumber = u64;
pub type AccountId = u64;
pub const UNITS: Balance = 100_000_000;
pub const ALICE: AccountId = 1;
pub const BOB: AccountId = 2;
pub const CHARLIE: AccountId = 3;

pub const MILLISECS_PER_BLOCK: u64 = 6000;
pub const SECS_PER_BLOCK: u64 = MILLISECS_PER_BLOCK / 1_000;


// Time is measured by number of blocks.
pub const MINUTES: BlockNumber = 60_000 / (MILLISECS_PER_BLOCK as BlockNumber);
pub const HOURS: BlockNumber = MINUTES * 60;
pub const DAYS: BlockNumber = HOURS * 24;

const SECS_PER_YEAR: u64 = 31_557_600; // (365.25 * 24 * 60 * 60)
pub const MONTHS: BlockNumber = (SECS_PER_YEAR / (12 * SECS_PER_BLOCK)) as BlockNumber;
pub const YEARS: BlockNumber = (SECS_PER_YEAR / SECS_PER_BLOCK) as BlockNumber;
pub const Hundread:BlockNumber = 100; 


type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;

frame_support::construct_runtime!(
	pub enum Test where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
	{
		System: frame_system,
		ClubsOnChain: pallet_clubs,
        Balances:pallet_balances,
	}
);

impl frame_system::Config for Test {
	type BaseCallFilter = frame_support::traits::Everything;
	type BlockWeights = ();
	type BlockLength = ();
	type DbWeight = ();
	type RuntimeOrigin = RuntimeOrigin;
	type RuntimeCall = RuntimeCall;
	type Index = u64;
	type BlockNumber = u64;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = u64;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = Header;
	type RuntimeEvent = RuntimeEvent;
	type BlockHashCount = ConstU64<250>;
	type Version = ();
	type PalletInfo = PalletInfo;
	type AccountData =  pallet_balances::AccountData<u128>;
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
	type SS58Prefix = ConstU16<42>;
	type OnSetCode = ();
	type MaxConsumers = frame_support::traits::ConstU32<16>;
}




impl pallet_balances::Config for Test {
	type MaxLocks = ();
	type MaxReserves = ();
	type ReserveIdentifier = [u8; 8];
	type Balance = u128;
	type RuntimeEvent = RuntimeEvent;
	type DustRemoval = ();
	type ExistentialDeposit = ConstU128<1>;
	type AccountStore = System;
	type WeightInfo = ();
}



parameter_types! {
    
	pub const FeesToCreateClub: Balance =1*UNITS ;
    pub const FeeCollector: PalletId = PalletId(*b"py/clubc");
    // max membership period is 100 years in terms of block number
    pub const MaximumMemberShipPeriod:BlockNumber = Hundread*YEARS;
    // unit Membership Period is one year in terms of blocknumnber
    pub const UnitMemberShipPeriod:BlockNumber = YEARS;
    // the Root 
    // pub const Root: u128 = 1;
}
ord_parameter_types! {
	pub const Root:u128 = 1;
	
}


impl pallet_clubs::Config for Test {

    type RuntimeEvent = RuntimeEvent;
    type Currency = Balances; 
    type FeesToCreateClub = FeesToCreateClub;
    type FeeCollector = FeeCollector;
    type MaxMembershipPeriod = MaximumMemberShipPeriod;
    type Root =EnsureRoot<AccountId>;
    type UnitMembeShipPeriod = UnitMemberShipPeriod;
	type WeightInfo =SubstrateWeight<Test>;

}



pub struct Extuilder ; 



impl Default for Extuilder {

	fn default() -> Self {
		Self
	}
}



impl Extuilder {

pub	fn build(self) -> sp_io::TestExternalities {

		let mut t = frame_system::GenesisConfig::default().build_storage::<Test>().unwrap(); 


	pallet_balances::GenesisConfig::<Test> {
		balances: vec![
			(ALICE, 100_000 * UNITS),
			(BOB, 200_000 * UNITS),
			(CHARLIE, 300_000 * UNITS),
		],
	}
	.assimilate_storage(&mut t)
	.unwrap();

		let mut  ext = sp_io::TestExternalities::new(t); 

		ext.execute_with(|| System::set_block_number(1));

		ext

	}
}


