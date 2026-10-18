#![allow(dead_code, non_snake_case)]

use crate::common_tools::{copy_vec_to_buffer, get_hkl, get_string_resource, is_layout_name, lang_to_name, parse_lang_kb, parse_resource_link};
use crate::keyboard_layout::KeyboardLayout;
use std::string::String;
use winreg::enums::HKEY_CURRENT_USER;
use winreg::RegKey;

const LANG_PROFILE_KEY : &str = r"Control Panel\International\User Profile";
const CACHED_LANG_NAME_KEY : &str = r"CachedLanguageName";

pub fn get_user_layouts(buffer: *mut KeyboardLayout, buffer_len : i32) -> i32 {
    if buffer_len < 0 {
        return -1;
    }

    let is_calculate = buffer.is_null() || buffer_len == 0;
    let mut layouts : Vec<KeyboardLayout> = Vec::new();
    let hccu = RegKey::predef(HKEY_CURRENT_USER);
    let Ok(profile_root) = hccu.open_subkey(LANG_PROFILE_KEY) else {
        return -1;
    };

    for lang in profile_root.enum_keys().map(|x| x.unwrap()) {
        let Ok(layout_subkey) = profile_root.open_subkey(lang) else {
            continue;
        };

        let mut short_name = String::new();
        let mut display_name = String::new();

        let Some((lang_id, kl_id)) = parse_layout_params(&layout_subkey) else {
            continue;
        };

        if lang_id == 0 && kl_id == 0 {
            continue;
        }

        let Some(hkl) = get_hkl(kl_id) else {
            continue;
        };

        if !is_calculate  {
            if let Some(short_name_parsed) = lang_to_name(lang_id) {
                short_name = short_name_parsed;
            }

            if let Some(display_name_parsed) = get_layout_display_name(&layout_subkey) {
                display_name = display_name_parsed;
            }
        }

        let layout = KeyboardLayout::new(
            lang_id,
            kl_id,
            hkl,
            short_name.as_str(),
            display_name.as_str()
        );

        layouts.push(layout);
    }

    if is_calculate {
        return layouts.len() as i32;
    }

    copy_vec_to_buffer(&layouts, buffer, buffer_len as _)
}

fn parse_layout_params(reg_key: &RegKey) -> Option<(u16, u32)> {

    let Some(layout_string) = reg_key.enum_values()
        .filter_map(|v|v.ok())
        .map(|(name, _)| name)
        .find(|name| is_layout_name(name))
    else {
        return None;
    };

    parse_lang_kb(layout_string.as_str())
}

fn get_layout_display_name(reg_key: &RegKey) -> Option<String> {
    let Ok(resource_path) = reg_key.get_value::<String, _>(CACHED_LANG_NAME_KEY) else {
        return None;
    };

    let Some((path, uid)) = parse_resource_link(resource_path.as_str()) else {
        return None;
    };

    let Ok(display_name_parsed) = get_string_resource(path.as_str(), uid) else {
        return None;
    };

    Some(display_name_parsed)
}
