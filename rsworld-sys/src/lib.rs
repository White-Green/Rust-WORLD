include!(concat!(env!("OUT_DIR"), "/bindgen.rs"));

use std::os::raw::{c_int, c_double};

impl CheapTrickOption {
    pub fn new(fs: c_int) -> Self {
        unsafe {
            let mut option = std::mem::MaybeUninit::uninit().assume_init();
            InitializeCheapTrickOption(fs, &mut option as *mut _);
            option
        }
    }
}

impl D4COption {
    pub fn new() -> Self {
        unsafe {
            let mut option:D4COption = std::mem::MaybeUninit::uninit().assume_init();
            InitializeD4COption(&mut option as *mut _);
            option
        }
    }
}

impl DioOption {
    pub fn new() -> Self {
        unsafe {
            let mut option:DioOption = std::mem::MaybeUninit::uninit().assume_init();
            InitializeDioOption(&mut option as *mut _);
            option
        }
    }
}

impl HarvestOption {
    pub fn new() -> Self {
        unsafe {
            let mut option:HarvestOption = std::mem::MaybeUninit::uninit().assume_init();
            InitializeHarvestOption(&mut option as *mut _);
            option
        }
    }
}
