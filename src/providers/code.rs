use super::dependencies::*;


/// Methods collection which provides methods for generating codes
pub struct Code;

impl Code {
    /// Get a random locale code (MS-LCID)
    ///
    /// return example: ru
    pub fn locale_code() -> &'static str {
        get_random_element(LOCALE_CODES.iter())
    }

    /// Generate a random ISSN
    ///
    /// return example: 1313-6666
    pub fn issn() -> String {
        custom_code("####-####", '@', '#')
    }

    /// Generate ISBN for current locale
    ///
    /// return example: isbn formatted string
    /// 
    /// # Arguments
    /// * `fmt` - ISBN format
    /// * `locale` - Locale code from enum
    pub fn isbn(fmt: Option<ISBNFormat>, locale: Locale) -> String {
        let data = locale.get_data();

        let fmt_key = validate_enum(fmt, None);

        let mask = ISBN_MASKS.get(fmt_key).expect("ISBN_MASKS doesnt have current ISBNFormat!")
            .replace("{0}", ISBN_GROUPS.get(&data.lang_code[..]).unwrap_or_else(|| ISBN_GROUPS.get("default").unwrap()));

        custom_code(&mask, '@', '#')
    }

    /// Generate EAN
    /// 
    /// return example: ean formatted string
    /// 
    /// # Arguments
    /// * `fmt` - Format of EAN
    pub fn ean(fmt: Option<EANFormat>) -> String {
        custom_code(EAN_MASKS.get(validate_enum(fmt, None)).expect("EAN_MASKS doesnt have current EANFormat!"), '@', '#')
    }

    /// Generate a random IMEI
    /// 
    /// return example: imei string
    pub fn imei() -> String {
        let num = format!("{}{}", get_random_element(IMEI_TACS.iter()), randint(100000, 999999));
        format!("{num}{}", luhn::checksum(num.as_bytes()))
    }

    /// Generate a random PIN code
    /// 
    /// return example: pin string
    pub fn pin() -> String {
        custom_code("####", '@', '#')
    }
}
