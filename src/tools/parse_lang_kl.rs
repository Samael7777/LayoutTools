pub fn parse_lang_kl(str: &str) -> Option<(u16, u32)> {
    let klid_str: String;
    let langid_str: String;

    if str.contains(':') {
        let parts: Vec<&str> = str.split(':').collect();
        if parts.len() != 2 || parts[1].len() != 8 {
            return None;
        }
        langid_str = parts[0].to_string();
        klid_str = parts[1].to_string();
    } else {
        if str.len() != 8 {
            return None;
        }
        klid_str = str.to_string();
        langid_str = str[4..].to_string();
    }

    let Ok(lang_id) = u16::from_str_radix(&langid_str, 16) else {
        return None;
    };

    let Ok(kl_id) = u32::from_str_radix(&klid_str, 16) else {
        return None;
    };

    Some((lang_id, kl_id))
}