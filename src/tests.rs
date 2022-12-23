use crate::enums::*;

#[test]
fn enums() {
    assert_eq!(TestLocale::TEST_TEST.to_string(), "test-test");
    assert_eq!(TestLocale::TEST.to_string(), "test");

    #[cfg(feature = "internet")] {
        let port: (u16, u16) = PortRange::ALL.into();
        assert_eq!(port, (1, 65535));
    }

    #[cfg(any(feature = "payment", feature = "person"))]
    assert_eq!(Gender::MALE.to_string(), "male");

    #[cfg(feature = "person")]
    assert_eq!(TitleType::TYPICAL.to_string(), "typical");

    #[cfg(feature = "payment")] {
        assert_eq!(CardType::VISA.to_string(), "Visa");
        assert_eq!(CardType::MASTER_CARD.to_string(), "MasterCard");
        assert_eq!(CardType::AMERICAN_EXPRESS.to_string(), "American Express");
    }

    #[cfg(feature = "cryptographic")] {
        assert_eq!(Algorithm::MD5.to_string(), "md5");
        assert_eq!(Algorithm::BLAKE2B.to_string(), "blake2b");
    }

    #[cfg(feature = "internet")]
    assert_eq!(TLDType::CCTLD.to_string(), "cctld");

    #[cfg(feature = "file")]
    assert_eq!(FileType::SOURCE.to_string(), "source");

    #[cfg(any(feature = "file", feature = "internet"))]
    assert_eq!(MimeType::APPLICATION.to_string(), "application");

    #[cfg(feature = "science")]
    assert_eq!(MetricPrefixSign::POSITIVE.to_string(), "positive");

    #[cfg(feature = "address")] {
        assert_eq!(CountryCode::A2.to_string(), "a2");
        assert_eq!(CountryCode::NUMERIC.to_string(), "numeric");
    }

    #[cfg(feature = "code")]
    assert_eq!(ISBNFormat::ISBN13.to_string(), "isbn-13");

    #[cfg(feature = "code")]
    assert_eq!(EANFormat::EAN8.to_string(), "ean-8");

    #[cfg(feature = "science")] {
        let measure: (&str, &str) = MeasureUnit::MASS.into();
        assert_eq!(measure, ("gram", "gr"));
    }

    #[cfg(feature = "numeric")] {
        assert_eq!(NumType::FLOAT.to_string(), "floats");
    }
    
    #[cfg(feature = "binaryfile")] {
        assert_eq!(VideoFile::MP4.to_string(), "mp4");
        assert_eq!(VideoFile::MOV.to_string(), "mov");
    }


    #[cfg(feature = "binaryfile")] {
        assert_eq!(AudioFile::MP3.to_string(), "mp3");
        assert_eq!(AudioFile::AAC.to_string(), "aac");
    }


    #[cfg(feature = "binaryfile")] {
        assert_eq!(ImageFile::JPG.to_string(), "jpg");
    }
    

    #[cfg(feature = "binaryfile")] {
        assert_eq!(DocumentFile::PDF.to_string(), "pdf");
    }
   

    #[cfg(feature = "binaryfile")] {
        assert_eq!(CompressedFile::ZIP.to_string(), "zip");
        assert_eq!(CompressedFile::GZIP.to_string(), "gz");
    }

    #[cfg(feature = "internet")]
    assert_eq!(URLScheme::WS.to_string(), "ws");

    #[cfg(feature = "date")]
    assert_eq!(TimezoneRegion::AFRICA.to_string(), "Africa");

    #[cfg(feature = "development")] {
        let dsn: (&str, u16) = DSNType::POSTGRES.into();
        assert_eq!(dsn, ("postgres", 5432));
    }
}
