pub use crate::macros::ValuedEnum;
use crate::{
    data::{parser::ParsedData, parsed::*},
    valued_enum,
};

/// This enum provides access to the supported locales from one place
#[derive(Clone)]
pub enum Locale {
    #[cfg(feature = "cs")]
    CS,

    #[cfg(feature = "da")]
    DA,

    #[cfg(feature = "de")]
    DE,

    #[cfg(feature = "el")]
    EL,

    #[cfg(feature = "en")]
    EN,

    #[cfg(feature = "es")]
    ES,

    #[cfg(feature = "et")]
    ET,

    #[cfg(feature = "fa")]
    FA,

    #[cfg(feature = "fi")]
    FI,

    #[cfg(feature = "fr")]
    FR,

    #[cfg(feature = "hu")]
    HU,

    #[cfg(feature = "is")]
    IS,

    #[cfg(feature = "it")]
    IT, 

    #[cfg(feature = "ja")]
    JA,

    #[cfg(feature = "kk")]
    KK,

    #[cfg(feature = "ko")]
    KO,

    #[cfg(feature = "nl")]
    NL,

    #[cfg(feature = "no")]
    NO,

    #[cfg(feature = "pl")]
    PL,

    #[cfg(feature = "pt")]
    PT,

    #[cfg(feature = "ru")]
    RU,

    #[cfg(feature = "sk")]
    SK,

    #[cfg(feature = "sv")]
    SV,

    #[cfg(feature = "tr")]
    TR,

    #[cfg(feature = "uk")]
    UK,

    #[cfg(feature = "zh")]
    ZH,
}

impl Locale {
    pub(crate) fn get_data(&self) -> &'static ParsedData {
        match self {
            #[cfg(feature = "cs")]
            Self::CS => &CS,
        
            #[cfg(feature = "da")]
            Self::DA => &DA,
        
            #[cfg(feature = "de")]
            Self::DE => &DE,
        
            #[cfg(feature = "el")]
            Self::EL => &EL,
        
            #[cfg(feature = "en")]
            Self::EN => &EN,
    
            #[cfg(feature = "es")]
            Self::ES => &ES,
        
            #[cfg(feature = "et")]
            Self::ET => &ET,
        
            #[cfg(feature = "fa")]
            Self::FA => &FA,
        
            #[cfg(feature = "fi")]
            Self::FI => &FI,
        
            #[cfg(feature = "fr")]
            Self::FR => &FR,
        
            #[cfg(feature = "hu")]
            Self::HU => &HU,
        
            #[cfg(feature = "is")]
            Self::IS => &IS,
        
            #[cfg(feature = "it")]
            Self::IT => &IT, 
        
            #[cfg(feature = "ja")]
            Self::JA => &JA,
        
            #[cfg(feature = "kk")]
            Self::KK => &KK,
        
            #[cfg(feature = "ko")]
            Self::KO => &KO,
        
            #[cfg(feature = "nl")]
            Self::NL => &NL,
        
            #[cfg(feature = "no")]
            Self::NO => &NO,
        
            #[cfg(feature = "pl")]
            Self::PL => &PL,
        
            #[cfg(feature = "pt")]
            Self::PT => &PT,
        
            #[cfg(feature = "ru")]
            Self::RU => &RU,
        
            #[cfg(feature = "sk")]
            Self::SK => &SK,
        
            #[cfg(feature = "sv")]
            Self::SV => &SV,
        
            #[cfg(feature = "tr")]
            Self::TR => &TR,
        
            #[cfg(feature = "uk")]
            Self::UK => &UK,
        
            #[cfg(feature = "zh")]
            Self::ZH => &ZH, 
        }
    }
}

valued_enum! {
    /// Represents port ranges
    pub PortRange((u16, u16)):
        ALL = (1, 65535)
        WELL_KNOWN = (1, 1023)
        EPHEMERAL = (49152, 65535)
        REGISTERED = (1024, 49151)
}

