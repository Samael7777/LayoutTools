use windows::Win32::UI::Input::KeyboardAndMouse::HKL;
use crate::common_tools::to_fixed_utf16;

#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct KeyboardLayout {
    pub lang_id: u16,
    pub kl_id: u32,
    pub hkl: usize,
    pub short_name: [u16; 8],
    pub display_name: [u16; 128],
}

impl KeyboardLayout {
    pub fn default() -> Self {
        KeyboardLayout{
            lang_id: 0,
            kl_id: 0,
            hkl: 0,
            short_name: [0; 8],
            display_name: [0; 128],
        }
    }
    
    pub fn new(lang_id: u16, kl_id: u32, hkl: HKL, short_name : &str, display_name: &str) -> Self{
        KeyboardLayout{
            lang_id,
            kl_id,
            hkl : hkl.0 as _,
            short_name: to_fixed_utf16(short_name),
            display_name: to_fixed_utf16(display_name),
        }
    }
}