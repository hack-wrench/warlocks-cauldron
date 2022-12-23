use strum_macros::Display;

#[doc(hidden)]
#[derive(Display)]
#[strum(serialize_all = "kebab-case")]
pub(crate) enum TestLocale {
    TEST_TEST,
    TEST,
}

#[derive(Display)]
#[strum(serialize_all = "kebab-case")]
pub enum Locale {
    #[cfg(feature = "cs")]
    CS,

    #[cfg(feature = "da")]
    DA,

    #[cfg(feature = "de")]
    DE,

    #[cfg(feature = "de-at")]
    DE_AT,

    #[cfg(feature = "de-ch")]
    DE_CH,

    #[cfg(feature = "el")]
    EL,

    #[cfg(feature = "en")]
    EN,

    #[cfg(feature = "en-au")]
    EN_AU,

    #[cfg(feature = "en-ca")]
    EN_CA,

    #[cfg(feature = "en-gb")]
    EN_GB,

    #[cfg(feature = "es")]
    ES,

    #[cfg(feature = "es-mx")]
    ES_MX,

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

    #[cfg(feature = "nl-be")]
    NL_BE,

    #[cfg(feature = "no")]
    NO,

    #[cfg(feature = "pl")]
    PL,

    #[cfg(feature = "pt")]
    PT,

    #[cfg(feature = "pt-br")]
    PT_BR,

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


#[cfg(feature = "internet")]
pub enum PortRange {
    ALL,
    WELL_KNOWN,
    EPHEMERAL,
    REGISTERED
}

#[cfg(feature = "internet")]
impl Into<(u16, u16)> for PortRange {
    fn into(self) -> (u16, u16) {
        match self {
            Self::ALL => (1, 65535),
            Self::WELL_KNOWN => (1, 1023),
            Self::EPHEMERAL => (49152, 65535),
            Self::REGISTERED => (1024, 49151),
        }
    }
}


#[cfg(any(feature = "payment", feature = "person"))]
#[derive(Display)]
#[strum(serialize_all = "lowercase")]
pub enum Gender {
    MALE,
    FEMALE,
}


#[cfg(feature = "person")]
#[derive(Display)]
#[strum(serialize_all = "lowercase")]
pub enum TitleType {
    TYPICAL,
    ACADEMIC,
}


#[cfg(feature = "payment")]
#[derive(Display)]
pub enum CardType {
    #[strum(serialize = "Visa")]
    VISA,
    
    #[strum(serialize = "MasterCard")]
    MASTER_CARD,
    
    #[strum(serialize = "American Express")]
    AMERICAN_EXPRESS,
}


#[cfg(feature = "cryptographic")]
#[derive(Display)]
#[strum(serialize_all = "lowercase")]
pub enum Algorithm {
    MD5,
    SHA1,
    SHA224,
    SHA256,
    SHA384,
    SHA512,
    BLAKE2B,
    BLAKE2S,
}


#[cfg(feature = "internet")]
#[derive(Display)]
#[strum(serialize_all = "lowercase")]
pub enum TLDType {
    CCTLD,
    GTLD,
    GEOTLD,
    UTLD,
    STLD,
}


#[cfg(feature = "file")]
#[derive(Display)]
#[strum(serialize_all = "lowercase")]
pub enum FileType {
    SOURCE,
    TEXT,
    DATA,
    AUDIO,
    VIDEO,
    IMAGE,
    EXECUTABLE,
    COMPRESSED,
}


#[cfg(any(feature = "file", feature = "internet"))]
#[derive(Display)]
#[strum(serialize_all = "lowercase")]
pub enum MimeType {
    APPLICATION,
    AUDIO,
    IMAGE,
    MESSAGE,
    TEXT,
    VIDEO,
}



#[cfg(feature = "science")]
#[derive(Display)]
#[strum(serialize_all = "lowercase")]
pub enum MetricPrefixSign {
    POSITIVE,
    NEGATIVE,
}


#[cfg(feature = "address")]
#[derive(Display)]
#[strum(serialize_all = "lowercase")]
pub enum CountryCode {
    A2,
    A3,
    NUMERIC,
    IOC,
    FIFA,
}


#[cfg(feature = "code")]
#[derive(Display)]
pub enum ISBNFormat {
    #[strum(serialize = "isbn-13")]
    ISBN13,

    #[strum(serialize = "isbn-10")]
    ISBN10,
}


#[cfg(feature = "code")]
#[derive(Display)]
pub enum EANFormat {
    #[strum(serialize = "ean-8")]
    EAN8,
    
