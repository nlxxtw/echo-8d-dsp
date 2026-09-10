//! EchoMusic DSP Provider ABI bindings (v2).

use std::os::raw::{c_char, c_void};

pub const PROVIDER_ABI_VERSION: u32 = 2;

#[repr(C)]
pub struct EchoDspConfig {
    pub abi_version: u32,
    pub sample_rate: u32,
    pub channels: u32,
    pub preferred_block_frames: u32,
    pub mode: u32,
    pub resource_json: *const c_char,
    pub preset_json: *const c_char,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct EchoDspInfo {
    pub abi_version: u32,
    pub latency_frames: u32,
    pub preferred_block_frames: u32,
    pub max_channels: u32,
    pub provider_id: *const c_char,
    pub provider_version: *const c_char,
    pub manifest_json: *const c_char,
    pub state_json: *const c_char,
}

#[repr(C)]
pub struct EchoDspApi {
    pub abi_version: u32,
    pub create: Option<unsafe extern "C" fn(*const EchoDspConfig, *mut EchoDspInfo) -> *mut c_void>,
    pub process: Option<unsafe extern "C" fn(*mut c_void, *mut f32, u32, u32) -> i32>,
    pub drain: Option<unsafe extern "C" fn(*mut c_void, *mut f32, u32, *mut u32) -> i32>,
    pub reset: Option<unsafe extern "C" fn(*mut c_void) -> i32>,
    pub configure: Option<unsafe extern "C" fn(*mut c_void, *const c_char) -> i32>,
    pub get_state_json: Option<unsafe extern "C" fn(*mut c_void) -> *const c_char>,
    pub destroy: Option<unsafe extern "C" fn(*mut c_void)>,
}
