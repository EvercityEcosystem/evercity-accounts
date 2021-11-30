#![cfg_attr(not(feature = "std"), no_std)]

// #[cfg(feature = "runtime-benchmarks")]
// mod benchmarking;

#[cfg(test)]
pub mod mock;
#[cfg(test)]    
pub mod tests;  

pub mod accounts;

use sp_std::{fmt::Debug, prelude::*};
use codec::{Encode, Decode, HasCompact};
use frame_support::{
	ensure,
	traits::{Currency, ReservableCurrency, BalanceStatus::Reserved},
	dispatch::DispatchError,
};
// pub use weights::WeightInfo;

pub use pallet::*;
use accounts::*;


#[frame_support::pallet]
pub mod pallet {
	use frame_support::{
		dispatch::{DispatchResultWithPostInfo, DispatchResult},
		pallet_prelude::*,
	};
	use frame_system::pallet_prelude::*;
	use super::*;
	// use accounts::*;

    #[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

    #[pallet::config]
	/// The module configuration trait.
	pub trait Config: frame_system::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

	#[pallet::genesis_config]
    pub struct GenesisConfig<T: Config> {
        // pub core_asset_admin: AccountIdOf<T>,
        // pub core_asset_id: super::serializable::AssetId<T>,
        // pub balances: Vec<(AccountIdOf<T>, super::serializable::AssetBalance<T>)>,
		pub genesis_account_registry: Vec<(T::AccountId, AccountStruct)>
    }

	#[cfg(feature = "std")]
    impl<T: Config> Default for GenesisConfig<T> {
        fn default() -> Self {
            GenesisConfig {
                genesis_account_registry: Default::default(),
            }
        }
    }

