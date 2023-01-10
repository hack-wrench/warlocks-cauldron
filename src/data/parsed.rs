use super::parser::ParsedData;
use crate::generate_payload;

#[cfg(test)]
lazy_static! {
    pub static ref TEST: ParsedData = ParsedData::from(
        generate_payload!("locale_template", with_builtin)
    );
}

#[cfg(feature = "cs")]
lazy_static! {
    pub static ref CS: ParsedData = ParsedData::from(
        generate_payload!("cs")
    );
}

#[cfg(feature = "da")]
lazy_static! {
    pub static ref DA: ParsedData = ParsedData::from(
        generate_payload!("da")
    );
}

#[cfg(feature = "de")]
lazy_static! {
    pub static ref DE: ParsedData = ParsedData::from(
        generate_payload!("de")
    );
}

#[cfg(feature = "el")]
lazy_static! {
    pub static ref EL: ParsedData = ParsedData::from(
        generate_payload!("el")
    );
}

#[cfg(feature = "en")]
lazy_static! {
    pub static ref EN: ParsedData = ParsedData::from(
        generate_payload!("en", with_builtin)
    );
}

#[cfg(feature = "es")]
lazy_static! {
    pub static ref ES: ParsedData = ParsedData::from(
        generate_payload!("es")
    );
}

#[cfg(feature = "et")]
lazy_static! {
    pub static ref ET: ParsedData = ParsedData::from(
        generate_payload!("et")
    );
}

#[cfg(feature = "fa")]
lazy_static! {
    pub static ref FA: ParsedData = ParsedData::from(
        generate_payload!("fa")
    );
}

#[cfg(feature = "fi")]
lazy_static! {
    pub static ref FI: ParsedData = ParsedData::from(
        generate_payload!("fi")
    );
}

#[cfg(feature = "fr")]
lazy_static! {
    pub static ref FR: ParsedData = ParsedData::from(
        generate_payload!("fr")
    );
}

#[cfg(feature = "hu")]
lazy_static! {
    pub static ref HU: ParsedData = ParsedData::from(
        generate_payload!("hu")
    );
}

#[cfg(feature = "is")]
lazy_static! {
    pub static ref IS: ParsedData = ParsedData::from(
        generate_payload!("is")
    );
}

#[cfg(feature = "it")]
lazy_static! {
    pub static ref IT: ParsedData = ParsedData::from(
        generate_payload!("it", with_builtin)
    );
}

#[cfg(feature = "ja")]
lazy_static! {
    pub static ref JA: ParsedData = ParsedData::from(
        generate_payload!("ja")
    );
}

#[cfg(feature = "kk")]
lazy_static! {
    pub static ref KK: ParsedData = ParsedData::from(
        generate_payload!("kk")
    );
}

#[cfg(feature = "ko")]
lazy_static! {
    pub static ref KO: ParsedData = ParsedData::from(
        generate_payload!("ko")
    );
}

#[cfg(feature = "nl")]
lazy_static! {
    pub static ref NL: ParsedData = ParsedData::from(
        generate_payload!("nl")
    );
}

#[cfg(feature = "no")]
lazy_static! {
    pub static ref NO: ParsedData = ParsedData::from(
        generate_payload!("no")
    );
}

#[cfg(feature = "pl")]
lazy_static! {
    pub static ref PL: ParsedData = ParsedData::from(
        generate_payload!("pl")
    );
}

#[cfg(feature = "pt")]
lazy_static! {
    pub static ref PT: ParsedData = ParsedData::from(
        generate_payload!("pt")
    );
}

#[cfg(feature = "ru")]
lazy_static! {
    pub static ref RU: ParsedData = ParsedData::from(
        generate_payload!("ru", with_builtin)
    );
}

#[cfg(feature = "sk")]
lazy_static! {
    pub static ref SK: ParsedData = ParsedData::from(
        generate_payload!("sk")
    );
}

#[cfg(feature = "sv")]
lazy_static! {
    pub static ref SV: ParsedData = ParsedData::from(
        generate_payload!("sv")
    );
}

#[cfg(feature = "tr")]
lazy_static! {
    pub static ref TR: ParsedData = ParsedData::from(
        generate_payload!("tr")
    );
}

#[cfg(feature = "uk")]
lazy_static! {
    pub static ref UK: ParsedData = ParsedData::from(
        generate_payload!("uk", with_builtin)
    );
}

#[cfg(feature = "zh")]
lazy_static! {
    pub static ref ZH: ParsedData = ParsedData::from(
        generate_payload!("zh")
    );
}
