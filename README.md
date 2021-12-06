# 1. Evercity Accounts Pallet

This repositary contains source code of blockchain node, which is a main part of Evercity's Accounts.

# 2. Introduction

# 3. Overview

# 4. Evercity carbon credits project main entities

Accounts pallet has several entities: 

### 4.1 AccountStruct 

Is the main entity for accounts, containing rolemask of account in pallet storage

### 4.2 RoleMask 

Each evercity account can can accommodate one or more roles:

- MASTER: the administrative role that can assign roles to accounts.

- CUSTODIAN: the role which can mint and burn the main platform token. This role is assigned to the public account of the partner bank, which exchanges USD --> EVERUSD and EVERUSD --> USD.
- ISSUER: the role which can create bonds. An account with the ISSUER role issues a bond to fund a sustainability-aligned project. After receiving funds from the sale of Bond Units, the ISSUER undertakes to provide data on the impact of the project, which influences the coupon rate that should be paid to the investor. The ISSUER is obliged to replenish the bond balance with the amount necessary to cover its financial obligations.
- INVESTOR: accounts with the INVESTOR role use the EVERUSD token to buy Bond Units and sell them on the secondary market. Each billing period Investor receives a coupon income proportional to its balances of various Bond Units
- AUDITOR: these accounts check and confirm the environmental impact data sent by Issuer, as well as certify the documents uploaded to the platform
- MANAGER: the task of accounts with this role is to help Issuers work with projects, verify data and prepare documents
- IMPACT_REPORTER: TBA
- EMISSION_CREATOR: TBA

- CC_PROJECT_OWNER: the role which can create carbon projects, annual report and issue caebon credits
- CC_AUDITOR: the role to sign project documentation and annual reports according to carbon credits standard
- CC_STANDARD: the role to sign project documentation and annual reports according to carbon credits standard
- CC_INVESTOR: carbon credits investor
- CC_REGISTRY: the role to sign project documentation and annual reports according to carbon credits standard


# 5. Evercity Account pallet can do several things

- Set master role on account

- Set any non-master role

- Add additional non-master role on account

- Withraw any non-master role

# 6. Accounts documentation

### 6.1 Runtime methods

<!-- Methods of pallet-evercity are described in Rust documentation [here](http://51.15.47.43/doc/pallet_evercity/) [TEMP] -->

### 6.2 Build

```bash
git clone https://github.com/EvercityEcosystem/evercity-accounts
cd evercity-accounts
make build
```
### 6.3 Add to runtime cargo.toml

```toml
pallet-evercity-accounts = { default-features = false, version = '0.1.7', git = 'https://github.com/EvercityEcosystem/evercity-accounts' }
#...
[features]
default = ['std']

std = [
    #...
    'pallet-evercity-accounts/std',
    #...
]
```

### 6.4 Add to runtime constructing

```rust
pub use pallet_evercity_accounts;
impl pallet_evercity_accounts::Config for Runtime {
    type Event = Event;
}
...
construct_runtime!(
    pub enum Runtime where
        Block = Block,
        NodeBlock = opaque::Block,
        UncheckedExtrinsic = UncheckedExtrinsic
    {
        ...
        EvercityAccounts: pallet_evercity_accounts::{ Module, Call, Storage, Config<T>, Event<T>},
        ...
    }
);
```

### 6.5 Check on smart sustainable bond node

```bash
git clone https://github.com/EvercityEcosystem/smart-sustainable-bond.git
cd smart-sustainable-bond
git checkout add_carbon_credits #temporary
make run
```

### 6.6 Run Unit Tests

```bash
make test
```

### 6.7 Launch linter

```bash
make lint
```