	#[pallet::genesis_build]
    impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
        fn build(&self) {
            // let mut storage = Default::default();
            // let _ = self.assimilate_storage(&mut storage);
            // CoreAssetId::<T>::put(self.core_asset_id.0);

            // let admin_source = T::Lookup::unlookup(self.core_asset_admin.clone());
            // let call = pallet_assets::Call::<T>::create(
            //     self.core_asset_id.0,
            //     admin_source,
            //     u32::MAX,
            //     One::one(),
            // );
            // let result = call
            //     .dispatch_bypass_filter(RawOrigin::Signed(self.core_asset_admin.clone()).into());
            // assert!(result.is_ok());

            // // ensure no duplicates exist.
            // let endowed_accounts = self
            //     .balances
            //     .iter()
            //     .map(|(x, _)| x)
            //     .cloned()
            //     .collect::<std::collections::BTreeSet<_>>();

            // assert!(
            //     endowed_accounts.len() == self.balances.len(),
            //     "duplicate balances in genesis."
            // );

            // for (ref who, amount) in &self.balances {
            //     let who_source = <T::Lookup as StaticLookup>::unlookup(who.clone());
            //     let call =
            //         pallet_assets::Call::<T>::mint(self.core_asset_id.0, who_source, amount.0);
            //     let result = call.dispatch_bypass_filter(
            //         RawOrigin::Signed(self.core_asset_admin.clone()).into(),
            //     );
            //     assert!(result.is_ok());
            // }
        }
    }

	#[pallet::storage]
	/// Details of an account.
	pub type AccountRegistry<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		T::AccountId,
		AccountStruct,
		ValueQuery
	>;

	#[pallet::storage]
	/// Details of an account.
	pub type LastID<T: Config> = StorageValue<
		 _,
		u32,
		ValueQuery
	>;

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(10_000)]
		pub fn create(
			origin: OriginFor<T>
		) -> DispatchResultWithPostInfo {
			let _who = ensure_signed(origin)?;
			Ok(().into())
		}

		#[pallet::weight(10_000)]
		pub fn account_add_with_role_and_data(origin: OriginFor<T>, who: T::AccountId, role: RoleMask) -> DispatchResultWithPostInfo {
            let caller = ensure_signed(origin)?;
            ensure!(Self::account_is_master(&caller), Error::<T>::AccountNotMaster);
            ensure!(!AccountRegistry::<T>::contains_key(&who), Error::<T>::AccountToAddAlreadyExists);
            ensure!(is_roles_correct(role), Error::<T>::AccountRoleParamIncorrect);
            ensure!(!is_roles_mask_included(role, MASTER_ROLE_MASK), Error::<T>::AccountRoleMasterIncluded);
            AccountRegistry::<T>::insert(who.clone(), AccountStruct::new(role));
            Self::deposit_event(Event::AccountAdd(caller, who, role));
            Ok(().into())
        }

		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(2, 1))]
        pub fn account_set_with_role_and_data(origin: OriginFor<T>, who: T::AccountId, role: RoleMask) -> DispatchResultWithPostInfo {
            let caller = ensure_signed(origin)?;
            ensure!(caller != who, Error::<T>::InvalidAction);
            ensure!(Self::account_is_master(&caller), Error::<T>::AccountNotMaster);
            ensure!(AccountRegistry::<T>::contains_key(&who), Error::<T>::AccountNotExist);
            ensure!(is_roles_correct(role), Error::<T>::AccountRoleParamIncorrect);
            ensure!(!is_roles_mask_included(role, MASTER_ROLE_MASK), Error::<T>::AccountRoleMasterIncluded);
            AccountRegistry::<T>::mutate(who.clone(),|acc|{
                acc.roles |= role;
            });
            Self::deposit_event(Event::AccountSet(caller, who, role));
            Ok(().into())
        }

        #[pallet::weight(10_000 + T::DbWeight::get().reads_writes(2, 1))]
        pub fn set_master(origin: OriginFor<T>, who: T::AccountId) -> DispatchResultWithPostInfo {
            let caller = ensure_signed(origin)?;
            ensure!(caller != who, Error::<T>::InvalidAction);
            ensure!(Self::account_is_master(&caller), Error::<T>::AccountNotMaster);
            ensure!(!Self::account_is_master(&who), Error::<T>::InvalidAction);
            AccountRegistry::<T>::mutate(who.clone(),|acc|{
                acc.roles |= MASTER_ROLE_MASK;
            });
            Self::deposit_event(Event::MasterSet(caller, who));
            Ok(().into())
        }

        #[pallet::weight(10_000 + T::DbWeight::get().reads_writes(2, 1))]
        pub fn account_withdraw_role(origin: OriginFor<T>, who: T::AccountId, role: RoleMask) -> DispatchResultWithPostInfo {
            let caller = ensure_signed(origin)?;
            ensure!(caller != who, Error::<T>::InvalidAction);
            ensure!(Self::account_is_master(&caller), Error::<T>::AccountNotMaster);
            ensure!(AccountRegistry::<T>::contains_key(&who), Error::<T>::AccountNotExist);
            ensure!(is_roles_correct(role), Error::<T>::AccountRoleParamIncorrect);
            ensure!(!is_roles_mask_included(role, MASTER_ROLE_MASK), Error::<T>::AccountRoleMasterIncluded);
            AccountRegistry::<T>::mutate(who.clone(),|acc|{
                acc.roles ^= role;
            });
            Self::deposit_event(Event::AccountWithdraw(caller, who, role));
            Ok(().into())
        }
	}

    #[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	#[pallet::metadata(T::AccountId = "AccountId", T::Balance = "Balance", T::AssetId = "AssetId")]
	pub enum Event<T: Config> {
        /// \[master, account, role\]
        AccountAdd(T::AccountId, T::AccountId, RoleMask),

        /// \[master, account, role\]
        AccountSet(T::AccountId, T::AccountId, RoleMask),

        /// \[master, account, role\]
        AccountWithdraw(T::AccountId, T::AccountId, RoleMask),

        /// \[master, account\]
        MasterSet(T::AccountId, T::AccountId),
	}

	#[pallet::error]
	pub enum Error<T> {
        // Account errors:
        AccountNotMaster,
        AccountNotAuditor,
        AccountNotOwner,
        AccountNotStandard,
        AccountNotRegistry,
        AccountNotInvestor,
        AccountToAddAlreadyExists,
        AccountRoleParamIncorrect,
        AccountNotExist,
        AccountRoleMasterIncluded,

        InvalidAction,
	}	
}

