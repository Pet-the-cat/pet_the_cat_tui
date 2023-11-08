use sys_locale::get_locale;

pub fn set_to_system() {
    let locale = get_locale().unwrap_or_else(|| String::from("en"));
    let available_locales: Vec<&str> = available_locales!();

    if !available_locales.contains(&locale.as_str()) && available_locales.contains(&&locale[..2])  {
        rust_i18n::set_locale(&&locale[..2]);
    } else {
        rust_i18n::set_locale(&locale);
    }
}