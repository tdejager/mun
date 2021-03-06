/* automatically generated by rust-bindgen */

#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
use crate::Privacy;

#[doc = " <div rustbindgen derive=\"PartialEq\">"]
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Guid {
    pub b: [u8; 16usize],
}
#[test]
fn bindgen_test_layout_Guid() {
    assert_eq!(
        ::std::mem::size_of::<Guid>(),
        16usize,
        concat!("Size of: ", stringify!(Guid))
    );
    assert_eq!(
        ::std::mem::align_of::<Guid>(),
        1usize,
        concat!("Alignment of ", stringify!(Guid))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Guid>())).b as *const _ as usize },
        0usize,
        concat!("Offset of field: ", stringify!(Guid), "::", stringify!(b))
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TypeInfo {
    pub guid: Guid,
    pub name: *const ::std::os::raw::c_char,
}
#[test]
fn bindgen_test_layout_TypeInfo() {
    assert_eq!(
        ::std::mem::size_of::<TypeInfo>(),
        24usize,
        concat!("Size of: ", stringify!(TypeInfo))
    );
    assert_eq!(
        ::std::mem::align_of::<TypeInfo>(),
        8usize,
        concat!("Alignment of ", stringify!(TypeInfo))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<TypeInfo>())).guid as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(TypeInfo),
            "::",
            stringify!(guid)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<TypeInfo>())).name as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(TypeInfo),
            "::",
            stringify!(name)
        )
    );
}
#[doc = " <div rustbindgen derive=\"Clone\">"]
#[repr(C)]
#[derive(Clone)]
pub struct FunctionSignature {
    pub name: *const ::std::os::raw::c_char,
    pub arg_types: *const TypeInfo,
    pub return_type: *const TypeInfo,
    pub num_arg_types: u16,
    pub privacy: Privacy,
}
#[test]
fn bindgen_test_layout_FunctionSignature() {
    assert_eq!(
        ::std::mem::size_of::<FunctionSignature>(),
        32usize,
        concat!("Size of: ", stringify!(FunctionSignature))
    );
    assert_eq!(
        ::std::mem::align_of::<FunctionSignature>(),
        8usize,
        concat!("Alignment of ", stringify!(FunctionSignature))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<FunctionSignature>())).name as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(FunctionSignature),
            "::",
            stringify!(name)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<FunctionSignature>())).arg_types as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(FunctionSignature),
            "::",
            stringify!(arg_types)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<FunctionSignature>())).return_type as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(FunctionSignature),
            "::",
            stringify!(return_type)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<FunctionSignature>())).num_arg_types as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(FunctionSignature),
            "::",
            stringify!(num_arg_types)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<FunctionSignature>())).privacy as *const _ as usize },
        26usize,
        concat!(
            "Offset of field: ",
            stringify!(FunctionSignature),
            "::",
            stringify!(privacy)
        )
    );
}
#[doc = " <div rustbindgen derive=\"Clone\">"]
#[repr(C)]
#[derive(Clone)]
pub struct FunctionInfo {
    pub signature: FunctionSignature,
    pub fn_ptr: *const ::std::os::raw::c_void,
}
#[test]
fn bindgen_test_layout_FunctionInfo() {
    assert_eq!(
        ::std::mem::size_of::<FunctionInfo>(),
        40usize,
        concat!("Size of: ", stringify!(FunctionInfo))
    );
    assert_eq!(
        ::std::mem::align_of::<FunctionInfo>(),
        8usize,
        concat!("Alignment of ", stringify!(FunctionInfo))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<FunctionInfo>())).signature as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(FunctionInfo),
            "::",
            stringify!(signature)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<FunctionInfo>())).fn_ptr as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(FunctionInfo),
            "::",
            stringify!(fn_ptr)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ModuleInfo {
    pub path: *const ::std::os::raw::c_char,
    pub functions: *const FunctionInfo,
    pub num_functions: u32,
}
#[test]
fn bindgen_test_layout_ModuleInfo() {
    assert_eq!(
        ::std::mem::size_of::<ModuleInfo>(),
        24usize,
        concat!("Size of: ", stringify!(ModuleInfo))
    );
    assert_eq!(
        ::std::mem::align_of::<ModuleInfo>(),
        8usize,
        concat!("Alignment of ", stringify!(ModuleInfo))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ModuleInfo>())).path as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(ModuleInfo),
            "::",
            stringify!(path)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ModuleInfo>())).functions as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(ModuleInfo),
            "::",
            stringify!(functions)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ModuleInfo>())).num_functions as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(ModuleInfo),
            "::",
            stringify!(num_functions)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DispatchTable {
    pub signatures: *const FunctionSignature,
    pub fn_ptrs: *mut *const ::std::os::raw::c_void,
    pub num_entries: u32,
}
#[test]
fn bindgen_test_layout_DispatchTable() {
    assert_eq!(
        ::std::mem::size_of::<DispatchTable>(),
        24usize,
        concat!("Size of: ", stringify!(DispatchTable))
    );
    assert_eq!(
        ::std::mem::align_of::<DispatchTable>(),
        8usize,
        concat!("Alignment of ", stringify!(DispatchTable))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<DispatchTable>())).signatures as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(DispatchTable),
            "::",
            stringify!(signatures)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<DispatchTable>())).fn_ptrs as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(DispatchTable),
            "::",
            stringify!(fn_ptrs)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<DispatchTable>())).num_entries as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(DispatchTable),
            "::",
            stringify!(num_entries)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AssemblyInfo {
    pub symbols: ModuleInfo,
    pub dispatch_table: DispatchTable,
    pub dependencies: *const *const ::std::os::raw::c_char,
    pub num_dependencies: u32,
}
#[test]
fn bindgen_test_layout_AssemblyInfo() {
    assert_eq!(
        ::std::mem::size_of::<AssemblyInfo>(),
        64usize,
        concat!("Size of: ", stringify!(AssemblyInfo))
    );
    assert_eq!(
        ::std::mem::align_of::<AssemblyInfo>(),
        8usize,
        concat!("Alignment of ", stringify!(AssemblyInfo))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<AssemblyInfo>())).symbols as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(AssemblyInfo),
            "::",
            stringify!(symbols)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<AssemblyInfo>())).dispatch_table as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(AssemblyInfo),
            "::",
            stringify!(dispatch_table)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<AssemblyInfo>())).dependencies as *const _ as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(AssemblyInfo),
            "::",
            stringify!(dependencies)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<AssemblyInfo>())).num_dependencies as *const _ as usize },
        56usize,
        concat!(
            "Offset of field: ",
            stringify!(AssemblyInfo),
            "::",
            stringify!(num_dependencies)
        )
    );
}