    #[strum(serialize = "ean-13")]
    EAN13,
}


#[cfg(feature = "science")]
pub enum MeasureUnit {
    MASS,
    INFORMATION,
    THERMODYNAMIC_TEMPERATURE,
    AMOUNT_OF_SUBSTANCE,
    ANGLE,
    SOLID_ANGLE,
    FREQUENCY,
    FORCE,
    PRESSURE,
    ENERGY,
    POWER,
    FLUX,
    ELECTRIC_CHARGE,
    VOLTAGE,
    ELECTRIC_CAPACITANCE,
    ELECTRIC_RESISTANCE,
    ELECTRICAL_CONDUCTANCE,
    MAGNETIC_FLUX,
    MAGNETIC_FLUX_DENSITY,
    INDUCTANCE,
    TEMPERATURE,
    RADIOACTIVITY,
}

#[cfg(feature = "science")]
impl Into<(&str, &str)> for MeasureUnit {
    fn into(self) -> (&'static str, &'static str) {
        match self {
            Self::MASS => ("gram", "gr"),
            Self::INFORMATION => ("byte", "b"),
            Self::THERMODYNAMIC_TEMPERATURE => ("kelvin", "K"),
            Self::AMOUNT_OF_SUBSTANCE => ("mole", "mol"),
            Self::ANGLE => ("radian", "r"),
            Self::SOLID_ANGLE => ("steradian", "㏛"),
            Self::FREQUENCY => ("hertz", "Hz"),
            Self::FORCE => ("newton", "N"),
            Self::PRESSURE => ("pascal", "P"),
            Self::ENERGY => ("joule", "J"),
            Self::POWER => ("watt", "W"),
            Self::FLUX => ("watt", "W"),
            Self::ELECTRIC_CHARGE => ("coulomb", "C"),
            Self::VOLTAGE => ("volt", "V"),
            Self::ELECTRIC_CAPACITANCE => ("farad", "F"),
            Self::ELECTRIC_RESISTANCE => ("ohm", "Ω"),
            Self::ELECTRICAL_CONDUCTANCE => ("siemens", "S"),
            Self::MAGNETIC_FLUX => ("weber", "Wb"),
            Self::MAGNETIC_FLUX_DENSITY => ("tesla", "T"),
            Self::INDUCTANCE => ("henry", "H"),
            Self::TEMPERATURE => ("Celsius", "°C"),
            Self::RADIOACTIVITY => ("becquerel", "Bq"),
        }
    }
}


#[cfg(feature = "numeric")]
#[derive(Display)]
pub enum NumType {
    #[strum(serialize = "floats")]
    FLOAT,

    #[strum(serialize = "integers")]
    INTEGER,

    #[strum(serialize = "complexes")]
    COMPLEX,

    #[strum(serialize = "decimals")]
    DECIMAL,
}


#[cfg(feature = "binaryfile")]
#[derive(Display)]
#[strum(serialize_all = "lowercase")]
pub enum VideoFile {
    MP4,
    MOV,
}


#[cfg(feature = "binaryfile")]
#[derive(Display)]
#[strum(serialize_all = "lowercase")]
pub enum AudioFile {
    MP3,
    AAC,
}


#[cfg(feature = "binaryfile")]
#[derive(Display)]
#[strum(serialize_all = "lowercase")]
pub enum ImageFile {
    JPG,
    PNG,
    GIF,
}


#[cfg(feature = "binaryfile")]
#[derive(Display)]
#[strum(serialize_all = "lowercase")]
pub enum DocumentFile {
    PDF,
    DOCX,
    PPTX,
    XLSX,
}


#[cfg(feature = "binaryfile")]
#[derive(Display)]
pub enum CompressedFile {
    #[strum(serialize = "zip")]
    ZIP,

    #[strum(serialize = "gz")]
    GZIP,
}


#[cfg(feature = "internet")]
#[derive(Display)]
#[strum(serialize_all = "lowercase")]
pub enum URLScheme {
    WS,
    WSS,
    FTP,
    SFTP,
    HTTP,
    HTTPS,
}


#[cfg(feature = "date")]
#[derive(Display)]
#[strum(serialize_all = "camel_case")]
pub enum TimezoneRegion {
    AFRICA,
    AMERICA,
    ANTARCTICA,
    ARCTIC,
    ASIA,
    ATLANTIC,
    AUSTRALIA,
    EUROPE,
    INDIAN,
    PACIFIC,
}


#[cfg(feature = "development")]
pub enum DSNType {
    POSTGRES,
    MYSQL,
    MONGODB,
    REDIS,
    COUCHBASE,
    MEMCACHED,
    RABBITMQ,
}

#[cfg(feature = "development")]
impl Into<(&str, u16)> for DSNType {
    fn into(self) -> (&'static str, u16) {
        match self {
            Self::POSTGRES => ("postgres", 5432),
            Self::MYSQL => ("mysql", 3306),
            Self::MONGODB => ("mongodb", 27017),
            Self::REDIS => ("redis", 6379),
            Self::COUCHBASE => ("couchbase", 8092),
            Self::MEMCACHED => ("memcached", 11211),
            Self::RABBITMQ => ("rabbitmq", 5672),
        }
    }
}