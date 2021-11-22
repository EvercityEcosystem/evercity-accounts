# 1. Evercity Accounts Pallet

This repositary contains source code of blockchain node, which is a main part of Evercity's Accounts.

# 2. Introduction

# 3. Overview

# 4. Evercity carbon credits project main entities

Accounts pallet has one main entity: 

### 4.1 AccountStruct 

Is the main entity for accounts, containing rolemask of account in pallet storage

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