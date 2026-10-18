use crate::tools::get_hkl::get_hkl;
use crate::tools::get_localized_language_name::get_localized_language_name;
use crate::tools::get_localized_layout_name::get_localized_layout_name;
use crate::tools::get_user_layouts::get_user_layouts;
use crate::tools::get_user_layouts_legacy::get_user_layouts_legacy;

#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetHKLFromKLID(klid: u32) -> usize {
    if let Some(hkl) = get_hkl(klid) {
        return hkl.0 as usize;
    };

    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetUserLayouts(buffer: *mut u32, buffer_len : i32) -> i32 {
    get_user_layouts(buffer, buffer_len)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetUserLayoutsLegacy(buffer: *mut u32, buffer_len : i32) -> i32 {
    get_user_layouts_legacy(buffer, buffer_len)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetLocalizedLanguageName(lang_id: u16, buffer: *mut u16, buffer_len : i32) -> i32 {
    get_localized_language_name(lang_id, buffer, buffer_len)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetLocalizedLayoutName(kl_id: u32, buffer: *mut u16, buffer_len : i32) -> i32 {
    get_localized_layout_name(kl_id, buffer, buffer_len)
}