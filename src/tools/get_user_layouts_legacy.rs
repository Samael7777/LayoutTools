use crate::common_tools::{copy_vec_to_buffer, get_hkl, get_string_resource, lang_to_name, num_to_hex, parse_lang_kb, parse_resource_link, u32_from_hex};
use crate::keyboard_layout::KeyboardLayout;
use winreg::RegKey;
use winreg::enums::{HKEY_CURRENT_USER, HKEY_LOCAL_MACHINE};

const SUBSTITUTES_KEY: &str = r"Keyboard Layout\Substitutes";
const PRELOADED_LANGS_KEY: &str = r"Keyboard Layout\Preload";
const LAYOUTS_KEY: &str = r"SYSTEM\CurrentControlSet\Control\Keyboard Layouts";

#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetUserLayoutsLegacy(buffer: *mut KeyboardLayout, buffer_len: i32) -> i32 {
    if buffer_len < 0 {
        return -1;
    }
    let is_calculate = buffer.is_null() || buffer_len == 0;
    let mut layouts : Vec<KeyboardLayout> = Vec::new();

    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let Ok(preloaded_langs_key) = hkcu.open_subkey(PRELOADED_LANGS_KEY) else {
        return -1;
    };

    for reg_value in preloaded_langs_key.enum_values().map(|x| x.unwrap()) {
        let Some((lang_id, _)) = parse_lang_kb(&reg_value.1.to_string()) else {
            continue;
        };

        let kl_id = get_klid(lang_id);
        let Some(hkl) = get_hkl(kl_id) else {
            continue;
        };

        let mut short_name = String::new();
        let mut display_name = String::new();
        if !is_calculate {
            let Some(short_name_converted) = lang_to_name(lang_id as _) else {
                return -1
            };
            short_name = short_name_converted;

            if let Some(display_name_parsed) = get_display_name(kl_id){
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

    copy_vec_to_buffer(&layouts, buffer, buffer_len)
}

fn get_klid(lang_id : u16) -> u32 {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let Ok(substitutes_key) = hkcu.open_subkey(SUBSTITUTES_KEY) else {
        return lang_id as u32;
    };

    let key = format!("{:08X}", lang_id);
    let Ok(klid_str) = substitutes_key.get_value::<String, _>(key) else {
        return lang_id as u32;
    };

    let Ok(kl_id) = u32_from_hex(klid_str.as_str()) else {
        return lang_id as u32;
    };

    kl_id
}

fn get_display_name(kl_id: u32) -> Option<String> {
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let Ok(layouts_key) = hklm.open_subkey(LAYOUTS_KEY) else {
        return None;
    };

    let klid_subkey = num_to_hex(kl_id);
    let Ok(layout_descr_key) = layouts_key.open_subkey(klid_subkey) else {
        return None;
    };

    let Ok(resource_path) = layout_descr_key.get_value::<String, _>("Layout Display Name") else {
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