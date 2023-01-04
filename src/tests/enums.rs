use crate::enums::*;

#[test]
fn enums() {
    let port: (u16, u16) = PortRange::ALL.into();
    assert_eq!(port, (1, 65535));
    
    assert_eq!(Gender::MALE.value(), "male");

    assert_eq!(TitleType::TYPICAL.value(), "typical");

    assert_eq!(CardType::VISA.value(), "Visa");
    assert_eq!(CardType::MASTER_CARD.value(), "MasterCard");
    assert_eq!(CardType::AMERICAN_EXPRESS.value(), "American Express");

    assert_eq!(Algorithm::MD5.value(), "md5");
    assert_eq!(Algorithm::BLAKE2B.value(), "blake2b");

    assert_eq!(TLDType::CCTLD.value(), "cctld");

    assert_eq!(FileType::SOURCE.value(), "source");

    assert_eq!(MimeType::APPLICATION.value(), "application");

    assert_eq!(MetricPrefixSign::POSITIVE.value(), "positive");

    assert_eq!(CountryCode::A2.value(), "a2");
    assert_eq!(CountryCode::NUMERIC.value(), "numeric");

    assert_eq!(ISBNFormat::ISBN13.value(), "isbn-13");

    assert_eq!(EANFormat::EAN8.value(), "ean-8");

    let measure: (&str, &str) = MeasureUnit::MASS.into();
    assert_eq!(measure, ("gram", "gr"));

    assert_eq!(NumType::FLOAT.value(), "floats");
    
    assert_eq!(VideoFile::MP4.value(), "mp4");
    assert_eq!(VideoFile::MOV.value(), "mov");

    assert_eq!(AudioFile::MP3.value(), "mp3");
    assert_eq!(AudioFile::AAC.value(), "aac");

    assert_eq!(ImageFile::JPG.value(), "jpg");
    
    assert_eq!(DocumentFile::PDF.value(), "pdf");

    assert_eq!(CompressedFile::ZIP.value(), "zip");
    assert_eq!(CompressedFile::GZIP.value(), "gz");

    assert_eq!(URLScheme::WS.value(), "ws");

    assert_eq!(TimezoneRegion::AFRICA.value(), "Africa");

    let dsn: (&str, u16) = DSNType::POSTGRES.into();
    assert_eq!(dsn, ("postgres", 5432));
}
