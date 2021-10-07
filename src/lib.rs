#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{
    ensure,
    decl_error, 
    decl_module, 
    decl_storage,
    dispatch::{
        DispatchResult, 
        DispatchError, 
        Vec,
    },
};
use frame_system::{
    ensure_signed,
};

// use frame_support::traits::IntegrityTest;

use frame_support::sp_std::{
    cmp::{
        Eq, 
        PartialEq}, 
};

pub mod accounts;
#[cfg(test)]
pub mod mock;
#[cfg(test)]    
pub mod tests;

use accounts::*;

pub trait Config: frame_system::Config {}

decl_storage! {
    trait Store for Module<T: Config> as CarbonCredits {
        Fuse get(fn fuse)
            build(|config| !config.genesis_account_registry.is_empty()):
            bool;

        /// Storage map for accounts, their roles and corresponding info
        AccountRegistry
            get(fn account_registry)
            config(genesis_account_registry):
            map hasher(blake2_128_concat) T::AccountId => CarbonCreditAccountStruct;

        LastID: u32;
    }
}

decl_error! {
    pub enum Error for Module<T: Config> {
        // Account errors:
        AccountNotMaster,
        AccountNotAuditor,
        AccountNotOwner,
        AccountNotStandard,
        AccountNotRegistry,
        AccountNotInvestor,
        AccountToAddAlreadyExists,
        AccountRoleParamIncorrect,
        InvalidAction,
        AccountNotExist,
    }
}

decl_module! {
    pub struct Module<T: Config> for enum Call where origin: T::Origin {
        #[weight = 10_000]
        fn account_add_with_role_and_data(origin, who: T::AccountId, role: RoleMask) -> DispatchResult {
            let caller = ensure_signed(origin)?;
            ensure!(Self::account_is_master(&caller), Error::<T>::AccountNotMaster);
            ensure!(!AccountRegistry::<T>::contains_key(&who), Error::<T>::AccountToAddAlreadyExists);
            ensure!(is_roles_correct(role), Error::<T>::AccountRoleParamIncorrect);
            Self::account_add(&who, CarbonCreditAccountStruct::new(role));
            Ok(())
        }

        #[weight = 10_000]
        fn account_set_with_role_and_data(origin, who: T::AccountId, role: RoleMask) -> DispatchResult {
            let caller = ensure_signed(origin)?;
            ensure!(caller != who, Error::<T>::InvalidAction);
            ensure!(Self::account_is_master(&caller), Error::<T>::AccountNotMaster);
            ensure!(AccountRegistry::<T>::contains_key(&who), Error::<T>::AccountNotExist);
            ensure!(is_roles_correct(role), Error::<T>::AccountRoleParamIncorrect);
            Self::account_set(&who, role);
            Ok(())
        }
    }
}

// Atomic operations here
impl<T: Config> Module<T> {
    fn account_set(who: &T::AccountId, role: RoleMask) {
        AccountRegistry::<T>::mutate(&who,|acc|{
            acc.roles |= role;
        });
    }

    fn account_add(account: &T::AccountId, data: CarbonCreditAccountStruct) {
        AccountRegistry::<T>::insert(account, &data);
    }

    /// <pre>
    /// Method: account_is_master(acc: &T::AccountId) -> bool
    /// Arguments: acc: AccountId - checked account id
    ///
    /// Checks if the acc has global Master role
    /// </pre>
    pub fn account_is_master(acc: &T::AccountId) -> bool {
        AccountRegistry::<T>::get(acc).roles & MASTER_ROLE_MASK != 0
    }

    pub fn account_is_project_owner(acc: &T::AccountId) -> bool {
        AccountRegistry::<T>::get(acc).roles & PROJECT_OWNER_ROLE_MASK != 0
    }

    pub fn account_is_auditor(acc: &T::AccountId) -> bool {
        AccountRegistry::<T>::get(acc).roles & AUDITOR_ROLE_MASK != 0
    }

    pub fn account_is_standard(acc: &T::AccountId) -> bool {
        AccountRegistry::<T>::get(acc).roles & STANDARD_ROLE_MASK != 0
    }

    pub fn account_is_investor(acc: &T::AccountId) -> bool {
        AccountRegistry::<T>::get(acc).roles & INVESTOR_ROLE_MASK != 0
    }

    pub fn account_is_registry(acc: &T::AccountId) -> bool {
        AccountRegistry::<T>::get(acc).roles & REGISTRY_ROLE_MASK != 0
    }
}
