include!(concat!(env!("OUT_DIR"), "/bindgen.rs"));

use std::mem::MaybeUninit;
use std::os::raw::c_int;

impl CheapTrickOption {
    pub fn new(fs: c_int) -> Self {
        unsafe {
            let mut value = MaybeUninit::uninit();
            InitializeCheapTrickOption(fs, value.as_mut_ptr());
            value.assume_init()
        }
    }
}

impl D4COption {
    pub fn new() -> Self {
        unsafe {
            let mut value = MaybeUninit::uninit();
            InitializeD4COption(value.as_mut_ptr());
            value.assume_init()
        }
    }
}

impl DioOption {
    pub fn new() -> Self {
        unsafe {
            let mut value = MaybeUninit::uninit();
            InitializeDioOption(value.as_mut_ptr());
            value.assume_init()
        }
    }
}

impl ForwardRealFFT {
    pub fn new(fft_size: c_int) -> Self {
        unsafe {
            let mut value = MaybeUninit::uninit();
            InitializeForwardRealFFT(fft_size, value.as_mut_ptr());
            value.assume_init()
        }
    }
}

impl HarvestOption {
    pub fn new() -> Self {
        unsafe {
            let mut value = MaybeUninit::uninit();
            InitializeHarvestOption(value.as_mut_ptr());
            value.assume_init()
        }
    }
}

impl InverseComplexFFT {
    pub fn new(fft_size: c_int) -> Self {
        unsafe {
            let mut value = MaybeUninit::uninit();
            InitializeInverseComplexFFT(fft_size, value.as_mut_ptr());
            value.assume_init()
        }
    }
}

impl InverseRealFFT {
    pub fn new(fft_size: c_int) -> Self {
        unsafe {
            let mut value = MaybeUninit::uninit();
            InitializeInverseRealFFT(fft_size, value.as_mut_ptr());
            value.assume_init()
        }
    }
}

impl MinimumPhaseAnalysis {
    pub fn new(fft_size: c_int) -> Self {
        unsafe {
            let mut value = MaybeUninit::uninit();
            InitializeMinimumPhaseAnalysis(fft_size, value.as_mut_ptr());
            value.assume_init()
        }
    }
}

impl WorldSynthesizer {
    pub fn new(
        fs: c_int,
        frame_period: f64,
        fft_size: c_int,
        buffer_size: c_int,
        number_of_pointers: c_int,
    ) -> Self {
        unsafe {
            let mut value = MaybeUninit::uninit();
            InitializeSynthesizer(
                fs,
                frame_period,
                fft_size,
                buffer_size,
                number_of_pointers,
                value.as_mut_ptr(),
            );
            value.assume_init()
        }
    }
}
