use frame_support::{
    codec::{Decode, Encode},
    sp_runtime::RuntimeDebug,
};


pub type RoleMask = u64;

pub const MASTER_ROLE_MASK: RoleMask = 1u64;
pub const PROJECT_OWNER_ROLE_MASK: RoleMask = 2u64;
pub const AUDITOR_ROLE_MASK: RoleMask = 4u64;
pub const STANDARD_ROLE_MASK: RoleMask = 8u64;
pub const INVESTOR_ROLE_MASK: RoleMask = 16u64;
pub const REGISTRY_ROLE_MASK: RoleMask = 32u64;

pub const ALL_ROLES_MASK: RoleMask = MASTER_ROLE_MASK
    | PROJECT_OWNER_ROLE_MASK
    | AUDITOR_ROLE_MASK
    | STANDARD_ROLE_MASK
    | AUDITOR_ROLE_MASK
    | INVESTOR_ROLE_MASK
    | REGISTRY_ROLE_MASK;

#[inline]
pub const fn is_roles_correct(roles: RoleMask) -> bool {
    // max value of any roles combinations
    roles <= ALL_ROLES_MASK && roles > 0
}

#[derive(Encode, Decode, Clone, Default, RuntimeDebug)]
pub struct CarbonCreditAccountStruct {
    pub roles: RoleMask,
}

impl CarbonCreditAccountStruct {
    pub fn new(roles: RoleMask) -> Self {
        CarbonCreditAccountStruct{
            roles
        }
    }
}