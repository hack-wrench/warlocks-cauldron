use crate::data::parsed::*;

#[test]
fn parse_locales() {
    #[cfg(feature = "cs")]
    assert_eq!(CS.lang_code, "cs");

    #[cfg(feature = "da")]
    assert_eq!(DA.lang_code, "da");

    #[cfg(feature = "de")]
    assert_eq!(DE.lang_code, "de");

    #[cfg(feature = "el")]
    assert_eq!(EL.lang_code, "el");

    #[cfg(feature = "en")]
    assert_eq!(EN.lang_code, "en");

    #[cfg(feature = "es")]
    assert_eq!(ES.lang_code, "es");

    #[cfg(feature = "et")]
    assert_eq!(ET.lang_code, "et");

    #[cfg(feature = "fa")]
    assert_eq!(FA.lang_code, "fa");

    #[cfg(feature = "fi")]
    assert_eq!(FI.lang_code, "fi");

    #[cfg(feature = "fr")]
    assert_eq!(FR.lang_code, "fr");

    #[cfg(feature = "hu")]
    assert_eq!(HU.lang_code, "hu");

    #[cfg(feature = "is")]
    assert_eq!(IS.lang_code, "is");

    #[cfg(feature = "it")]
    assert_eq!(IT.lang_code, "it");

    #[cfg(feature = "ja")]
    assert_eq!(JA.lang_code, "ja");

    #[cfg(feature = "kk")]
    assert_eq!(KK.lang_code, "kk");

    #[cfg(feature = "ko")]
    assert_eq!(KO.lang_code, "ko");

    #[cfg(feature = "nl")]
    assert_eq!(NL.lang_code, "nl");

    #[cfg(feature = "no")]
    assert_eq!(NO.lang_code, "no");

    #[cfg(feature = "pl")]
    assert_eq!(PL.lang_code, "pl");

    #[cfg(feature = "pt")]
    assert_eq!(PT.lang_code, "pt");

    #[cfg(feature = "ru")]
    assert_eq!(RU.lang_code, "ru");

    #[cfg(feature = "sk")]
    assert_eq!(SK.lang_code, "sk");

    #[cfg(feature = "sv")]
    assert_eq!(SV.lang_code, "sv");

    #[cfg(feature = "tr")]
    assert_eq!(TR.lang_code, "tr");

    #[cfg(feature = "uk")]
    assert_eq!(UK.lang_code, "uk");

    #[cfg(feature = "zh")]
    assert_eq!(ZH.lang_code, "zh");
}