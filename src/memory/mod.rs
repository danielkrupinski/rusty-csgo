pub mod vmt;

use std::mem::transmute;
use std::ptr::null_mut;

use super::types::CreateInterfaceFn;

use winapi::shared::minwindef::{FARPROC, HMODULE};

pub fn get_module(module_name: &str) -> HMODULE {
    unsafe { winapi::um::libloaderapi::GetModuleHandleA(module_name.as_ptr() as *const i8) }
}

pub fn get_export(module: HMODULE, export_name: &str) -> FARPROC {
    unsafe { winapi::um::libloaderapi::GetProcAddress(module, export_name.as_ptr() as *const i8) }
}

pub fn get_interface(module_name: &str, interface_name: &str) -> *const usize {
    let module = unsafe { winapi::um::libloaderapi::GetModuleHandleA(module_name.as_ptr() as *const i8) };
    let function_ptr = unsafe { winapi::um::libloaderapi::GetProcAddress(module, "CreateInterface\0".as_ptr() as *const i8) };
    let function: CreateInterfaceFn = unsafe { transmute::<_, CreateInterfaceFn>(function_ptr) };

    unsafe { function(interface_name.as_ptr() as *const i8, null_mut()) as *const usize }
}
