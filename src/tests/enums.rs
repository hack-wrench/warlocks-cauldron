use crate::enums::*;

#[test]
fn enums() {
    #[cfg(feature = "internet")] {
        let port: (u16, u16) = PortRange::ALL.into();
        assert_eq!(port, (1, 65535));
    }

    #[cfg(any(feature = "payment", feature = "person"))]
    assert_eq!(Gender::MALE.value(), "male");

    #[cfg(feature = "person")]
    assert_eq!(TitleType::TYPICAL.value(), "typical");

    #[cfg(feature = "payment")] {
        assert_eq!(CardType::VISA.value(), "Visa");
        assert_eq!(CardType::MASTER_CARD.value(), "MasterCard");
        assert_eq!(CardType::AMERICAN_EXPRESS.value(), "American Express");
    }

    #[cfg(feature = "cryptographic")] {
        assert_eq!(Algorithm::MD5.value(), "md5");
        assert_eq!(Algorithm::BLAKE2B.value(), "blake2b");
    }

    #[cfg(feature = "internet")]
    assert_eq!(TLDType::CCTLD.value(), "cctld");

    #[cfg(feature = "file")]
    assert_eq!(FileType::SOURCE.value(), "source");

    #[cfg(any(feature = "file", feature = "internet"))]
    assert_eq!(MimeType::APPLICATION.value(), "application");

    #[cfg(feature = "science")]
    assert_eq!(MetricPrefixSign::POSITIVE.value(), "positive");

    #[cfg(feature = "address")] {
        assert_eq!(CountryCode::A2.value(), "a2");
        assert_eq!(CountryCode::NUMERIC.value(), "numeric");
    }

    #[cfg(feature = "code")]
    assert_eq!(ISBNFormat::ISBN13.value(), "isbn-13");

    #[cfg(feature = "code")]
    assert_eq!(EANFormat::EAN8.value(), "ean-8");

    #[cfg(feature = "science")] {
        let measure: (&str, &str) = MeasureUnit::MASS.into();
        assert_eq!(measure, ("gram", "gr"));
    }

    #[cfg(feature = "numeric")] {
        assert_eq!(NumType::FLOAT.value(), "floats");
    }
    
    #[cfg(feature = "binaryfile")] {
        assert_eq!(VideoFile::MP4.value(), "mp4");
        assert_eq!(VideoFile::MOV.value(), "mov");
    }


    #[cfg(feature = "binaryfile")] {
        assert_eq!(AudioFile::MP3.value(), "mp3");
        assert_eq!(AudioFile::AAC.value(), "aac");
    }


    #[cfg(feature = "binaryfile")] {
        assert_eq!(ImageFile::JPG.value(), "jpg");
    }
    

    #[cfg(feature = "binaryfile")] {
        assert_eq!(DocumentFile::PDF.value(), "pdf");
    }
   

    #[cfg(feature = "binaryfile")] {
        assert_eq!(CompressedFile::ZIP.value(), "zip");
        assert_eq!(CompressedFile::GZIP.value(), "gz");
    }

    #[cfg(feature = "internet")]
    assert_eq!(URLScheme::WS.value(), "ws");

    #[cfg(feature = "date")]
    assert_eq!(TimezoneRegion::AFRICA.value(), "Africa");

    #[cfg(feature = "development")] {
        let dsn: (&str, u16) = DSNType::POSTGRES.into();
        assert_eq!(dsn, ("postgres", 5432));
    }
}
