use frame_support::sp_runtime::{
    testing::Header,
    traits::{BlakeTwo256, IdentityLookup},
};
use sp_core::H256;
use crate as pallet_evercity_accounts;
use crate::accounts::*;
use frame_support::parameter_types;


type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<TestRuntime>;
type Block = frame_system::mocking::MockBlock<TestRuntime>;

// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
	pub enum TestRuntime where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
	{
		System: frame_system::{Module, Call, Config, Storage, Event<T>},
        Balances: pallet_balances::{Module, Call, Storage, Config<T>, Event<T>},
		EvercityAccounts: pallet_evercity_accounts::{Module, Call, Storage},
	}
);

impl frame_system::Config for TestRuntime {
	type BaseCallFilter = ();
	type BlockWeights = ();
	type BlockLength = ();
	type DbWeight = ();
	type Origin = Origin;
	type Call = Call;
	type Index = u64;
	type BlockNumber = u64;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = u64;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = Header;
	type Event = ();
	type BlockHashCount = ();
	type Version = ();
	type PalletInfo = PalletInfo;
	type AccountData = pallet_balances::AccountData<u64>;
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
	type SS58Prefix = ();

}

impl pallet_evercity_accounts::Config for TestRuntime {
}

parameter_types! {
    pub const ExistentialDeposit: u64 = 0;
    pub const MaxLocks: u32 = 50;
}

impl pallet_balances::Config for TestRuntime {
    type Balance = u64;
    type Event = ();
    type DustRemoval = ();
    type ExistentialDeposit = ExistentialDeposit;
    type AccountStore = System;
    type WeightInfo = ();
    type MaxLocks = MaxLocks;
}

// (AccountId, role)
static ROLES: [(u64, RoleMask); 6] = [
    (1_u64, MASTER_ROLE_MASK),
    (2_u64, CC_PROJECT_OWNER_ROLE_MASK),
    (3_u64, CC_AUDITOR_ROLE_MASK),
    (4_u64, CC_STANDARD_ROLE_MASK),
    (5_u64, CC_INVESTOR_ROLE_MASK),
    (6_u64, CC_REGISTRY_ROLE_MASK),
];

//Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> frame_support::sp_io::TestExternalities {
    let mut t = frame_system::GenesisConfig::default()
        .build_storage::<TestRuntime>()
        .unwrap();
    pallet_balances::GenesisConfig::<TestRuntime> {
        // Provide some initial balances
        balances: ROLES.iter().map(|x| (x.0, 100000)).collect(),
    }
    .assimilate_storage(&mut t)
    .unwrap();

    super::GenesisConfig::<TestRuntime> {
        // Accounts for tests
        genesis_account_registry: ROLES
            .iter()
            .map(|(acc, role)| {
                (
                    *acc,
                    CarbonCreditAccountStruct {
                        roles: *role
                    },
                )
            })
            .collect(),
    }
    .assimilate_storage(&mut t)
    .unwrap();

    t.into()
}