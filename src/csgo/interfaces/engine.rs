use std::mem::transmute;

type GetScreenSizeFn = unsafe extern "thiscall" fn(thisptr: *const usize, width: &mut i32, height: &mut i32);
type GetLocalPlayerFn = unsafe extern "thiscall" fn(thisptr: *const usize) -> i32;

pub struct Interface {
    interface_pointer: *const usize,
}

impl Interface {
    fn get_method(&self, index: isize) -> *const usize {
        let vtable = unsafe { *(self.interface_pointer) as *const usize };
        let vfunc = unsafe { vtable.offset(index).read() as *const usize };

        vfunc
    }

    pub fn new(interface_pointer: *const usize) -> Interface {
        Interface { interface_pointer }
    }

    pub fn get_screen_size(&self, width: &mut i32, height: &mut i32) {
        let vfunc = unsafe { transmute::<_, GetScreenSizeFn>(self.get_method(5)) };
        unsafe { vfunc(self.interface_pointer, width, height); }
    }

    pub fn get_local_player(&self) -> i32 {
        let vfunc = unsafe { transmute::<_, GetLocalPlayerFn>(self.get_method(12)) };
        unsafe { vfunc(self.interface_pointer) }
    }
}
