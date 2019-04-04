use std::mem::transmute;

type GetAllClassesFn = unsafe extern "thiscall" fn(thisptr: *const usize) -> *const usize;

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

    pub fn get_all_classes(&self) -> *const usize {
        let vfunc = unsafe { transmute::<_, GetAllClassesFn>(self.get_method(8)) };
        unsafe { vfunc(self.interface_pointer) }
    }
}