valued_enum! {
    /// Represents genders
    pub Gender(&'static str):
        MALE = "male"
        FEMALE = "female"
}

valued_enum! {
    /// Represents title types
    pub TitleType(&'static str):
        TYPICAL = "typical"
        ACADEMIC = "academic"
}

valued_enum! {
    /// Provides credit card types
    pub CardType(&'static str):
        VISA = "Visa"
        MASTER_CARD = "MasterCard"
        AMERICAN_EXPRESS = "American Express"
}

valued_enum! {
    /// Provides algorithms which available
    pub Algorithm(&'static str):
        MD5 = "md5"
        SHA1 = "sha1"
        SHA224 = "sha224"
        SHA256 = "sha256"
        SHA384 = "sha384"
        SHA512 = "sha512"
        BLAKE2B = "blake2b"
        BLAKE2S = "blake2s"
}

valued_enum! {
    /// Provides top level domain types
    pub TLDType(&'static str):
        CCTLD = "cctld"
        GTLD = "gtld"
        GEOTLD = "geotld"
        UTLD = "utld"
        STLD = "stld"
}

valued_enum! {
    /// Provides file types
    pub FileType(&'static str):
        SOURCE = "source"
        TEXT = "text"
        DATA = "data"
        AUDIO = "audio"
        VIDEO = "video"
        IMAGE = "image"
        EXECUTABLE = "executable"
        COMPRESSED = "compressed"
}

valued_enum! {
    /// Provides common mime types
    pub MimeType(&'static str):
        APPLICATION = "application"
        AUDIO = "audio"
        IMAGE = "image"
        MESSAGE = "message"
        TEXT = "text"
        VIDEO = "video"
}

valued_enum! {
    /// Provides prefix signs
    pub MetricPrefixSign(&'static str):
        POSITIVE = "positive"
        NEGATIVE = "negative"
}

valued_enum! {
    /// Provides types of country codes
    pub CountryCode(&'static str):
        A2 = "a2"
        A3 = "a3"
        NUMERIC = "numeric"
        IOC = "ioc"
        FIFA = "fifa"
}

valued_enum! {
    /// Provides formats of ISBN
    pub ISBNFormat(&'static str):
        ISBN13 = "isbn-13"
        ISBN10 = "isbn-10"
}

valued_enum! {
    /// Provides formats of EAN
    pub EANFormat(&'static str):
        EAN8 = "ean-8"
        EAN13 = "ean-13"
}

valued_enum! {
    /// Provide unit names
    pub MeasureUnit((&'static str, &'static str)):
        MASS = ("gram", "gr")
        INFORMATION = ("byte", "b")
        THERMODYNAMIC_TEMPERATURE = ("kelvin", "K")
        AMOUNT_OF_SUBSTANCE = ("mole", "mol")
        ANGLE = ("radian", "r")
        SOLID_ANGLE = ("steradian", "㏛")
        FREQUENCY = ("hertz", "Hz")
        FORCE = ("newton", "N")
        PRESSURE = ("pascal", "P")
        ENERGY = ("joule", "J")
        POWER = ("watt", "W")
        FLUX= ("watt", "W")
        ELECTRIC_CHARGE = ("coulomb", "C")
        VOLTAGE = ("volt", "V")
        ELECTRIC_CAPACITANCE = ("farad", "F")
        ELECTRIC_RESISTANCE = ("ohm", "Ω")
        ELECTRICAL_CONDUCTANCE = ("siemens", "S")
        MAGNETIC_FLUX = ("weber", "Wb")
        MAGNETIC_FLUX_DENSITY = ("tesla", "T")
        INDUCTANCE = ("henry", "H")
        TEMPERATURE = ("Celsius", "°C")
        RADIOACTIVITY = ("becquerel", "Bq")
}

valued_enum! {
    /// Provides the number types
    pub NumType(&'static str):
        FLOAT = "floats"
        INTEGER = "integers"
        COMPLEX = "complexes"
        DECIMAL = "decimals"
}

valued_enum! {
    /// Provides the vide file types
    pub VideoFile(&'static str):
        MP4 = "mp4"
        MOV = "mov"
}

valued_enum! {
    /// Provides the audio file types
    pub AudioFile(&'static str):
        MP3 = "mp3"
        AAC = "aac"
}

valued_enum! {
    /// Provides the image file types
    pub ImageFile(&'static str):
        JPG = "jpg"
        PNG = "png"
        GIF = "gif"
}

valued_enum! {
    /// Provides the document file types
    pub DocumentFile(&'static str):
        PDF = "pdf"
        DOCX = "docx"
        PPTX = "pptx"
        XLSX = "xlsx"
}

valued_enum! {
    /// Provides the compressed file types
    pub CompressedFile(&'static str):
        ZIP = "zip"
        GZIP = "gz"
}

valued_enum! {
    /// Provides URL schemes
    pub TestEnum((u8, u8)):
        ONE   = (1, 1)
        TWO   = (2, 2)
        THREE = (3, 3)
}

valued_enum! {
    /// Provides URL schemes
    pub URLScheme(&'static str):
        WS = "ws"
        WSS = "wss"
        FTP = "ftp"
        SFTP = "sftp"
        HTTP = "http"
        HTTPS = "https"
}

valued_enum! {
    /// Provides regions of timezones
    pub TimezoneRegion(&'static str):
        AFRICA = "Africa"
        AMERICA = "America"
        ANTARCTICA = "Antarctica"
        ARCTIC = "Arctic"
        ASIA = "Asia"
        ATLANTIC = "Atlantic"
        AUSTRALIA = "Australia"
        EUROPE = "Europe"
        INDIAN = "Indian"
        PACIFIC = "Pacific"
}

valued_enum! {
    /// Provides URI and port of DSN
    pub DSNType((&'static str, u16)):
        POSTGRES = ("postgres", 5432)
        MYSQL = ("mysql", 3306)
        MONGODB = ("mongodb", 27017)
        REDIS = ("redis", 6379)
        COUCHBASE = ("couchbase", 8092)
        MEMCACHED = ("memcached", 11211)
        RABBITMQ = ("rabbitmq", 5672)
}
