#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2018::*;
#[macro_use]
extern crate std;
use crate::sp_api_hidden_includes_decl_storage::hidden_include::traits::Get;
use frame_support::{
    ensure, decl_error, decl_module, decl_storage, decl_event,
    dispatch::{DispatchResult, Vec},
};
use frame_system::{ensure_signed};
use frame_support::sp_std::{
    cmp::{Eq, PartialEq},
};
use accounts::*;
pub mod accounts {
    use frame_support::{
        codec::{Decode, Encode},
        sp_runtime::RuntimeDebug,
    };
    #[cfg(feature = "std")]
    use serde::{Deserialize, Serialize};
    pub type RoleMask = u32;
    pub const MASTER_ROLE_MASK: RoleMask = 1;
    pub const CUSTODIAN_ROLE_MASK: RoleMask = 2;
    pub const ISSUER_ROLE_MASK: RoleMask = 4;
    pub const INVESTOR_ROLE_MASK: RoleMask = 8;
    pub const AUDITOR_ROLE_MASK: RoleMask = 16;
    pub const MANAGER_ROLE_MASK: RoleMask = 32;
    pub const IMPACT_REPORTER_ROLE_MASK: RoleMask = 64;
    pub const EMISSION_CREATOR_ROLE_MASK: RoleMask = 128;
    pub const CC_PROJECT_OWNER_ROLE_MASK: RoleMask = 256;
    pub const CC_AUDITOR_ROLE_MASK: RoleMask = 512;
    pub const CC_STANDARD_ROLE_MASK: RoleMask = 1024;
    pub const CC_INVESTOR_ROLE_MASK: RoleMask = 2048;
    pub const CC_REGISTRY_ROLE_MASK: RoleMask = 4096;
    pub const ALL_ROLES_MASK: RoleMask = MASTER_ROLE_MASK
        | CUSTODIAN_ROLE_MASK
        | ISSUER_ROLE_MASK
        | INVESTOR_ROLE_MASK
        | AUDITOR_ROLE_MASK
        | MANAGER_ROLE_MASK
        | IMPACT_REPORTER_ROLE_MASK
        | EMISSION_CREATOR_ROLE_MASK
        | CC_PROJECT_OWNER_ROLE_MASK
        | CC_AUDITOR_ROLE_MASK
        | CC_STANDARD_ROLE_MASK
        | CC_INVESTOR_ROLE_MASK
        | CC_REGISTRY_ROLE_MASK;
    #[inline]
    pub const fn is_roles_correct(roles: RoleMask) -> bool {
        roles <= ALL_ROLES_MASK && roles > 0
    }
    #[inline]
    pub const fn is_roles_mask_included(roles: RoleMask, const_mask: RoleMask) -> bool {
        roles <= const_mask && roles > 0
    }
    pub struct AccountStruct {
        pub roles: RoleMask,
    }
    const _: () = {
        impl ::codec::Encode for AccountStruct {
            fn encode_to<__CodecOutputEdqy: ::codec::Output + ?::core::marker::Sized>(
                &self,
                __codec_dest_edqy: &mut __CodecOutputEdqy,
            ) {
                ::codec::Encode::encode_to(&&self.roles, __codec_dest_edqy)
            }
            fn encode(&self) -> ::codec::alloc::vec::Vec<::core::primitive::u8> {
                ::codec::Encode::encode(&&self.roles)
            }
            fn using_encoded<R, F: ::core::ops::FnOnce(&[::core::primitive::u8]) -> R>(
                &self,
                f: F,
            ) -> R {
                ::codec::Encode::using_encoded(&&self.roles, f)
            }
        }
        impl ::codec::EncodeLike for AccountStruct {}
    };
    const _: () = {
        impl ::codec::Decode for AccountStruct {
            fn decode<__CodecInputEdqy: ::codec::Input>(
                __codec_input_edqy: &mut __CodecInputEdqy,
            ) -> ::core::result::Result<Self, ::codec::Error> {
                ::core::result::Result::Ok(AccountStruct {
                    roles: {
                        let __codec_res_edqy =
                            <RoleMask as ::codec::Decode>::decode(__codec_input_edqy);
                        match __codec_res_edqy {
                            ::core::result::Result::Err(e) => {
                                return ::core::result::Result::Err(
                                    e.chain("Could not decode `AccountStruct::roles`"),
                                )
                            }
                            ::core::result::Result::Ok(__codec_res_edqy) => __codec_res_edqy,
                        }
                    },
                })
            }
        }
    };
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for AccountStruct {
        #[inline]
        fn clone(&self) -> AccountStruct {
            match *self {
                AccountStruct {
                    roles: ref __self_0_0,
                } => AccountStruct {
                    roles: ::core::clone::Clone::clone(&(*__self_0_0)),
                },
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::default::Default for AccountStruct {
        #[inline]
        fn default() -> AccountStruct {
            AccountStruct {
                roles: ::core::default::Default::default(),
            }
        }
    }
    impl core::fmt::Debug for AccountStruct {
        fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
            fmt.debug_struct("AccountStruct")
                .field("roles", &self.roles)
                .finish()
        }
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for AccountStruct {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = match _serde::Serializer::serialize_struct(
                    __serializer,
                    "AccountStruct",
                    false as usize + 1,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "roles",
                    &self.roles,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for AccountStruct {
            fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                enum __Field {
                    __field0,
                    __ignore,
                }
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(__formatter, "field identifier")
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::__private::Ok(__Field::__field0),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "roles" => _serde::__private::Ok(__Field::__field0),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"roles" => _serde::__private::Ok(__Field::__field0),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                    }
                }
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<AccountStruct>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = AccountStruct;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(__formatter, "struct AccountStruct")
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 =
                            match match _serde::de::SeqAccess::next_element::<RoleMask>(&mut __seq)
                            {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            0usize,
                                            &"struct AccountStruct with 1 element",
                                        ),
                                    );
                                }
                            };
                        _serde::__private::Ok(AccountStruct { roles: __field0 })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field0: _serde::__private::Option<RoleMask> =
                            _serde::__private::None;
                        while let _serde::__private::Some(__key) =
                            match _serde::de::MapAccess::next_key::<__Field>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            }
                        {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::__private::Option::is_some(&__field0) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "roles",
                                            ),
                                        );
                                    }
                                    __field0 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<RoleMask>(
                                            &mut __map,
                                        ) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                _ => {
                                    let _ = match _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map)
                                    {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    };
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::__private::Some(__field0) => __field0,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("roles") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        _serde::__private::Ok(AccountStruct { roles: __field0 })
                    }
                }
                const FIELDS: &'static [&'static str] = &["roles"];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "AccountStruct",
                    FIELDS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<AccountStruct>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    impl AccountStruct {
        pub fn new(roles: RoleMask) -> Self {
            AccountStruct { roles }
        }
    }
}
pub trait Config: frame_system::Config {
    type Event: From<Event<Self>> + Into<<Self as frame_system::Config>::Event>;
}
use self::sp_api_hidden_includes_decl_storage::hidden_include::{
    StorageValue as _, StorageMap as _, StorageDoubleMap as _, StoragePrefixedMap as _,
    IterableStorageMap as _, IterableStorageDoubleMap as _,
};
#[doc(hidden)]
mod sp_api_hidden_includes_decl_storage {
    pub extern crate frame_support as hidden_include;
}
trait Store {
    type Fuse;
    type AccountRegistry;
    type LastID;
}
impl<T: Config + 'static> Store for Module<T> {
    type Fuse = Fuse;
    type AccountRegistry = AccountRegistry<T>;
    type LastID = LastID;
}
impl<T: Config + 'static> Module<T> {
    pub fn fuse() -> bool {
        <Fuse as self::sp_api_hidden_includes_decl_storage::hidden_include::storage::StorageValue<
            bool,
        >>::get()
    }
    /// Storage map for accounts, their roles and corresponding info
    pub fn account_registry<
        K: self::sp_api_hidden_includes_decl_storage::hidden_include::codec::EncodeLike<T::AccountId>,
    >(
        key: K,
    ) -> AccountStruct {
        < AccountRegistry < T > as self :: sp_api_hidden_includes_decl_storage :: hidden_include :: storage :: StorageMap < T :: AccountId , AccountStruct > > :: get (key)
    }
}
#[doc(hidden)]
pub struct __GetByteStructFuse<T>(
    pub self::sp_api_hidden_includes_decl_storage::hidden_include::sp_std::marker::PhantomData<(T)>,
);
#[cfg(feature = "std")]
#[allow(non_upper_case_globals)]
static __CACHE_GET_BYTE_STRUCT_Fuse:
    self::sp_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell<
        self::sp_api_hidden_includes_decl_storage::hidden_include::sp_std::vec::Vec<u8>,
    > = self::sp_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell::new();
#[cfg(feature = "std")]
impl<T: Config> self::sp_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByte
    for __GetByteStructFuse<T>
{
    fn default_byte(
        &self,
    ) -> self::sp_api_hidden_includes_decl_storage::hidden_include::sp_std::vec::Vec<u8> {
        use self::sp_api_hidden_includes_decl_storage::hidden_include::codec::Encode;
        __CACHE_GET_BYTE_STRUCT_Fuse
            .get_or_init(|| {
                let def_val: bool = Default::default();
                <bool as Encode>::encode(&def_val)
            })
            .clone()
    }
}
unsafe impl<T: Config> Send for __GetByteStructFuse<T> {}
unsafe impl<T: Config> Sync for __GetByteStructFuse<T> {}
#[doc(hidden)]
pub struct __GetByteStructAccountRegistry<T>(
    pub self::sp_api_hidden_includes_decl_storage::hidden_include::sp_std::marker::PhantomData<(T)>,
);
#[cfg(feature = "std")]
#[allow(non_upper_case_globals)]
static __CACHE_GET_BYTE_STRUCT_AccountRegistry:
    self::sp_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell<
        self::sp_api_hidden_includes_decl_storage::hidden_include::sp_std::vec::Vec<u8>,
    > = self::sp_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell::new();
#[cfg(feature = "std")]
impl<T: Config> self::sp_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByte
    for __GetByteStructAccountRegistry<T>
{
    fn default_byte(
        &self,
    ) -> self::sp_api_hidden_includes_decl_storage::hidden_include::sp_std::vec::Vec<u8> {
        use self::sp_api_hidden_includes_decl_storage::hidden_include::codec::Encode;
        __CACHE_GET_BYTE_STRUCT_AccountRegistry
            .get_or_init(|| {
                let def_val: AccountStruct = Default::default();
                <AccountStruct as Encode>::encode(&def_val)
            })
            .clone()
    }
}
unsafe impl<T: Config> Send for __GetByteStructAccountRegistry<T> {}
unsafe impl<T: Config> Sync for __GetByteStructAccountRegistry<T> {}
#[doc(hidden)]
pub struct __GetByteStructLastID<T>(
    pub self::sp_api_hidden_includes_decl_storage::hidden_include::sp_std::marker::PhantomData<(T)>,
);
#[cfg(feature = "std")]
#[allow(non_upper_case_globals)]
static __CACHE_GET_BYTE_STRUCT_LastID:
    self::sp_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell<
        self::sp_api_hidden_includes_decl_storage::hidden_include::sp_std::vec::Vec<u8>,
    > = self::sp_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell::new();