impl<T: Config> Pallet<T> {
		    /// <pre>
    /// Method: account_is_master(acc: &T::AccountId) -> bool
    /// Arguments: acc: AccountId - checked account id
    ///
    /// Checks if the acc has global Master role
    /// </pre>
    #[inline]
    pub fn account_is_master(acc: &T::AccountId) -> bool {
        AccountRegistry::<T>::get(acc).roles & MASTER_ROLE_MASK != 0
    }

     /// <pre>
    /// Method: account_is_cc_project_owner(acc: &T::AccountId) -> bool
    /// Arguments: acc: AccountId - checked account id
    ///
    /// Checks if the acc has carbon credits project owner role
    /// </pre>
    #[inline]
    pub fn account_is_cc_project_owner(acc: &T::AccountId) -> bool {
        AccountRegistry::<T>::get(acc).roles & CC_PROJECT_OWNER_ROLE_MASK != 0
    }

    /// <pre>
    /// Method: account_is_cc_auditor(acc: &T::AccountId) -> bool
    /// Arguments: acc: AccountId - checked account id
    ///
    /// Checks if the acc hasc carbon credits auditor role
    /// </pre>
    #[inline]
    pub fn account_is_cc_auditor(acc: &T::AccountId) -> bool {
        AccountRegistry::<T>::get(acc).roles & CC_AUDITOR_ROLE_MASK != 0
    }

    /// <pre>
    /// Method: account_is_cc_standard(acc: &T::AccountId) -> bool
    /// Arguments: acc: AccountId - checked account id
    ///
    /// Checks if the acc has carbon credits standard role
    /// </pre>
    #[inline]
    pub fn account_is_cc_standard(acc: &T::AccountId) -> bool {
        AccountRegistry::<T>::get(acc).roles & CC_STANDARD_ROLE_MASK != 0
    }

    /// <pre>
    /// Method: account_is_cc_investor(acc: &T::AccountId) -> bool
    /// Arguments: acc: AccountId - checked account id
    ///
    /// Checks if the acc has carbon credits investor role
    /// </pre>
    #[inline]
    pub fn account_is_cc_investor(acc: &T::AccountId) -> bool {
        AccountRegistry::<T>::get(acc).roles & CC_INVESTOR_ROLE_MASK != 0
    }

    /// <pre>
    /// Method: account_is_cc_registry(acc: &T::AccountId) -> bool
    /// Arguments: acc: AccountId - checked account id
    ///
    /// Checks if the acc has carbon credits registry role
    /// </pre>
    #[inline]
    pub fn account_is_cc_registry(acc: &T::AccountId) -> bool {
        AccountRegistry::<T>::get(acc).roles & CC_REGISTRY_ROLE_MASK != 0
    }

    /// <pre>
    /// Method: accoount_is_selected_role(acc: &T::AccountId, role: RoleMask) -> bool
    /// Arguments: acc: AccountId - checked account id
    ///
    /// Checks if the acc has some custom role
    /// </pre>
    #[inline]
    pub fn account_is_selected_role(acc: &T::AccountId, role: RoleMask) -> bool {
        AccountRegistry::<T>::get(acc).roles & role != 0
    }
}

#[cfg(test)]
#[cfg(feature = "std")]
impl<T: Config> GenesisConfig<T> {
    /// Direct implementation of `GenesisBuild::build_storage`.
    ///
    /// Kept in order not to break dependency.
    pub fn build_storage(&self) -> Result<sp_runtime::Storage, String> {
        <Self as frame_support::traits::GenesisBuild<T>>::build_storage(self)
    }

    /// Direct implementation of `GenesisBuild::assimilate_storage`.
    ///
    /// Kept in order not to break dependency.
    pub fn assimilate_storage(&self, storage: &mut sp_runtime::Storage) -> Result<(), String> {
        <Self as frame_support::traits::GenesisBuild<T>>::assimilate_storage(self, storage)
    }
}