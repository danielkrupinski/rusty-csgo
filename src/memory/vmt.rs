pub struct Hook {
    hooked_class: *mut usize,

    original_methods: Vec<usize>,
    overriden_methods: Vec<usize>,
}

impl Hook {
    pub fn new(base_class: *mut usize) -> Hook {
        let mut method_count: isize = 0;
        let mut original_methods: Vec<usize> = Vec::new();

        unsafe {
            while !base_class.offset(method_count).is_null() {
                original_methods.push(base_class.offset(method_count).read());
                method_count += 1;
            }
        }

        Hook {
            hooked_class: base_class,
            original_methods,
            overriden_methods: vec![0usize; method_count as usize],
        }
    }

    pub fn swap(&mut self, index: isize, new_function: usize) {
        self.overriden_methods[index as usize] = new_function;
        unsafe { self.hooked_class.offset(index).write(new_function); }
    }

    pub fn restore(&mut self, index: usize) {
        let original = self.original_methods[index];
        self.overriden_methods[index] = original;
    }

    pub fn get_original(&self, index: usize) -> usize {
        self.original_methods[index]
    }
}
