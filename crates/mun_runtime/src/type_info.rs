use generic_static::StaticTypeMap;
use once_cell::sync::OnceCell;
use std::ffi::{CStr, CString};
use std::sync::Once;

/// A trait that defines that for a type we can statically return a abi::TypeInfo
pub trait HasStaticTypeInfo {
    /// Returns a reference to the TypeInfo for the type
    fn type_info() -> &'static abi::TypeInfo;
}

/// A trait that defines that for a type we can statically return the name that would be used in a
/// abi::TypeInfo. This is useful for opaque types that we do not know the full details of but we
/// could use it as a pointer type
pub trait HasStaticTypeInfoName {
    /// Returns the type info name for the type
    fn type_name() -> &'static CStr;
}

/// Implement HasStaticTypeInfoName for everything that can produce a type info.
impl<T: HasStaticTypeInfo> HasStaticTypeInfoName for T {
    fn type_name() -> &'static CStr {
        unsafe { CStr::from_ptr(Self::type_info().name) }
    }
}

/// Every type that has at least a type name also has a valid pointer type name
impl<T: HasStaticTypeInfoName + 'static> HasStaticTypeInfo for *const T {
    fn type_info() -> &'static abi::TypeInfo {
        static mut VALUE: Option<StaticTypeMap<(CString, abi::TypeInfo)>> = None;
        static INIT: Once = Once::new();

        let map = unsafe {
            INIT.call_once(|| {
                VALUE = Some(StaticTypeMap::new());
            });
            VALUE.as_ref().unwrap()
        };

        &map.call_once::<T, _>(|| {
            let name =
                CString::new(format!("*const {}", T::type_name().to_str().unwrap())).unwrap();
            let guid = abi::Guid {
                b: md5::compute(&name.as_bytes()).0,
            };
            let name_ptr = name.as_ptr();
            (
                name,
                abi::TypeInfo {
                    guid,
                    name: name_ptr,
                    group: abi::TypeGroup::FundamentalTypes,
                    // size: std::mem::size_of::<*const T>() as u64,
                    // alignment: std::mem::align_of::<*const T>() as u32,
                },
            )
        })
        .1
    }
}

/// Every type that has at least a type name also has a valid pointer type name
impl<T: HasStaticTypeInfoName + 'static> HasStaticTypeInfo for *mut T {
    fn type_info() -> &'static abi::TypeInfo {
        static mut VALUE: Option<StaticTypeMap<(CString, abi::TypeInfo)>> = None;
        static INIT: Once = Once::new();

        let map = unsafe {
            INIT.call_once(|| {
                VALUE = Some(StaticTypeMap::new());
            });
            VALUE.as_ref().unwrap()
        };

        &map.call_once::<T, _>(|| {
            let name = CString::new(format!("*mut {}", T::type_name().to_str().unwrap())).unwrap();
            let guid = abi::Guid {
                b: md5::compute(&name.as_bytes()).0,
            };
            let name_ptr = name.as_ptr();
            (
                name,
                abi::TypeInfo {
                    guid,
                    name: name_ptr,
                    group: abi::TypeGroup::FundamentalTypes,
                    // size: std::mem::size_of::<*const T>() as u64,
                    // alignment: std::mem::align_of::<*const T>() as u32,
                },
            )
        })
        .1
    }
}

macro_rules! impl_basic_type_info {
    ($(
        $ty:ty
    ),+) => {
        $(
            impl HasStaticTypeInfo for $ty {
                fn type_info() -> &'static abi::TypeInfo {
                    static TYPE_INFO: OnceCell<abi::TypeInfo> = OnceCell::new();
                    TYPE_INFO.get_or_init(|| {
                        static TYPE_INFO_NAME: OnceCell<CString> = OnceCell::new();
                        let type_info_name: &'static CString = TYPE_INFO_NAME
                            .get_or_init(|| CString::new(format!("core::{}", stringify!($ty))).unwrap());

                        abi::TypeInfo {
                            guid: abi::Guid{ b: md5::compute(&type_info_name.as_bytes()).0 },
                            name: type_info_name.as_ptr(),
                            group: abi::TypeGroup::FundamentalTypes,
                            // size: std::mem::size_of::<$ty>() as u64,
                            // alignment: std::mem::align_of::<$ty>() as u32,
                        }
                    })
                }
            }
        )+
    }
}

macro_rules! impl_has_type_info_name {
    ($(
        $ty:ty => $name:tt
    ),+) => {
        $(
            impl crate::type_info::HasStaticTypeInfoName for $ty {
                fn type_name() -> &'static std::ffi::CStr {
                    static TYPE_INFO_NAME: once_cell::sync::OnceCell<std::ffi::CString> = once_cell::sync::OnceCell::new();
                    let type_info_name: &'static std::ffi::CString = TYPE_INFO_NAME
                        .get_or_init(|| std::ffi::CString::new($name).unwrap());
                    type_info_name.as_ref()
                }
            }
        )+
    }
}

impl_basic_type_info!(i8, i16, i32, i64, i128, u8, u16, u32, u64, u128, f32, f64, bool);

impl_has_type_info_name!(
    std::ffi::c_void => "core::void"
);

#[cfg(target_pointer_width = "64")]
impl HasStaticTypeInfo for usize {
    fn type_info() -> &'static abi::TypeInfo {
        u64::type_info()
    }
}

#[cfg(target_pointer_width = "64")]
impl HasStaticTypeInfo for isize {
    fn type_info() -> &'static abi::TypeInfo {
        i64::type_info()
    }
}

#[cfg(target_pointer_width = "32")]
impl HasStaticTypeInfo for usize {
    fn type_info() -> &'static abi::TypeInfo {
        u32::type_info()
    }
}

#[cfg(target_pointer_width = "32")]
impl HasStaticTypeInfo for isize {
    fn type_info() -> &'static abi::TypeInfo {
        i32::type_info()
    }
}

#[cfg(test)]
mod tests {
    use super::HasStaticTypeInfoName;

    #[test]
    fn ptr_test() {
        let ty = <*const std::ffi::c_void>::type_name();
        assert_eq!(ty.to_str().unwrap(), "*const core::void");

        let ty = <*const *mut std::ffi::c_void>::type_name();
        assert_eq!(ty.to_str().unwrap(), "*const *mut core::void");

        let ty = <*const *const std::ffi::c_void>::type_name();
        assert_eq!(ty.to_str().unwrap(), "*const *const core::void");
    }
}
