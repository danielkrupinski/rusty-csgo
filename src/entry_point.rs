#![feature(abi_thiscall)]

extern crate chiter;
extern crate winapi;

mod macros;
mod types;
mod memory;
mod csgo;

use std::thread;
use std::mem::transmute;

fn entry_point() {
    unsafe { winapi::um::consoleapi::AllocConsole(); }

    let client = csgo::interfaces::ClientDll::new(memory::get_interface(c_str!("client_panorama.dll"), c_str!("VClient018")));
    let engine = csgo::interfaces::EngineClient::new(memory::get_interface(c_str!("engine.dll"), c_str!("VEngineClient014")));

    let mut client_hook = memory::vmt::Hook::new(memory::get_interface(c_str!("client_panorama.dll"), c_str!("VClient018")) as *mut usize);

    extern "fastcall" fn fsn(ecx: *const usize, edx: *const usize, stage: i32) {
        // type FsnFn = unsafe extern "fastcall" fn(ecx: *const usize, edx: *const usize, stage: i32);
        // unsafe { transmute::<_, FsnFn>(client_hook.get_original(37))(ecx, edx, stage); }

        println!("Current frame stage: {}", stage);
    }

    client_hook.swap(37, unsafe { transmute::<_, usize>(&fsn) });

    println!("VClient018::GetAllClasses() = {:?}", client.get_all_classes());
    println!("VEngineClient014::GetLocalPlayer() = {}", engine.get_local_player());
    
    let mut width: i32 = 0;
    let mut height: i32 = 0;

    engine.get_screen_size(&mut width, &mut height);

    println!("Screen size: {}px wide / {}px tall", width, height);
}

chiter::make_entrypoint!(entry_point);