#[cfg(feature = "std")]
impl<T: Config> self::sp_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByte
    for __GetByteStructLastID<T>
{
    fn default_byte(
        &self,
    ) -> self::sp_api_hidden_includes_decl_storage::hidden_include::sp_std::vec::Vec<u8> {
        use self::sp_api_hidden_includes_decl_storage::hidden_include::codec::Encode;
        __CACHE_GET_BYTE_STRUCT_LastID
            .get_or_init(|| {
                let def_val: u32 = Default::default();
                <u32 as Encode>::encode(&def_val)
            })
            .clone()
    }
}
unsafe impl<T: Config> Send for __GetByteStructLastID<T> {}
unsafe impl<T: Config> Sync for __GetByteStructLastID<T> {}
impl<T: Config + 'static> Module<T> {
    #[doc(hidden)]
    pub fn storage_metadata(
    ) -> self::sp_api_hidden_includes_decl_storage::hidden_include::metadata::StorageMetadata {
        self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageMetadata { prefix : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ("CarbonCredits") , entries : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode (& [self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageEntryMetadata { name : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ("Fuse") , modifier : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageEntryModifier :: Default , ty : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageEntryType :: Plain (self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ("bool")) , default : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode (self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DefaultByteGetter (& __GetByteStructFuse :: < T > (self :: sp_api_hidden_includes_decl_storage :: hidden_include :: sp_std :: marker :: PhantomData))) , documentation : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode (& []) , } , self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageEntryMetadata { name : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ("AccountRegistry") , modifier : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageEntryModifier :: Default , ty : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageEntryType :: Map { hasher : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageHasher :: Blake2_128Concat , key : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ("T::AccountId") , value : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ("AccountStruct") , unused : false , } , default : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode (self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DefaultByteGetter (& __GetByteStructAccountRegistry :: < T > (self :: sp_api_hidden_includes_decl_storage :: hidden_include :: sp_std :: marker :: PhantomData))) , documentation : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode (& [" Storage map for accounts, their roles and corresponding info"]) , } , self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageEntryMetadata { name : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ("LastID") , modifier : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageEntryModifier :: Default , ty : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageEntryType :: Plain (self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ("u32")) , default : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode (self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DefaultByteGetter (& __GetByteStructLastID :: < T > (self :: sp_api_hidden_includes_decl_storage :: hidden_include :: sp_std :: marker :: PhantomData))) , documentation : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode (& []) , }] [..]) , }
    }
}
/// Hidden instance generated to be internally used when module is used without
/// instance.
#[doc(hidden)]
pub struct __InherentHiddenInstance;
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::clone::Clone for __InherentHiddenInstance {
    #[inline]
    fn clone(&self) -> __InherentHiddenInstance {
        match *self {
            __InherentHiddenInstance => __InherentHiddenInstance,
        }
    }
}
impl ::core::marker::StructuralEq for __InherentHiddenInstance {}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::cmp::Eq for __InherentHiddenInstance {
    #[inline]
    #[doc(hidden)]
    #[no_coverage]
    fn assert_receiver_is_total_eq(&self) -> () {
        {}
    }
}
impl ::core::marker::StructuralPartialEq for __InherentHiddenInstance {}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::cmp::PartialEq for __InherentHiddenInstance {
    #[inline]
    fn eq(&self, other: &__InherentHiddenInstance) -> bool {
        match *other {
            __InherentHiddenInstance => match *self {
                __InherentHiddenInstance => true,
            },
        }
    }
}
const _: () = {
    impl ::codec::Encode for __InherentHiddenInstance {
        fn encode_to<__CodecOutputEdqy: ::codec::Output + ?::core::marker::Sized>(
            &self,
            __codec_dest_edqy: &mut __CodecOutputEdqy,
        ) {
        }
    }
    impl ::codec::EncodeLike for __InherentHiddenInstance {}
};
const _: () = {
    impl ::codec::Decode for __InherentHiddenInstance {
        fn decode<__CodecInputEdqy: ::codec::Input>(
            __codec_input_edqy: &mut __CodecInputEdqy,
        ) -> ::core::result::Result<Self, ::codec::Error> {
            ::core::result::Result::Ok(__InherentHiddenInstance)
        }
    }
};
impl core::fmt::Debug for __InherentHiddenInstance {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
        fmt.debug_tuple("__InherentHiddenInstance").finish()
    }
}
impl self::sp_api_hidden_includes_decl_storage::hidden_include::traits::Instance
    for __InherentHiddenInstance
{
    const PREFIX: &'static str = "CarbonCredits";
}
/// Genesis config for the module, allow to build genesis storage.
#[cfg(feature = "std")]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
#[serde(bound(
    serialize = "Vec < (T :: AccountId, AccountStruct) > : self :: sp_api_hidden_includes_decl_storage :: hidden_include::serde::Serialize, "
))]
#[serde(bound(
    deserialize = "Vec < (T :: AccountId, AccountStruct) > : self :: sp_api_hidden_includes_decl_storage :: hidden_include::serde::de::DeserializeOwned, "
))]
pub struct GenesisConfig<T: Config> {
    /// Storage map for accounts, their roles and corresponding info
    pub genesis_account_registry: Vec<(T::AccountId, AccountStruct)>,
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl<T: Config> _serde::Serialize for GenesisConfig<T>
    where
        Vec<(T::AccountId, AccountStruct)>:
            self::sp_api_hidden_includes_decl_storage::hidden_include::serde::Serialize,
    {
        fn serialize<__S>(
            &self,
            __serializer: __S,
        ) -> _serde::__private::Result<__S::Ok, __S::Error>
        where
            __S: _serde::Serializer,
        {
            let mut __serde_state = match _serde::Serializer::serialize_struct(
                __serializer,
                "GenesisConfig",
                false as usize + 1,
            ) {
                _serde::__private::Ok(__val) => __val,
                _serde::__private::Err(__err) => {
                    return _serde::__private::Err(__err);
                }
            };
            match _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "genesisAccountRegistry",
                &self.genesis_account_registry,
            ) {
                _serde::__private::Ok(__val) => __val,
                _serde::__private::Err(__err) => {
                    return _serde::__private::Err(__err);
                }
            };
            _serde::ser::SerializeStruct::end(__serde_state)
        }
    }
};
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl<'de, T: Config> _serde::Deserialize<'de> for GenesisConfig<T>
    where
        Vec<(T::AccountId, AccountStruct)>:
            self::sp_api_hidden_includes_decl_storage::hidden_include::serde::de::DeserializeOwned,
    {
        fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            enum __Field {
                __field0,
            }
            struct __FieldVisitor;
            impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                type Value = __Field;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(__formatter, "field identifier")
                }
                fn visit_u64<__E>(self, __value: u64) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        0u64 => _serde::__private::Ok(__Field::__field0),
                        _ => _serde::__private::Err(_serde::de::Error::invalid_value(
                            _serde::de::Unexpected::Unsigned(__value),
                            &"field index 0 <= i < 1",
                        )),
                    }
                }
                fn visit_str<__E>(
                    self,
                    __value: &str,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        "genesisAccountRegistry" => _serde::__private::Ok(__Field::__field0),
                        _ => _serde::__private::Err(_serde::de::Error::unknown_field(
                            __value, FIELDS,
                        )),
                    }
                }
                fn visit_bytes<__E>(
                    self,
                    __value: &[u8],
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        b"genesisAccountRegistry" => _serde::__private::Ok(__Field::__field0),
                        _ => {
                            let __value = &_serde::__private::from_utf8_lossy(__value);
                            _serde::__private::Err(_serde::de::Error::unknown_field(
                                __value, FIELDS,
                            ))
                        }
                    }
                }
            }
            impl<'de> _serde::Deserialize<'de> for __Field {
                #[inline]
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                }
            }
            struct __Visitor < 'de , T : Config > where Vec < (T :: AccountId , AccountStruct) > : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: serde :: de :: DeserializeOwned { marker : _serde :: __private :: PhantomData < GenesisConfig < T > > , lifetime : _serde :: __private :: PhantomData < & 'de () > , }
            impl < 'de , T : Config > _serde :: de :: Visitor < 'de > for __Visitor < 'de , T > where Vec < (T :: AccountId , AccountStruct) > : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: serde :: de :: DeserializeOwned { type Value = GenesisConfig < T > ; fn expecting (& self , __formatter : & mut _serde :: __private :: Formatter) -> _serde :: __private :: fmt :: Result { _serde :: __private :: Formatter :: write_str (__formatter , "struct GenesisConfig") } # [inline] fn visit_seq < __A > (self , mut __seq : __A) -> _serde :: __private :: Result < Self :: Value , __A :: Error > where __A : _serde :: de :: SeqAccess < 'de > { let __field0 = match match _serde :: de :: SeqAccess :: next_element :: < Vec < (T :: AccountId , AccountStruct) > > (& mut __seq) { _serde :: __private :: Ok (__val) => __val , _serde :: __private :: Err (__err) => { return _serde :: __private :: Err (__err) ; } } { _serde :: __private :: Some (__value) => __value , _serde :: __private :: None => { return _serde :: __private :: Err (_serde :: de :: Error :: invalid_length (0usize , & "struct GenesisConfig with 1 element")) ; } } ; _serde :: __private :: Ok (GenesisConfig { genesis_account_registry : __field0 , }) } # [inline] fn visit_map < __A > (self , mut __map : __A) -> _serde :: __private :: Result < Self :: Value , __A :: Error > where __A : _serde :: de :: MapAccess < 'de > { let mut __field0 : _serde :: __private :: Option < Vec < (T :: AccountId , AccountStruct) > > = _serde :: __private :: None ; while let _serde :: __private :: Some (__key) = match _serde :: de :: MapAccess :: next_key :: < __Field > (& mut __map) { _serde :: __private :: Ok (__val) => __val , _serde :: __private :: Err (__err) => { return _serde :: __private :: Err (__err) ; } } { match __key { __Field :: __field0 => { if _serde :: __private :: Option :: is_some (& __field0) { return _serde :: __private :: Err (< __A :: Error as _serde :: de :: Error > :: duplicate_field ("genesisAccountRegistry")) ; } __field0 = _serde :: __private :: Some (match _serde :: de :: MapAccess :: next_value :: < Vec < (T :: AccountId , AccountStruct) > > (& mut __map) { _serde :: __private :: Ok (__val) => __val , _serde :: __private :: Err (__err) => { return _serde :: __private :: Err (__err) ; } }) ; } } } let __field0 = match __field0 { _serde :: __private :: Some (__field0) => __field0 , _serde :: __private :: None => match _serde :: __private :: de :: missing_field ("genesisAccountRegistry") { _serde :: __private :: Ok (__val) => __val , _serde :: __private :: Err (__err) => { return _serde :: __private :: Err (__err) ; } } , } ; _serde :: __private :: Ok (GenesisConfig { genesis_account_registry : __field0 , }) } }
            const FIELDS: &'static [&'static str] = &["genesisAccountRegistry"];
            _serde::Deserializer::deserialize_struct(
                __deserializer,
                "GenesisConfig",
                FIELDS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<GenesisConfig<T>>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
#[cfg(feature = "std")]
impl<T: Config> Default for GenesisConfig<T> {
    fn default() -> Self {
        GenesisConfig {
            genesis_account_registry: Default::default(),
        }
    }
}
#[cfg(feature = "std")]
impl<T: Config> GenesisConfig<T> {
    /// Build the storage for this module.
    pub fn build_storage(
        &self,
    ) -> std::result::Result<
        self::sp_api_hidden_includes_decl_storage::hidden_include::sp_runtime::Storage,
        String,
    > {
        let mut storage = Default::default();
        self.assimilate_storage(&mut storage)?;
        Ok(storage)
    }
    /// Assimilate the storage for this module into pre-existing overlays.
    pub fn assimilate_storage(
        &self,
        storage : & mut self :: sp_api_hidden_includes_decl_storage :: hidden_include :: sp_runtime :: Storage,
    ) -> std::result::Result<(), String> {
        self :: sp_api_hidden_includes_decl_storage :: hidden_include :: BasicExternalities :: execute_with_storage (storage , | | { { let builder : fn (& Self) -> _ = | config | ! config . genesis_account_registry . is_empty () ; let data = & builder (self) ; let v : & bool = data ; < Fuse < > as self :: sp_api_hidden_includes_decl_storage :: hidden_include :: storage :: StorageValue < bool > > :: put :: < & bool > (v) ; } { let data = & self . genesis_account_registry ; let data : & self :: sp_api_hidden_includes_decl_storage :: hidden_include :: sp_std :: vec :: Vec < (T :: AccountId , AccountStruct) > = data ; data . iter () . for_each (| (k , v) | { < AccountRegistry < T > as self :: sp_api_hidden_includes_decl_storage :: hidden_include :: storage :: StorageMap < T :: AccountId , AccountStruct > > :: insert :: < & T :: AccountId , & AccountStruct > (k , v) ; }) ; } Ok (()) })
    }
}
#[cfg(feature = "std")]
impl<
        T: Config,
        __GeneratedInstance: self::sp_api_hidden_includes_decl_storage::hidden_include::traits::Instance,
    >
    self::sp_api_hidden_includes_decl_storage::hidden_include::sp_runtime::BuildModuleGenesisStorage<
        T,
        __GeneratedInstance,
    > for GenesisConfig<T>
{
    fn build_module_genesis_storage(
        &self,
        storage : & mut self :: sp_api_hidden_includes_decl_storage :: hidden_include :: sp_runtime :: Storage,
    ) -> std::result::Result<(), String> {
        self.assimilate_storage(storage)
    }
}
struct Fuse(
    self::sp_api_hidden_includes_decl_storage::hidden_include::sp_std::marker::PhantomData<()>,
);
impl
    self::sp_api_hidden_includes_decl_storage::hidden_include::storage::generator::StorageValue<
        bool,
    > for Fuse
{
    type Query = bool;
    fn module_prefix() -> &'static [u8] {
        < __InherentHiddenInstance as self :: sp_api_hidden_includes_decl_storage :: hidden_include :: traits :: Instance > :: PREFIX . as_bytes ()
    }
    fn storage_prefix() -> &'static [u8] {
        b"Fuse"
    }
    fn from_optional_value_to_query(v: Option<bool>) -> Self::Query {
        v.unwrap_or_else(|| Default::default())
    }
    fn from_query_to_optional_value(v: Self::Query) -> Option<bool> {
        Some(v)
    }
}
/// Storage map for accounts, their roles and corresponding info
struct AccountRegistry<T: Config>(
    self::sp_api_hidden_includes_decl_storage::hidden_include::sp_std::marker::PhantomData<(T,)>,
);
impl<T: Config>
    self::sp_api_hidden_includes_decl_storage::hidden_include::storage::StoragePrefixedMap<
        AccountStruct,
    > for AccountRegistry<T>
{
    fn module_prefix() -> &'static [u8] {
        < __InherentHiddenInstance as self :: sp_api_hidden_includes_decl_storage :: hidden_include :: traits :: Instance > :: PREFIX . as_bytes ()
    }
    fn storage_prefix() -> &'static [u8] {
        b"AccountRegistry"
    }
}
impl<T: Config>
    self::sp_api_hidden_includes_decl_storage::hidden_include::storage::generator::StorageMap<
        T::AccountId,
        AccountStruct,
    > for AccountRegistry<T>
{
    type Query = AccountStruct;
    type Hasher = self::sp_api_hidden_includes_decl_storage::hidden_include::Blake2_128Concat;
    fn module_prefix() -> &'static [u8] {
        < __InherentHiddenInstance as self :: sp_api_hidden_includes_decl_storage :: hidden_include :: traits :: Instance > :: PREFIX . as_bytes ()
    }
    fn storage_prefix() -> &'static [u8] {
        b"AccountRegistry"
    }
    fn from_optional_value_to_query(v: Option<AccountStruct>) -> Self::Query {
        v.unwrap_or_else(|| Default::default())
    }
    fn from_query_to_optional_value(v: Self::Query) -> Option<AccountStruct> {
        Some(v)
    }
}
struct LastID(
    self::sp_api_hidden_includes_decl_storage::hidden_include::sp_std::marker::PhantomData<()>,
);
impl
    self::sp_api_hidden_includes_decl_storage::hidden_include::storage::generator::StorageValue<u32>
    for LastID
{
    type Query = u32;
    fn module_prefix() -> &'static [u8] {
        < __InherentHiddenInstance as self :: sp_api_hidden_includes_decl_storage :: hidden_include :: traits :: Instance > :: PREFIX . as_bytes ()
    }
    fn storage_prefix() -> &'static [u8] {
        b"LastID"
    }
    fn from_optional_value_to_query(v: Option<u32>) -> Self::Query {
        v.unwrap_or_else(|| Default::default())
    }
    fn from_query_to_optional_value(v: Self::Query) -> Option<u32> {
        Some(v)
    }
}
/// [`RawEvent`] specialized for the configuration [`Config`]
///
/// [`RawEvent`]: enum.RawEvent.html
/// [`Config`]: trait.Config.html
pub type Event<T> = RawEvent<<T as frame_system::Config>::AccountId>;
/// Events for this module.
///
pub enum RawEvent<AccountId> {
    /// \[master, account, role\]
    AccountAdd(AccountId, AccountId, RoleMask),
    /// \[master, account, role\]
    AccountSet(AccountId, AccountId, RoleMask),
    /// \[master, account, role\]
    AccountWithdraw(AccountId, AccountId, RoleMask),
    /// \[master, account\]
    MasterSet(AccountId, AccountId),
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl<AccountId: ::core::clone::Clone> ::core::clone::Clone for RawEvent<AccountId> {
    #[inline]
    fn clone(&self) -> RawEvent<AccountId> {
        match (&*self,) {
            (&RawEvent::AccountAdd(ref __self_0, ref __self_1, ref __self_2),) => {
                RawEvent::AccountAdd(
                    ::core::clone::Clone::clone(&(*__self_0)),
                    ::core::clone::Clone::clone(&(*__self_1)),
                    ::core::clone::Clone::clone(&(*__self_2)),
                )
            }
            (&RawEvent::AccountSet(ref __self_0, ref __self_1, ref __self_2),) => {
                RawEvent::AccountSet(
                    ::core::clone::Clone::clone(&(*__self_0)),
                    ::core::clone::Clone::clone(&(*__self_1)),
                    ::core::clone::Clone::clone(&(*__self_2)),
                )
            }
            (&RawEvent::AccountWithdraw(ref __self_0, ref __self_1, ref __self_2),) => {
                RawEvent::AccountWithdraw(
                    ::core::clone::Clone::clone(&(*__self_0)),
                    ::core::clone::Clone::clone(&(*__self_1)),
                    ::core::clone::Clone::clone(&(*__self_2)),
                )
            }
            (&RawEvent::MasterSet(ref __self_0, ref __self_1),) => RawEvent::MasterSet(
                ::core::clone::Clone::clone(&(*__self_0)),
                ::core::clone::Clone::clone(&(*__self_1)),
            ),
        }
    }
}
impl<AccountId> ::core::marker::StructuralPartialEq for RawEvent<AccountId> {}
#[automatically_derived]
#[allow(unused_qualifications)]
impl<AccountId: ::core::cmp::PartialEq> ::core::cmp::PartialEq for RawEvent<AccountId> {
    #[inline]
    fn eq(&self, other: &RawEvent<AccountId>) -> bool {
        {
            let __self_vi = ::core::intrinsics::discriminant_value(&*self);
            let __arg_1_vi = ::core::intrinsics::discriminant_value(&*other);
            if true && __self_vi == __arg_1_vi {
                match (&*self, &*other) {
                    (
                        &RawEvent::AccountAdd(ref __self_0, ref __self_1, ref __self_2),
                        &RawEvent::AccountAdd(ref __arg_1_0, ref __arg_1_1, ref __arg_1_2),
                    ) => {
                        (*__self_0) == (*__arg_1_0)
                            && (*__self_1) == (*__arg_1_1)
                            && (*__self_2) == (*__arg_1_2)
                    }
                    (
                        &RawEvent::AccountSet(ref __self_0, ref __self_1, ref __self_2),
                        &RawEvent::AccountSet(ref __arg_1_0, ref __arg_1_1, ref __arg_1_2),
                    ) => {
                        (*__self_0) == (*__arg_1_0)
                            && (*__self_1) == (*__arg_1_1)
                            && (*__self_2) == (*__arg_1_2)
                    }
                    (
                        &RawEvent::AccountWithdraw(ref __self_0, ref __self_1, ref __self_2),
                        &RawEvent::AccountWithdraw(ref __arg_1_0, ref __arg_1_1, ref __arg_1_2),
                    ) => {
                        (*__self_0) == (*__arg_1_0)
                            && (*__self_1) == (*__arg_1_1)
                            && (*__self_2) == (*__arg_1_2)
                    }
                    (
                        &RawEvent::MasterSet(ref __self_0, ref __self_1),
                        &RawEvent::MasterSet(ref __arg_1_0, ref __arg_1_1),
                    ) => (*__self_0) == (*__arg_1_0) && (*__self_1) == (*__arg_1_1),
                    _ => unsafe { ::core::intrinsics::unreachable() },
                }
            } else {
                false
            }
        }
    }
    #[inline]
    fn ne(&self, other: &RawEvent<AccountId>) -> bool {
        {
            let __self_vi = ::core::intrinsics::discriminant_value(&*self);
            let __arg_1_vi = ::core::intrinsics::discriminant_value(&*other);
            if true && __self_vi == __arg_1_vi {
                match (&*self, &*other) {
                    (
                        &RawEvent::AccountAdd(ref __self_0, ref __self_1, ref __self_2),
                        &RawEvent::AccountAdd(ref __arg_1_0, ref __arg_1_1, ref __arg_1_2),
                    ) => {
                        (*__self_0) != (*__arg_1_0)
                            || (*__self_1) != (*__arg_1_1)
                            || (*__self_2) != (*__arg_1_2)
                    }
                    (
                        &RawEvent::AccountSet(ref __self_0, ref __self_1, ref __self_2),
                        &RawEvent::AccountSet(ref __arg_1_0, ref __arg_1_1, ref __arg_1_2),
                    ) => {
                        (*__self_0) != (*__arg_1_0)
                            || (*__self_1) != (*__arg_1_1)
                            || (*__self_2) != (*__arg_1_2)
                    }
                    (
                        &RawEvent::AccountWithdraw(ref __self_0, ref __self_1, ref __self_2),
                        &RawEvent::AccountWithdraw(ref __arg_1_0, ref __arg_1_1, ref __arg_1_2),
                    ) => {
                        (*__self_0) != (*__arg_1_0)
                            || (*__self_1) != (*__arg_1_1)
                            || (*__self_2) != (*__arg_1_2)
                    }
                    (
                        &RawEvent::MasterSet(ref __self_0, ref __self_1),
                        &RawEvent::MasterSet(ref __arg_1_0, ref __arg_1_1),
                    ) => (*__self_0) != (*__arg_1_0) || (*__self_1) != (*__arg_1_1),
                    _ => unsafe { ::core::intrinsics::unreachable() },
                }
            } else {
                true
            }
        }
    }
}
impl<AccountId> ::core::marker::StructuralEq for RawEvent<AccountId> {}
#[automatically_derived]
#[allow(unused_qualifications)]
impl<AccountId: ::core::cmp::Eq> ::core::cmp::Eq for RawEvent<AccountId> {
    #[inline]
    #[doc(hidden)]
    #[no_coverage]
    fn assert_receiver_is_total_eq(&self) -> () {
        {
            let _: ::core::cmp::AssertParamIsEq<AccountId>;
            let _: ::core::cmp::AssertParamIsEq<AccountId>;
            let _: ::core::cmp::AssertParamIsEq<RoleMask>;
            let _: ::core::cmp::AssertParamIsEq<AccountId>;
            let _: ::core::cmp::AssertParamIsEq<AccountId>;
            let _: ::core::cmp::AssertParamIsEq<RoleMask>;
            let _: ::core::cmp::AssertParamIsEq<AccountId>;
            let _: ::core::cmp::AssertParamIsEq<AccountId>;
            let _: ::core::cmp::AssertParamIsEq<RoleMask>;
            let _: ::core::cmp::AssertParamIsEq<AccountId>;
            let _: ::core::cmp::AssertParamIsEq<AccountId>;
        }
    }
}
const _: () = {
    impl<AccountId> ::codec::Encode for RawEvent<AccountId>
    where
        AccountId: ::codec::Encode,
        AccountId: ::codec::Encode,
        AccountId: ::codec::Encode,
        AccountId: ::codec::Encode,
        AccountId: ::codec::Encode,
        AccountId: ::codec::Encode,
        AccountId: ::codec::Encode,
        AccountId: ::codec::Encode,
        AccountId: ::codec::Encode,
        AccountId: ::codec::Encode,
        AccountId: ::codec::Encode,
        AccountId: ::codec::Encode,
        AccountId: ::codec::Encode,
        AccountId: ::codec::Encode,
        AccountId: ::codec::Encode,
        AccountId: ::codec::Encode,
    {
        fn encode_to<__CodecOutputEdqy: ::codec::Output + ?::core::marker::Sized>(
            &self,
            __codec_dest_edqy: &mut __CodecOutputEdqy,
        ) {
            match *self {
                RawEvent::AccountAdd(ref aa, ref ba, ref ca) => {
                    __codec_dest_edqy.push_byte(0usize as ::core::primitive::u8);
                    ::codec::Encode::encode_to(aa, __codec_dest_edqy);
                    ::codec::Encode::encode_to(ba, __codec_dest_edqy);
                    ::codec::Encode::encode_to(ca, __codec_dest_edqy);
                }
                RawEvent::AccountSet(ref aa, ref ba, ref ca) => {
                    __codec_dest_edqy.push_byte(1usize as ::core::primitive::u8);
                    ::codec::Encode::encode_to(aa, __codec_dest_edqy);
                    ::codec::Encode::encode_to(ba, __codec_dest_edqy);
                    ::codec::Encode::encode_to(ca, __codec_dest_edqy);
                }
                RawEvent::AccountWithdraw(ref aa, ref ba, ref ca) => {
                    __codec_dest_edqy.push_byte(2usize as ::core::primitive::u8);
                    ::codec::Encode::encode_to(aa, __codec_dest_edqy);
                    ::codec::Encode::encode_to(ba, __codec_dest_edqy);
                    ::codec::Encode::encode_to(ca, __codec_dest_edqy);
                }
                RawEvent::MasterSet(ref aa, ref ba) => {
                    __codec_dest_edqy.push_byte(3usize as ::core::primitive::u8);
                    ::codec::Encode::encode_to(aa, __codec_dest_edqy);
                    ::codec::Encode::encode_to(ba, __codec_dest_edqy);
                }
                _ => (),
            }
        }
    }
    impl<AccountId> ::codec::EncodeLike for RawEvent<AccountId>
    where
        AccountId: ::codec::Encode,
        AccountId: ::codec::Encode,
        AccountId: ::codec::Encode,
        AccountId: ::codec::Encode,
        AccountId: ::codec::Encode,
        AccountId: ::codec::Encode,
        AccountId: ::codec::Encode,
        AccountId: ::codec::Encode,
        AccountId: ::codec::Encode,
        AccountId: ::codec::Encode,
        AccountId: ::codec::Encode,
        AccountId: ::codec::Encode,
        AccountId: ::codec::Encode,
        AccountId: ::codec::Encode,
        AccountId: ::codec::Encode,
        AccountId: ::codec::Encode,
    {
    }
};
const _: () = {
    impl<AccountId> ::codec::Decode for RawEvent<AccountId>
    where
        AccountId: ::codec::Decode,
        AccountId: ::codec::Decode,
        AccountId: ::codec::Decode,
        AccountId: ::codec::Decode,
        AccountId: ::codec::Decode,
        AccountId: ::codec::Decode,
        AccountId: ::codec::Decode,
        AccountId: ::codec::Decode,
        AccountId: ::codec::Decode,
        AccountId: ::codec::Decode,
        AccountId: ::codec::Decode,
        AccountId: ::codec::Decode,
        AccountId: ::codec::Decode,
        AccountId: ::codec::Decode,
        AccountId: ::codec::Decode,
        AccountId: ::codec::Decode,
    {
        fn decode<__CodecInputEdqy: ::codec::Input>(
            __codec_input_edqy: &mut __CodecInputEdqy,
        ) -> ::core::result::Result<Self, ::codec::Error> {
            match __codec_input_edqy
                .read_byte()
                .map_err(|e| e.chain("Could not decode `RawEvent`, failed to read variant byte"))?
            {
                __codec_x_edqy if __codec_x_edqy == 0usize as ::core::primitive::u8 => {
                    ::core::result::Result::Ok(RawEvent::<AccountId>::AccountAdd(
                        {
                            let __codec_res_edqy =
                                <AccountId as ::codec::Decode>::decode(__codec_input_edqy);
                            match __codec_res_edqy {
                                ::core::result::Result::Err(e) => {
                                    return ::core::result::Result::Err(
                                        e.chain("Could not decode `RawEvent::AccountAdd.0`"),
                                    )
                                }
                                ::core::result::Result::Ok(__codec_res_edqy) => __codec_res_edqy,
                            }
                        },
                        {
                            let __codec_res_edqy =
                                <AccountId as ::codec::Decode>::decode(__codec_input_edqy);
                            match __codec_res_edqy {
                                ::core::result::Result::Err(e) => {
                                    return ::core::result::Result::Err(
                                        e.chain("Could not decode `RawEvent::AccountAdd.1`"),
                                    )
                                }
                                ::core::result::Result::Ok(__codec_res_edqy) => __codec_res_edqy,
                            }
                        },
                        {
                            let __codec_res_edqy =
                                <RoleMask as ::codec::Decode>::decode(__codec_input_edqy);
                            match __codec_res_edqy {
                                ::core::result::Result::Err(e) => {
                                    return ::core::result::Result::Err(
                                        e.chain("Could not decode `RawEvent::AccountAdd.2`"),
                                    )
                                }
                                ::core::result::Result::Ok(__codec_res_edqy) => __codec_res_edqy,
                            }
                        },
                    ))
                }
                __codec_x_edqy if __codec_x_edqy == 1usize as ::core::primitive::u8 => {
                    ::core::result::Result::Ok(RawEvent::<AccountId>::AccountSet(
                        {
                            let __codec_res_edqy =
                                <AccountId as ::codec::Decode>::decode(__codec_input_edqy);
                            match __codec_res_edqy {
                                ::core::result::Result::Err(e) => {
                                    return ::core::result::Result::Err(
                                        e.chain("Could not decode `RawEvent::AccountSet.0`"),
                                    )
                                }
                                ::core::result::Result::Ok(__codec_res_edqy) => __codec_res_edqy,
                            }
                        },
                        {
                            let __codec_res_edqy =
                                <AccountId as ::codec::Decode>::decode(__codec_input_edqy);
                            match __codec_res_edqy {
                                ::core::result::Result::Err(e) => {
                                    return ::core::result::Result::Err(
                                        e.chain("Could not decode `RawEvent::AccountSet.1`"),
                                    )
                                }
                                ::core::result::Result::Ok(__codec_res_edqy) => __codec_res_edqy,
                            }
                        },
                        {
                            let __codec_res_edqy =
                                <RoleMask as ::codec::Decode>::decode(__codec_input_edqy);
                            match __codec_res_edqy {
                                ::core::result::Result::Err(e) => {
                                    return ::core::result::Result::Err(
                                        e.chain("Could not decode `RawEvent::AccountSet.2`"),
                                    )
                                }
                                ::core::result::Result::Ok(__codec_res_edqy) => __codec_res_edqy,
                            }
                        },
                    ))
                }
                __codec_x_edqy if __codec_x_edqy == 2usize as ::core::primitive::u8 => {
                    ::core::result::Result::Ok(RawEvent::<AccountId>::AccountWithdraw(
                        {
                            let __codec_res_edqy =
                                <AccountId as ::codec::Decode>::decode(__codec_input_edqy);
                            match __codec_res_edqy {
                                ::core::result::Result::Err(e) => {
                                    return ::core::result::Result::Err(
                                        e.chain("Could not decode `RawEvent::AccountWithdraw.0`"),
                                    )
                                }
                                ::core::result::Result::Ok(__codec_res_edqy) => __codec_res_edqy,
                            }
                        },
                        {
                            let __codec_res_edqy =
                                <AccountId as ::codec::Decode>::decode(__codec_input_edqy);
                            match __codec_res_edqy {
                                ::core::result::Result::Err(e) => {
                                    return ::core::result::Result::Err(
                                        e.chain("Could not decode `RawEvent::AccountWithdraw.1`"),
                                    )
                                }
                                ::core::result::Result::Ok(__codec_res_edqy) => __codec_res_edqy,
                            }
                        },
                        {
                            let __codec_res_edqy =
                                <RoleMask as ::codec::Decode>::decode(__codec_input_edqy);
                            match __codec_res_edqy {
                                ::core::result::Result::Err(e) => {
                                    return ::core::result::Result::Err(
                                        e.chain("Could not decode `RawEvent::AccountWithdraw.2`"),
                                    )
                                }
                                ::core::result::Result::Ok(__codec_res_edqy) => __codec_res_edqy,
                            }
                        },
                    ))
                }
                __codec_x_edqy if __codec_x_edqy == 3usize as ::core::primitive::u8 => {
                    ::core::result::Result::Ok(RawEvent::<AccountId>::MasterSet(
                        {
                            let __codec_res_edqy =
                                <AccountId as ::codec::Decode>::decode(__codec_input_edqy);
                            match __codec_res_edqy {
                                ::core::result::Result::Err(e) => {
                                    return ::core::result::Result::Err(
                                        e.chain("Could not decode `RawEvent::MasterSet.0`"),
                                    )
                                }
                                ::core::result::Result::Ok(__codec_res_edqy) => __codec_res_edqy,
                            }
                        },
                        {
                            let __codec_res_edqy =
                                <AccountId as ::codec::Decode>::decode(__codec_input_edqy);
                            match __codec_res_edqy {
                                ::core::result::Result::Err(e) => {
                                    return ::core::result::Result::Err(
                                        e.chain("Could not decode `RawEvent::MasterSet.1`"),
                                    )
                                }
                                ::core::result::Result::Ok(__codec_res_edqy) => __codec_res_edqy,
                            }
                        },
                    ))
                }
                _ => ::core::result::Result::Err(<_ as ::core::convert::Into<_>>::into(
                    "Could not decode `RawEvent`, variant doesn\'t exist",
                )),
            }
        }
    }
};
impl<AccountId> core::fmt::Debug for RawEvent<AccountId>
where
    AccountId: core::fmt::Debug,
{
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::AccountAdd(ref a0, ref a1, ref a2) => fmt
                .debug_tuple("RawEvent::AccountAdd")
                .field(a0)
                .field(a1)
                .field(a2)
                .finish(),
            Self::AccountSet(ref a0, ref a1, ref a2) => fmt
                .debug_tuple("RawEvent::AccountSet")
                .field(a0)
                .field(a1)
                .field(a2)
                .finish(),
            Self::AccountWithdraw(ref a0, ref a1, ref a2) => fmt
                .debug_tuple("RawEvent::AccountWithdraw")
                .field(a0)
                .field(a1)
                .field(a2)
                .finish(),
            Self::MasterSet(ref a0, ref a1) => fmt
                .debug_tuple("RawEvent::MasterSet")
                .field(a0)
                .field(a1)
                .finish(),
            _ => Ok(()),
        }
    }
}
impl<AccountId> From<RawEvent<AccountId>> for () {
    fn from(_: RawEvent<AccountId>) -> () {
        ()
    }
}
impl<AccountId> RawEvent<AccountId> {
    #[allow(dead_code)]
    #[doc(hidden)]
    pub fn metadata() -> &'static [::frame_support::event::EventMetadata] {
        &[
            ::frame_support::event::EventMetadata {
                name: ::frame_support::event::DecodeDifferent::Encode("AccountAdd"),
                arguments: ::frame_support::event::DecodeDifferent::Encode(&[
                    "AccountId",
                    "AccountId",
                    "RoleMask",
                ]),
                documentation: ::frame_support::event::DecodeDifferent::Encode(&[
                    r" \[master, account, role\]",
                ]),
            },
            ::frame_support::event::EventMetadata {
                name: ::frame_support::event::DecodeDifferent::Encode("AccountSet"),
                arguments: ::frame_support::event::DecodeDifferent::Encode(&[
                    "AccountId",
                    "AccountId",
                    "RoleMask",
                ]),
                documentation: ::frame_support::event::DecodeDifferent::Encode(&[
                    r" \[master, account, role\]",
                ]),
            },
            ::frame_support::event::EventMetadata {
                name: ::frame_support::event::DecodeDifferent::Encode("AccountWithdraw"),
                arguments: ::frame_support::event::DecodeDifferent::Encode(&[
                    "AccountId",
                    "AccountId",
                    "RoleMask",
                ]),
                documentation: ::frame_support::event::DecodeDifferent::Encode(&[
                    r" \[master, account, role\]",
                ]),
            },
            ::frame_support::event::EventMetadata {
                name: ::frame_support::event::DecodeDifferent::Encode("MasterSet"),
                arguments: ::frame_support::event::DecodeDifferent::Encode(&[
                    "AccountId",
                    "AccountId",
                ]),
                documentation: ::frame_support::event::DecodeDifferent::Encode(&[
                    r" \[master, account\]",
                ]),
            },
        ]
    }
}
pub enum Error<T: Config> {
    #[doc(hidden)]
    __Ignore(
        ::frame_support::sp_std::marker::PhantomData<(T,)>,
        ::frame_support::Never,
    ),
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
impl<T: Config> ::frame_support::sp_std::fmt::Debug for Error<T> {
    fn fmt(
        &self,
        f: &mut ::frame_support::sp_std::fmt::Formatter<'_>,
    ) -> ::frame_support::sp_std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl<T: Config> Error<T> {
    fn as_u8(&self) -> u8 {
        match self {
            Error::__Ignore(_, _) => ::core::panicking::panic_fmt(::core::fmt::Arguments::new_v1(
                &["internal error: entered unreachable code: "],
                &match (&"`__Ignore` can never be constructed",) {
                    _args => [::core::fmt::ArgumentV1::new(
                        _args.0,
                        ::core::fmt::Display::fmt,
                    )],
                },
            )),
            Error::AccountNotMaster => 0,
            Error::AccountNotAuditor => 0 + 1,
            Error::AccountNotOwner => 0 + 1 + 1,
            Error::AccountNotStandard => 0 + 1 + 1 + 1,
            Error::AccountNotRegistry => 0 + 1 + 1 + 1 + 1,
            Error::AccountNotInvestor => 0 + 1 + 1 + 1 + 1 + 1,
            Error::AccountToAddAlreadyExists => 0 + 1 + 1 + 1 + 1 + 1 + 1,
            Error::AccountRoleParamIncorrect => 0 + 1 + 1 + 1 + 1 + 1 + 1 + 1,
            Error::AccountNotExist => 0 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1,
            Error::AccountRoleMasterIncluded => 0 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1,
            Error::InvalidAction => 0 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1,
        }
    }
    fn as_str(&self) -> &'static str {
        match self {
            Self::__Ignore(_, _) => ::core::panicking::panic_fmt(::core::fmt::Arguments::new_v1(
                &["internal error: entered unreachable code: "],
                &match (&"`__Ignore` can never be constructed",) {
                    _args => [::core::fmt::ArgumentV1::new(
                        _args.0,
                        ::core::fmt::Display::fmt,
                    )],
                },
            )),
            Error::AccountNotMaster => "AccountNotMaster",
            Error::AccountNotAuditor => "AccountNotAuditor",
            Error::AccountNotOwner => "AccountNotOwner",
            Error::AccountNotStandard => "AccountNotStandard",
            Error::AccountNotRegistry => "AccountNotRegistry",
            Error::AccountNotInvestor => "AccountNotInvestor",
            Error::AccountToAddAlreadyExists => "AccountToAddAlreadyExists",
            Error::AccountRoleParamIncorrect => "AccountRoleParamIncorrect",
            Error::AccountNotExist => "AccountNotExist",
            Error::AccountRoleMasterIncluded => "AccountRoleMasterIncluded",
            Error::InvalidAction => "InvalidAction",
        }
    }
}
impl<T: Config> From<Error<T>> for &'static str {
    fn from(err: Error<T>) -> &'static str {
        err.as_str()
    }
}
impl<T: Config> From<Error<T>> for ::frame_support::sp_runtime::DispatchError {
    fn from(err: Error<T>) -> Self {
        let index = <T::PalletInfo as ::frame_support::traits::PalletInfo>::index::<Module<T>>()
            .expect("Every active module has an index in the runtime; qed")
            as u8;
        ::frame_support::sp_runtime::DispatchError::Module {
            index,
            error: err.as_u8(),
            message: Some(err.as_str()),
        }
    }
}
impl<T: Config> ::frame_support::error::ModuleErrorMetadata for Error<T> {
    fn metadata() -> &'static [::frame_support::error::ErrorMetadata] {
        &[
            ::frame_support::error::ErrorMetadata {
                name: ::frame_support::error::DecodeDifferent::Encode("AccountNotMaster"),
                documentation: ::frame_support::error::DecodeDifferent::Encode(&[]),
            },
            ::frame_support::error::ErrorMetadata {
                name: ::frame_support::error::DecodeDifferent::Encode("AccountNotAuditor"),
                documentation: ::frame_support::error::DecodeDifferent::Encode(&[]),
            },
            ::frame_support::error::ErrorMetadata {
                name: ::frame_support::error::DecodeDifferent::Encode("AccountNotOwner"),
                documentation: ::frame_support::error::DecodeDifferent::Encode(&[]),
            },
            ::frame_support::error::ErrorMetadata {
                name: ::frame_support::error::DecodeDifferent::Encode("AccountNotStandard"),
                documentation: ::frame_support::error::DecodeDifferent::Encode(&[]),
            },
            ::frame_support::error::ErrorMetadata {
                name: ::frame_support::error::DecodeDifferent::Encode("AccountNotRegistry"),
                documentation: ::frame_support::error::DecodeDifferent::Encode(&[]),
            },
            ::frame_support::error::ErrorMetadata {
                name: ::frame_support::error::DecodeDifferent::Encode("AccountNotInvestor"),
                documentation: ::frame_support::error::DecodeDifferent::Encode(&[]),
            },
            ::frame_support::error::ErrorMetadata {
                name: ::frame_support::error::DecodeDifferent::Encode("AccountToAddAlreadyExists"),
                documentation: ::frame_support::error::DecodeDifferent::Encode(&[]),
            },
            ::frame_support::error::ErrorMetadata {
                name: ::frame_support::error::DecodeDifferent::Encode("AccountRoleParamIncorrect"),
                documentation: ::frame_support::error::DecodeDifferent::Encode(&[]),
            },
            ::frame_support::error::ErrorMetadata {
                name: ::frame_support::error::DecodeDifferent::Encode("AccountNotExist"),
                documentation: ::frame_support::error::DecodeDifferent::Encode(&[]),
            },
            ::frame_support::error::ErrorMetadata {
                name: ::frame_support::error::DecodeDifferent::Encode("AccountRoleMasterIncluded"),
                documentation: ::frame_support::error::DecodeDifferent::Encode(&[]),
            },
            ::frame_support::error::ErrorMetadata {
                name: ::frame_support::error::DecodeDifferent::Encode("InvalidAction"),
                documentation: ::frame_support::error::DecodeDifferent::Encode(&[]),
            },
        ]
    }
}
pub struct Module<T: Config>(::frame_support::sp_std::marker::PhantomData<(T,)>);
#[automatically_derived]
#[allow(unused_qualifications)]
impl<T: ::core::clone::Clone + Config> ::core::clone::Clone for Module<T> {
    #[inline]
    fn clone(&self) -> Module<T> {
        match *self {
            Module(ref __self_0_0) => Module(::core::clone::Clone::clone(&(*__self_0_0))),
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl<T: ::core::marker::Copy + Config> ::core::marker::Copy for Module<T> {}
impl<T: Config> ::core::marker::StructuralPartialEq for Module<T> {}
#[automatically_derived]
#[allow(unused_qualifications)]
impl<T: ::core::cmp::PartialEq + Config> ::core::cmp::PartialEq for Module<T> {
    #[inline]
    fn eq(&self, other: &Module<T>) -> bool {
        match *other {
            Module(ref __self_1_0) => match *self {
                Module(ref __self_0_0) => (*__self_0_0) == (*__self_1_0),
            },
        }
    }
    #[inline]
    fn ne(&self, other: &Module<T>) -> bool {
        match *other {
            Module(ref __self_1_0) => match *self {
                Module(ref __self_0_0) => (*__self_0_0) != (*__self_1_0),
            },
        }
    }
}
impl<T: Config> ::core::marker::StructuralEq for Module<T> {}
#[automatically_derived]
#[allow(unused_qualifications)]
impl<T: ::core::cmp::Eq + Config> ::core::cmp::Eq for Module<T> {
    #[inline]
    #[doc(hidden)]
    #[no_coverage]
    fn assert_receiver_is_total_eq(&self) -> () {
        {
            let _: ::core::cmp::AssertParamIsEq<
                ::frame_support::sp_std::marker::PhantomData<(T,)>,
            >;
        }
    }
}
impl<T: Config> core::fmt::Debug for Module<T>
where
    T: core::fmt::Debug,
{
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
        fmt.debug_tuple("Module").field(&self.0).finish()
    }
}
impl<T: frame_system::Config + Config>
    ::frame_support::traits::OnInitialize<<T as frame_system::Config>::BlockNumber> for Module<T>
{
}
impl<T: Config> ::frame_support::traits::OnRuntimeUpgrade for Module<T> {
    fn on_runtime_upgrade() -> ::frame_support::dispatch::Weight {
        let __within_span__ = {
            use ::tracing::__macro_support::Callsite as _;
            static CALLSITE: ::tracing::__macro_support::MacroCallsite = {
                use ::tracing::__macro_support::MacroCallsite;
                static META: ::tracing::Metadata<'static> = {
                    ::tracing_core::metadata::Metadata::new(
                        "on_runtime_upgrade",
                        "pallet_evercity_accounts",
                        ::tracing::Level::TRACE,
                        Some("src/lib.rs"),
                        Some(88u32),
                        Some("pallet_evercity_accounts"),
                        ::tracing_core::field::FieldSet::new(
                            &[],
                            ::tracing_core::callsite::Identifier(&CALLSITE),
                        ),
                        ::tracing::metadata::Kind::SPAN,
                    )
                };
                MacroCallsite::new(&META)
            };
            let mut interest = ::tracing::subscriber::Interest::never();
            if ::tracing::Level::TRACE <= ::tracing::level_filters::STATIC_MAX_LEVEL
                && ::tracing::Level::TRACE <= ::tracing::level_filters::LevelFilter::current()
                && {
                    interest = CALLSITE.interest();
                    !interest.is_never()
                }
                && CALLSITE.is_enabled(interest)
            {
                let meta = CALLSITE.metadata();
                ::tracing::Span::new(meta, &{ meta.fields().value_set(&[]) })
            } else {
                let span = CALLSITE.disabled_span();
                {};
                span
            }
        };
        let __tracing_guard__ = __within_span__.enter();
        frame_support::traits::PalletVersion {
            major: 0u16,
            minor: 1u8,
            patch: 7u8,
        }
        .put_into_storage::<<T as frame_system::Config>::PalletInfo, Self>();
        <<T as frame_system::Config>::DbWeight as ::frame_support::traits::Get<_>>::get().writes(1)
    }
}
impl<T: frame_system::Config + Config>
    ::frame_support::traits::OnFinalize<<T as frame_system::Config>::BlockNumber> for Module<T>
{
}
impl<T: frame_system::Config + Config>
    ::frame_support::traits::OffchainWorker<<T as frame_system::Config>::BlockNumber>
    for Module<T>
{
}
impl<T: Config> Module<T> {
    /// Deposits an event using `frame_system::Module::deposit_event`.
    fn deposit_event(event: impl Into<<T as Config>::Event>) {
        <frame_system::Module<T>>::deposit_event(event.into())
    }
}
#[cfg(feature = "std")]
impl<T: Config> ::frame_support::traits::IntegrityTest for Module<T> {}
/// Can also be called using [`Call`].
///
/// [`Call`]: enum.Call.html
impl<T: Config> Module<T> {
    ///
    /// NOTE: Calling this function will bypass origin filters.
    pub fn account_add_with_role_and_data(
        origin: T::Origin,
        who: T::AccountId,
        role: RoleMask,
    ) -> DispatchResult {
        let __within_span__ = {
            use ::tracing::__macro_support::Callsite as _;
            static CALLSITE: ::tracing::__macro_support::MacroCallsite = {
                use ::tracing::__macro_support::MacroCallsite;
                static META: ::tracing::Metadata<'static> = {
                    ::tracing_core::metadata::Metadata::new(
                        "account_add_with_role_and_data",
                        "pallet_evercity_accounts",
                        ::tracing::Level::TRACE,
                        Some("src/lib.rs"),
                        Some(88u32),
                        Some("pallet_evercity_accounts"),
                        ::tracing_core::field::FieldSet::new(
                            &[],
                            ::tracing_core::callsite::Identifier(&CALLSITE),
                        ),
                        ::tracing::metadata::Kind::SPAN,
                    )
                };
                MacroCallsite::new(&META)
            };
            let mut interest = ::tracing::subscriber::Interest::never();
            if ::tracing::Level::TRACE <= ::tracing::level_filters::STATIC_MAX_LEVEL
                && ::tracing::Level::TRACE <= ::tracing::level_filters::LevelFilter::current()
                && {
                    interest = CALLSITE.interest();
                    !interest.is_never()
                }
                && CALLSITE.is_enabled(interest)
            {
                let meta = CALLSITE.metadata();
                ::tracing::Span::new(meta, &{ meta.fields().value_set(&[]) })
            } else {
                let span = CALLSITE.disabled_span();
                {};
                span
            }
        };
        let __tracing_guard__ = __within_span__.enter();
        let caller = ensure_signed(origin)?;
        {
            if !Self::account_is_master(&caller) {
                {
                    return Err(Error::<T>::AccountNotMaster.into());
                };
            }
        };
        {
            if !!AccountRegistry::<T>::contains_key(&who) {
                {
                    return Err(Error::<T>::AccountToAddAlreadyExists.into());
                };
            }
        };
        {
            if !is_roles_correct(role) {
                {
                    return Err(Error::<T>::AccountRoleParamIncorrect.into());
                };
            }
        };
        {
            if !!is_roles_mask_included(role, MASTER_ROLE_MASK) {
                {
                    return Err(Error::<T>::AccountRoleMasterIncluded.into());
                };
            }
        };
        AccountRegistry::<T>::insert(who.clone(), AccountStruct::new(role));
        Self::deposit_event(RawEvent::AccountAdd(caller, who, role));
        Ok(())
    }
    ///
    /// NOTE: Calling this function will bypass origin filters.
    pub fn account_set_with_role_and_data(
        origin: T::Origin,
        who: T::AccountId,
        role: RoleMask,
    ) -> DispatchResult {
        let __within_span__ = {
            use ::tracing::__macro_support::Callsite as _;
            static CALLSITE: ::tracing::__macro_support::MacroCallsite = {
                use ::tracing::__macro_support::MacroCallsite;
                static META: ::tracing::Metadata<'static> = {
                    ::tracing_core::metadata::Metadata::new(
                        "account_set_with_role_and_data",
                        "pallet_evercity_accounts",
                        ::tracing::Level::TRACE,
                        Some("src/lib.rs"),
                        Some(88u32),
                        Some("pallet_evercity_accounts"),
                        ::tracing_core::field::FieldSet::new(
                            &[],
                            ::tracing_core::callsite::Identifier(&CALLSITE),
                        ),
                        ::tracing::metadata::Kind::SPAN,
                    )
                };
                MacroCallsite::new(&META)
            };
            let mut interest = ::tracing::subscriber::Interest::never();
            if ::tracing::Level::TRACE <= ::tracing::level_filters::STATIC_MAX_LEVEL
                && ::tracing::Level::TRACE <= ::tracing::level_filters::LevelFilter::current()
                && {
                    interest = CALLSITE.interest();
                    !interest.is_never()
                }
                && CALLSITE.is_enabled(interest)
            {
                let meta = CALLSITE.metadata();
                ::tracing::Span::new(meta, &{ meta.fields().value_set(&[]) })
            } else {
                let span = CALLSITE.disabled_span();
                {};
                span
            }
        };
        let __tracing_guard__ = __within_span__.enter();
        let caller = ensure_signed(origin)?;
        {
            if !(caller != who) {
                {
                    return Err(Error::<T>::InvalidAction.into());
                };
            }
        };
        {
            if !Self::account_is_master(&caller) {
                {
                    return Err(Error::<T>::AccountNotMaster.into());
                };
            }
        };
        {
            if !AccountRegistry::<T>::contains_key(&who) {
                {
                    return Err(Error::<T>::AccountNotExist.into());
                };
            }
        };
        {
            if !is_roles_correct(role) {
                {
                    return Err(Error::<T>::AccountRoleParamIncorrect.into());
                };
            }
        };
        {
            if !!is_roles_mask_included(role, MASTER_ROLE_MASK) {
                {
                    return Err(Error::<T>::AccountRoleMasterIncluded.into());
                };
            }
        };
        AccountRegistry::<T>::mutate(who.clone(), |acc| {
            acc.roles |= role;
        });
        Self::deposit_event(RawEvent::AccountSet(caller, who, role));
        Ok(())
    }
    ///
    /// NOTE: Calling this function will bypass origin filters.
    pub fn set_master(origin: T::Origin, who: T::AccountId) -> DispatchResult {
        let __within_span__ = {
            use ::tracing::__macro_support::Callsite as _;
            static CALLSITE: ::tracing::__macro_support::MacroCallsite = {
                use ::tracing::__macro_support::MacroCallsite;
                static META: ::tracing::Metadata<'static> = {
                    ::tracing_core::metadata::Metadata::new(
                        "set_master",
                        "pallet_evercity_accounts",
                        ::tracing::Level::TRACE,
                        Some("src/lib.rs"),
                        Some(88u32),
                        Some("pallet_evercity_accounts"),
                        ::tracing_core::field::FieldSet::new(
                            &[],
                            ::tracing_core::callsite::Identifier(&CALLSITE),
                        ),
                        ::tracing::metadata::Kind::SPAN,
                    )
                };
                MacroCallsite::new(&META)
            };
            let mut interest = ::tracing::subscriber::Interest::never();
            if ::tracing::Level::TRACE <= ::tracing::level_filters::STATIC_MAX_LEVEL
                && ::tracing::Level::TRACE <= ::tracing::level_filters::LevelFilter::current()
                && {
                    interest = CALLSITE.interest();
                    !interest.is_never()
                }
                && CALLSITE.is_enabled(interest)
            {
                let meta = CALLSITE.metadata();
                ::tracing::Span::new(meta, &{ meta.fields().value_set(&[]) })
            } else {
                let span = CALLSITE.disabled_span();
                {};
                span
            }
        };
        let __tracing_guard__ = __within_span__.enter();
        let caller = ensure_signed(origin)?;
        {
            if !(caller != who) {
                {
                    return Err(Error::<T>::InvalidAction.into());
                };
            }
        };
        {
            if !Self::account_is_master(&caller) {
                {
                    return Err(Error::<T>::AccountNotMaster.into());
                };
            }
        };
        {
            if !!Self::account_is_master(&who) {
                {
                    return Err(Error::<T>::InvalidAction.into());
                };
            }
        };
        AccountRegistry::<T>::mutate(who.clone(), |acc| {
            acc.roles |= MASTER_ROLE_MASK;
        });
        Self::deposit_event(RawEvent::MasterSet(caller, who));
        Ok(())
    }
    ///
    /// NOTE: Calling this function will bypass origin filters.
    pub fn account_withdraw_role(
        origin: T::Origin,
        who: T::AccountId,
        role: RoleMask,
    ) -> DispatchResult {
        let __within_span__ = {
            use ::tracing::__macro_support::Callsite as _;
            static CALLSITE: ::tracing::__macro_support::MacroCallsite = {
                use ::tracing::__macro_support::MacroCallsite;
                static META: ::tracing::Metadata<'static> = {
                    ::tracing_core::metadata::Metadata::new(
                        "account_withdraw_role",
                        "pallet_evercity_accounts",
                        ::tracing::Level::TRACE,
                        Some("src/lib.rs"),
                        Some(88u32),
                        Some("pallet_evercity_accounts"),
                        ::tracing_core::field::FieldSet::new(
                            &[],
                            ::tracing_core::callsite::Identifier(&CALLSITE),
                        ),
                        ::tracing::metadata::Kind::SPAN,
                    )
                };
                MacroCallsite::new(&META)
            };
            let mut interest = ::tracing::subscriber::Interest::never();
            if ::tracing::Level::TRACE <= ::tracing::level_filters::STATIC_MAX_LEVEL
                && ::tracing::Level::TRACE <= ::tracing::level_filters::LevelFilter::current()
                && {
                    interest = CALLSITE.interest();
                    !interest.is_never()
                }
                && CALLSITE.is_enabled(interest)
            {
                let meta = CALLSITE.metadata();
                ::tracing::Span::new(meta, &{ meta.fields().value_set(&[]) })
            } else {
                let span = CALLSITE.disabled_span();
                {};
                span
            }
        };
        let __tracing_guard__ = __within_span__.enter();
        let caller = ensure_signed(origin)?;
        {
            if !(caller != who) {
                {
                    return Err(Error::<T>::InvalidAction.into());
                };
            }
        };
        {
            if !Self::account_is_master(&caller) {
                {
                    return Err(Error::<T>::AccountNotMaster.into());
                };
            }
        };
        {
            if !AccountRegistry::<T>::contains_key(&who) {
                {
                    return Err(Error::<T>::AccountNotExist.into());
                };
            }
        };
        {
            if !is_roles_correct(role) {
                {
                    return Err(Error::<T>::AccountRoleParamIncorrect.into());
                };
            }
        };
        {
            if !!is_roles_mask_included(role, MASTER_ROLE_MASK) {
                {
                    return Err(Error::<T>::AccountRoleMasterIncluded.into());
                };
            }
        };
        AccountRegistry::<T>::mutate(who.clone(), |acc| {
            acc.roles ^= role;
        });
        Self::deposit_event(RawEvent::AccountWithdraw(caller, who, role));
        Ok(())
    }
}
/// Dispatchable calls.
///
/// Each variant of this enum maps to a dispatchable function from the associated module.
pub enum Call<T: Config> {
    #[doc(hidden)]
    #[codec(skip)]
    __PhantomItem(
        ::frame_support::sp_std::marker::PhantomData<(T,)>,
        ::frame_support::Never,
    ),
    #[allow(non_camel_case_types)]
    account_add_with_role_and_data(T::AccountId, RoleMask),
    #[allow(non_camel_case_types)]
    account_set_with_role_and_data(T::AccountId, RoleMask),
    #[allow(non_camel_case_types)]
    set_master(T::AccountId),
    #[allow(non_camel_case_types)]
    account_withdraw_role(T::AccountId, RoleMask),
}
const _: () = {
    impl<T: Config> ::codec::Encode for Call<T>
    where
        T::AccountId: ::codec::Encode,
        T::AccountId: ::codec::Encode,
        T::AccountId: ::codec::Encode,
        T::AccountId: ::codec::Encode,
        T::AccountId: ::codec::Encode,
        T::AccountId: ::codec::Encode,
        T::AccountId: ::codec::Encode,
        T::AccountId: ::codec::Encode,
    {
        fn encode_to<__CodecOutputEdqy: ::codec::Output + ?::core::marker::Sized>(
            &self,
            __codec_dest_edqy: &mut __CodecOutputEdqy,
        ) {
            match *self {
                Call::account_add_with_role_and_data(ref aa, ref ba) => {
                    __codec_dest_edqy.push_byte(0usize as ::core::primitive::u8);
                    ::codec::Encode::encode_to(aa, __codec_dest_edqy);
                    ::codec::Encode::encode_to(ba, __codec_dest_edqy);
                }
                Call::account_set_with_role_and_data(ref aa, ref ba) => {
                    __codec_dest_edqy.push_byte(1usize as ::core::primitive::u8);
                    ::codec::Encode::encode_to(aa, __codec_dest_edqy);
                    ::codec::Encode::encode_to(ba, __codec_dest_edqy);
                }
                Call::set_master(ref aa) => {
                    __codec_dest_edqy.push_byte(2usize as ::core::primitive::u8);
                    ::codec::Encode::encode_to(aa, __codec_dest_edqy);
                }
                Call::account_withdraw_role(ref aa, ref ba) => {
                    __codec_dest_edqy.push_byte(3usize as ::core::primitive::u8);
                    ::codec::Encode::encode_to(aa, __codec_dest_edqy);
                    ::codec::Encode::encode_to(ba, __codec_dest_edqy);
                }
                _ => (),
            }
        }
    }
    impl<T: Config> ::codec::EncodeLike for Call<T>
    where
        T::AccountId: ::codec::Encode,
        T::AccountId: ::codec::Encode,
        T::AccountId: ::codec::Encode,
        T::AccountId: ::codec::Encode,
        T::AccountId: ::codec::Encode,
        T::AccountId: ::codec::Encode,
        T::AccountId: ::codec::Encode,
        T::AccountId: ::codec::Encode,
    {
    }
};
const _: () = {
    impl<T: Config> ::codec::Decode for Call<T>
    where
        T::AccountId: ::codec::Decode,
        T::AccountId: ::codec::Decode,
        T::AccountId: ::codec::Decode,
        T::AccountId: ::codec::Decode,
        T::AccountId: ::codec::Decode,
        T::AccountId: ::codec::Decode,
        T::AccountId: ::codec::Decode,
        T::AccountId: ::codec::Decode,
    {
        fn decode<__CodecInputEdqy: ::codec::Input>(
            __codec_input_edqy: &mut __CodecInputEdqy,
        ) -> ::core::result::Result<Self, ::codec::Error> {
            match __codec_input_edqy
                .read_byte()
                .map_err(|e| e.chain("Could not decode `Call`, failed to read variant byte"))?
            {
                __codec_x_edqy if __codec_x_edqy == 0usize as ::core::primitive::u8 => {
                    ::core::result::Result::Ok(Call::<T>::account_add_with_role_and_data(
                        {
                            let __codec_res_edqy =
                                <T::AccountId as ::codec::Decode>::decode(__codec_input_edqy);
                            match __codec_res_edqy {
                                ::core::result::Result::Err(e) => {
                                    return ::core::result::Result::Err(e.chain(
                                        "Could not decode `Call::account_add_with_role_and_data.0`",
                                    ))
                                }
                                ::core::result::Result::Ok(__codec_res_edqy) => __codec_res_edqy,
                            }
                        },
                        {
                            let __codec_res_edqy =
                                <RoleMask as ::codec::Decode>::decode(__codec_input_edqy);
                            match __codec_res_edqy {
                                ::core::result::Result::Err(e) => {
                                    return ::core::result::Result::Err(e.chain(
                                        "Could not decode `Call::account_add_with_role_and_data.1`",
                                    ))
                                }
                                ::core::result::Result::Ok(__codec_res_edqy) => __codec_res_edqy,
                            }
                        },
                    ))
                }
                __codec_x_edqy if __codec_x_edqy == 1usize as ::core::primitive::u8 => {
                    ::core::result::Result::Ok(Call::<T>::account_set_with_role_and_data(
                        {
                            let __codec_res_edqy =
                                <T::AccountId as ::codec::Decode>::decode(__codec_input_edqy);
                            match __codec_res_edqy {
                                ::core::result::Result::Err(e) => {
                                    return ::core::result::Result::Err(e.chain(
                                        "Could not decode `Call::account_set_with_role_and_data.0`",
                                    ))
                                }
                                ::core::result::Result::Ok(__codec_res_edqy) => __codec_res_edqy,
                            }
                        },
                        {
                            let __codec_res_edqy =
                                <RoleMask as ::codec::Decode>::decode(__codec_input_edqy);
                            match __codec_res_edqy {
                                ::core::result::Result::Err(e) => {
                                    return ::core::result::Result::Err(e.chain(
                                        "Could not decode `Call::account_set_with_role_and_data.1`",
                                    ))
                                }
                                ::core::result::Result::Ok(__codec_res_edqy) => __codec_res_edqy,
                            }
                        },
                    ))
                }
                __codec_x_edqy if __codec_x_edqy == 2usize as ::core::primitive::u8 => {
                    ::core::result::Result::Ok(Call::<T>::set_master({
                        let __codec_res_edqy =
                            <T::AccountId as ::codec::Decode>::decode(__codec_input_edqy);
                        match __codec_res_edqy {
                            ::core::result::Result::Err(e) => {
                                return ::core::result::Result::Err(
                                    e.chain("Could not decode `Call::set_master.0`"),
                                )
                            }
                            ::core::result::Result::Ok(__codec_res_edqy) => __codec_res_edqy,
                        }
                    }))
                }
                __codec_x_edqy if __codec_x_edqy == 3usize as ::core::primitive::u8 => {
                    ::core::result::Result::Ok(Call::<T>::account_withdraw_role(
                        {
                            let __codec_res_edqy =
                                <T::AccountId as ::codec::Decode>::decode(__codec_input_edqy);
                            match __codec_res_edqy {
                                ::core::result::Result::Err(e) => {
                                    return ::core::result::Result::Err(
                                        e.chain("Could not decode `Call::account_withdraw_role.0`"),
                                    )
                                }
                                ::core::result::Result::Ok(__codec_res_edqy) => __codec_res_edqy,
                            }
                        },
                        {
                            let __codec_res_edqy =
                                <RoleMask as ::codec::Decode>::decode(__codec_input_edqy);
                            match __codec_res_edqy {
                                ::core::result::Result::Err(e) => {
                                    return ::core::result::Result::Err(
                                        e.chain("Could not decode `Call::account_withdraw_role.1`"),
                                    )
                                }
                                ::core::result::Result::Ok(__codec_res_edqy) => __codec_res_edqy,
                            }
                        },
                    ))
                }
                _ => ::core::result::Result::Err(<_ as ::core::convert::Into<_>>::into(
                    "Could not decode `Call`, variant doesn\'t exist",
                )),
            }
        }
    }
};
impl<T: Config> ::frame_support::dispatch::GetDispatchInfo for Call<T> {
    fn get_dispatch_info(&self) -> ::frame_support::dispatch::DispatchInfo {
        match *self {
            Call::account_add_with_role_and_data(ref who, ref role) => {
                let base_weight = 10_000 + T::DbWeight::get().reads_writes(2, 1);
                let weight = < dyn :: frame_support :: dispatch :: WeighData < (& T :: AccountId , & RoleMask) > > :: weigh_data (& base_weight , (who , role)) ;
                let class = <dyn ::frame_support::dispatch::ClassifyDispatch<(
                    &T::AccountId,
                    &RoleMask,
                )>>::classify_dispatch(&base_weight, (who, role));
                let pays_fee =
                    <dyn ::frame_support::dispatch::PaysFee<(&T::AccountId, &RoleMask)>>::pays_fee(
                        &base_weight,
                        (who, role),
                    );
                ::frame_support::dispatch::DispatchInfo {
                    weight,
                    class,
                    pays_fee,
                }
            }
            Call::account_set_with_role_and_data(ref who, ref role) => {
                let base_weight = 10_000 + T::DbWeight::get().reads_writes(2, 1);
                let weight = < dyn :: frame_support :: dispatch :: WeighData < (& T :: AccountId , & RoleMask) > > :: weigh_data (& base_weight , (who , role)) ;
                let class = <dyn ::frame_support::dispatch::ClassifyDispatch<(
                    &T::AccountId,
                    &RoleMask,
                )>>::classify_dispatch(&base_weight, (who, role));
                let pays_fee =
                    <dyn ::frame_support::dispatch::PaysFee<(&T::AccountId, &RoleMask)>>::pays_fee(
                        &base_weight,
                        (who, role),
                    );
                ::frame_support::dispatch::DispatchInfo {
                    weight,
                    class,
                    pays_fee,
                }
            }
            Call::set_master(ref who) => {
                let base_weight = 10_000 + T::DbWeight::get().reads_writes(2, 1);
                let weight =
                    <dyn ::frame_support::dispatch::WeighData<(&T::AccountId,)>>::weigh_data(
                        &base_weight,
                        (who,),
                    );
                let class = < dyn :: frame_support :: dispatch :: ClassifyDispatch < (& T :: AccountId ,) > > :: classify_dispatch (& base_weight , (who ,)) ;
                let pays_fee = <dyn ::frame_support::dispatch::PaysFee<(&T::AccountId,)>>::pays_fee(
                    &base_weight,
                    (who,),
                );
                ::frame_support::dispatch::DispatchInfo {
                    weight,
                    class,
                    pays_fee,
                }
            }
            Call::account_withdraw_role(ref who, ref role) => {
                let base_weight = 10_000 + T::DbWeight::get().reads_writes(2, 1);
                let weight = < dyn :: frame_support :: dispatch :: WeighData < (& T :: AccountId , & RoleMask) > > :: weigh_data (& base_weight , (who , role)) ;
                let class = <dyn ::frame_support::dispatch::ClassifyDispatch<(
                    &T::AccountId,
                    &RoleMask,
                )>>::classify_dispatch(&base_weight, (who, role));
                let pays_fee =
                    <dyn ::frame_support::dispatch::PaysFee<(&T::AccountId, &RoleMask)>>::pays_fee(
                        &base_weight,
                        (who, role),
                    );
                ::frame_support::dispatch::DispatchInfo {
                    weight,
                    class,
                    pays_fee,
                }
            }
            Call::__PhantomItem(_, _) => {
                ::core::panicking::panic_fmt(::core::fmt::Arguments::new_v1(
                    &["internal error: entered unreachable code: "],
                    &match (&"__PhantomItem should never be used.",) {
                        _args => [::core::fmt::ArgumentV1::new(
                            _args.0,
                            ::core::fmt::Display::fmt,
                        )],
                    },
                ))
            }
        }
    }
}
impl<T: Config> ::frame_support::dispatch::GetCallName for Call<T> {
    fn get_call_name(&self) -> &'static str {
        match *self {
            Call::account_add_with_role_and_data(ref who, ref role) => {
                let _ = (who, role);
                "account_add_with_role_and_data"
            }
            Call::account_set_with_role_and_data(ref who, ref role) => {
                let _ = (who, role);
                "account_set_with_role_and_data"
            }
            Call::set_master(ref who) => {
                let _ = (who);
                "set_master"
            }
            Call::account_withdraw_role(ref who, ref role) => {
                let _ = (who, role);
                "account_withdraw_role"
            }
            Call::__PhantomItem(_, _) => {
                ::core::panicking::panic_fmt(::core::fmt::Arguments::new_v1(
                    &["internal error: entered unreachable code: "],
                    &match (&"__PhantomItem should never be used.",) {
                        _args => [::core::fmt::ArgumentV1::new(
                            _args.0,
                            ::core::fmt::Display::fmt,
                        )],
                    },
                ))
            }
        }
    }
    fn get_call_names() -> &'static [&'static str] {
        &[
            "account_add_with_role_and_data",
            "account_set_with_role_and_data",
            "set_master",
            "account_withdraw_role",
        ]
    }
}
pub use ::frame_support::traits::GetPalletVersion as _;
impl<T: Config> ::frame_support::traits::GetPalletVersion for Module<T> {
    fn current_version() -> ::frame_support::traits::PalletVersion {
        frame_support::traits::PalletVersion {
            major: 0u16,
            minor: 1u8,
            patch: 7u8,
        }
    }
    fn storage_version() -> Option<::frame_support::traits::PalletVersion> {
        let key = ::frame_support::traits::PalletVersion::storage_key::<
            <T as frame_system::Config>::PalletInfo,
            Self,
        >()
        .expect("Every active pallet has a name in the runtime; qed");
        ::frame_support::storage::unhashed::get(&key)
    }
}
impl<T: Config> ::frame_support::traits::OnGenesis for Module<T> {
    fn on_genesis() {
        frame_support::traits::PalletVersion {
            major: 0u16,
            minor: 1u8,
            patch: 7u8,
        }
        .put_into_storage::<<T as frame_system::Config>::PalletInfo, Self>();
    }
}
impl<T: Config> ::frame_support::dispatch::Clone for Call<T> {
    fn clone(&self) -> Self {
        match *self {
            Call::account_add_with_role_and_data(ref who, ref role) => {
                Call::account_add_with_role_and_data((*who).clone(), (*role).clone())
            }
            Call::account_set_with_role_and_data(ref who, ref role) => {
                Call::account_set_with_role_and_data((*who).clone(), (*role).clone())
            }
            Call::set_master(ref who) => Call::set_master((*who).clone()),
            Call::account_withdraw_role(ref who, ref role) => {
                Call::account_withdraw_role((*who).clone(), (*role).clone())
            }
            _ => ::core::panicking::panic("internal error: entered unreachable code"),
        }
    }
}
impl<T: Config> ::frame_support::dispatch::PartialEq for Call<T> {
    fn eq(&self, _other: &Self) -> bool {
        match *self {
            Call::account_add_with_role_and_data(ref who, ref role) => {
                let self_params = (who, role);
                if let Call::account_add_with_role_and_data(ref who, ref role) = *_other {
                    self_params == (who, role)
                } else {
                    match *_other {
                        Call::__PhantomItem(_, _) => {
                            ::core::panicking::panic("internal error: entered unreachable code")
                        }
                        _ => false,
                    }
                }
            }
            Call::account_set_with_role_and_data(ref who, ref role) => {
                let self_params = (who, role);
                if let Call::account_set_with_role_and_data(ref who, ref role) = *_other {
                    self_params == (who, role)
                } else {
                    match *_other {
                        Call::__PhantomItem(_, _) => {
                            ::core::panicking::panic("internal error: entered unreachable code")
                        }
                        _ => false,
                    }
                }
            }
            Call::set_master(ref who) => {
                let self_params = (who,);
                if let Call::set_master(ref who) = *_other {
                    self_params == (who,)
                } else {
                    match *_other {
                        Call::__PhantomItem(_, _) => {
                            ::core::panicking::panic("internal error: entered unreachable code")
                        }
                        _ => false,
                    }
                }
            }
            Call::account_withdraw_role(ref who, ref role) => {
                let self_params = (who, role);
                if let Call::account_withdraw_role(ref who, ref role) = *_other {
                    self_params == (who, role)
                } else {
                    match *_other {
                        Call::__PhantomItem(_, _) => {
                            ::core::panicking::panic("internal error: entered unreachable code")
                        }
                        _ => false,
                    }
                }
            }
            _ => ::core::panicking::panic("internal error: entered unreachable code"),
        }
    }
}
impl<T: Config> ::frame_support::dispatch::Eq for Call<T> {}
impl<T: Config> ::frame_support::dispatch::fmt::Debug for Call<T> {
    fn fmt(
        &self,
        _f: &mut ::frame_support::dispatch::fmt::Formatter,
    ) -> ::frame_support::dispatch::result::Result<(), ::frame_support::dispatch::fmt::Error> {
        match *self {
            Call::account_add_with_role_and_data(ref who, ref role) => {
                _f.write_fmt(::core::fmt::Arguments::new_v1(
                    &["", ""],
                    &match (
                        &"account_add_with_role_and_data",
                        &(who.clone(), role.clone()),
                    ) {
                        _args => [
                            ::core::fmt::ArgumentV1::new(_args.0, ::core::fmt::Display::fmt),
                            ::core::fmt::ArgumentV1::new(_args.1, ::core::fmt::Debug::fmt),
                        ],
                    },
                ))
            }
            Call::account_set_with_role_and_data(ref who, ref role) => {
                _f.write_fmt(::core::fmt::Arguments::new_v1(
                    &["", ""],
                    &match (
                        &"account_set_with_role_and_data",
                        &(who.clone(), role.clone()),
                    ) {
                        _args => [
                            ::core::fmt::ArgumentV1::new(_args.0, ::core::fmt::Display::fmt),
                            ::core::fmt::ArgumentV1::new(_args.1, ::core::fmt::Debug::fmt),
                        ],
                    },
                ))
            }
            Call::set_master(ref who) => _f.write_fmt(::core::fmt::Arguments::new_v1(
                &["", ""],
                &match (&"set_master", &(who.clone(),)) {
                    _args => [
                        ::core::fmt::ArgumentV1::new(_args.0, ::core::fmt::Display::fmt),
                        ::core::fmt::ArgumentV1::new(_args.1, ::core::fmt::Debug::fmt),
                    ],
                },
            )),
            Call::account_withdraw_role(ref who, ref role) => {
                _f.write_fmt(::core::fmt::Arguments::new_v1(
                    &["", ""],
                    &match (&"account_withdraw_role", &(who.clone(), role.clone())) {
                        _args => [
                            ::core::fmt::ArgumentV1::new(_args.0, ::core::fmt::Display::fmt),
                            ::core::fmt::ArgumentV1::new(_args.1, ::core::fmt::Debug::fmt),
                        ],
                    },
                ))
            }
            _ => ::core::panicking::panic("internal error: entered unreachable code"),
        }
    }
}
impl<T: Config> ::frame_support::traits::UnfilteredDispatchable for Call<T> {
    type Origin = T::Origin;
    fn dispatch_bypass_filter(
        self,
        _origin: Self::Origin,
    ) -> ::frame_support::dispatch::DispatchResultWithPostInfo {
        match self {
            Call::account_add_with_role_and_data(who, role) => {
                <Module<T>>::account_add_with_role_and_data(_origin, who, role)
                    .map(Into::into)
                    .map_err(Into::into)
            }
            Call::account_set_with_role_and_data(who, role) => {
                <Module<T>>::account_set_with_role_and_data(_origin, who, role)
                    .map(Into::into)
                    .map_err(Into::into)
            }
            Call::set_master(who) => <Module<T>>::set_master(_origin, who)
                .map(Into::into)
                .map_err(Into::into),
            Call::account_withdraw_role(who, role) => {
                <Module<T>>::account_withdraw_role(_origin, who, role)
                    .map(Into::into)
                    .map_err(Into::into)
            }
            Call::__PhantomItem(_, _) => {
                ::core::panicking::panic_fmt(::core::fmt::Arguments::new_v1(
                    &["internal error: entered unreachable code: "],
                    &match (&"__PhantomItem should never be used.",) {
                        _args => [::core::fmt::ArgumentV1::new(
                            _args.0,
                            ::core::fmt::Display::fmt,
                        )],
                    },
                ))
            }
        }
    }
}
impl<T: Config> ::frame_support::dispatch::Callable<T> for Module<T> {
    type Call = Call<T>;
}
impl<T: Config> Module<T> {
    #[doc(hidden)]
    #[allow(dead_code)]
    pub fn call_functions() -> &'static [::frame_support::dispatch::FunctionMetadata] {
        &[
            ::frame_support::dispatch::FunctionMetadata {
                name: ::frame_support::dispatch::DecodeDifferent::Encode(
                    "account_add_with_role_and_data",
                ),
                arguments: ::frame_support::dispatch::DecodeDifferent::Encode(&[
                    ::frame_support::dispatch::FunctionArgumentMetadata {
                        name: ::frame_support::dispatch::DecodeDifferent::Encode("who"),
                        ty: ::frame_support::dispatch::DecodeDifferent::Encode("T::AccountId"),
                    },
                    ::frame_support::dispatch::FunctionArgumentMetadata {
                        name: ::frame_support::dispatch::DecodeDifferent::Encode("role"),
                        ty: ::frame_support::dispatch::DecodeDifferent::Encode("RoleMask"),
                    },
                ]),
                documentation: ::frame_support::dispatch::DecodeDifferent::Encode(&[]),
            },
            ::frame_support::dispatch::FunctionMetadata {
                name: ::frame_support::dispatch::DecodeDifferent::Encode(
                    "account_set_with_role_and_data",
                ),
                arguments: ::frame_support::dispatch::DecodeDifferent::Encode(&[
                    ::frame_support::dispatch::FunctionArgumentMetadata {
                        name: ::frame_support::dispatch::DecodeDifferent::Encode("who"),
                        ty: ::frame_support::dispatch::DecodeDifferent::Encode("T::AccountId"),
                    },
                    ::frame_support::dispatch::FunctionArgumentMetadata {
                        name: ::frame_support::dispatch::DecodeDifferent::Encode("role"),
                        ty: ::frame_support::dispatch::DecodeDifferent::Encode("RoleMask"),
                    },
                ]),
                documentation: ::frame_support::dispatch::DecodeDifferent::Encode(&[]),
            },
            ::frame_support::dispatch::FunctionMetadata {
                name: ::frame_support::dispatch::DecodeDifferent::Encode("set_master"),
                arguments: ::frame_support::dispatch::DecodeDifferent::Encode(&[
                    ::frame_support::dispatch::FunctionArgumentMetadata {
                        name: ::frame_support::dispatch::DecodeDifferent::Encode("who"),
                        ty: ::frame_support::dispatch::DecodeDifferent::Encode("T::AccountId"),
                    },
                ]),
                documentation: ::frame_support::dispatch::DecodeDifferent::Encode(&[]),
            },
            ::frame_support::dispatch::FunctionMetadata {
                name: ::frame_support::dispatch::DecodeDifferent::Encode("account_withdraw_role"),
                arguments: ::frame_support::dispatch::DecodeDifferent::Encode(&[
                    ::frame_support::dispatch::FunctionArgumentMetadata {
                        name: ::frame_support::dispatch::DecodeDifferent::Encode("who"),
                        ty: ::frame_support::dispatch::DecodeDifferent::Encode("T::AccountId"),
                    },
                    ::frame_support::dispatch::FunctionArgumentMetadata {
                        name: ::frame_support::dispatch::DecodeDifferent::Encode("role"),
                        ty: ::frame_support::dispatch::DecodeDifferent::Encode("RoleMask"),
                    },
                ]),
                documentation: ::frame_support::dispatch::DecodeDifferent::Encode(&[]),
            },
        ]
    }
}
impl<T: 'static + Config> Module<T> {
    #[doc(hidden)]
    #[allow(dead_code)]
    pub fn module_constants_metadata(
    ) -> &'static [::frame_support::dispatch::ModuleConstantMetadata] {
        &[]
    }
}
impl<T: Config> ::frame_support::dispatch::ModuleErrorMetadata for Module<T> {
    fn metadata() -> &'static [::frame_support::dispatch::ErrorMetadata] {
        <Error<T> as ::frame_support::dispatch::ModuleErrorMetadata>::metadata()
    }
}
impl<T: Config> Module<T> {
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
