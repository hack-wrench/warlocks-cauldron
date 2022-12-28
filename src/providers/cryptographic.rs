use blake2::{Digest, digest::core_api::CoreWrapper};
use itertools::Itertools;

use super::dependecies::*;
pub use uuid::Uuid;


/// Struct that provides cryptographic data
pub struct Cryptographic;

impl Cryptographic {
    /// Generate UUID4 struct
    pub fn uuid_object() -> Uuid {
        Uuid::new_v4()
    }

    /// Generate UUID4 string
    pub fn uuid() -> String {
        Self::uuid_object().to_string()
    }

    /// Generate random hash
    ///
    /// # Arguments
    /// * `algorithm` - Enum object
    pub fn hash(fmt: Option<Algorithm>) -> String {
        let vars = Algorithm::variants();
        let algorithm = match fmt {
            Some(a) => a,
            None => get_random_element(vars.into_iter()),
        };

        let uuid = Self::uuid().as_bytes().to_vec();

        let bytes = match algorithm {
            Algorithm::MD5 => md5::Md5::new_with_prefix(uuid).finalize().to_vec(),
            Algorithm::SHA1 => sha1::Sha1::new_with_prefix(uuid).finalize().to_vec(),
            Algorithm::SHA224 => sha2::Sha224::new_with_prefix(uuid).finalize().to_vec(),
            Algorithm::SHA256 => sha2::Sha256::new_with_prefix(uuid).finalize().to_vec(),
            Algorithm::SHA384 => sha2::Sha384::new_with_prefix(uuid).finalize().to_vec(),
            Algorithm::SHA512 => sha2::Sha512::new_with_prefix(uuid).finalize().to_vec(),
            Algorithm::BLAKE2B => blake2::Blake2b512::new_with_prefix(uuid).finalize().to_vec(),
            Algorithm::BLAKE2S => blake2::Blake2s256::new_with_prefix(uuid).finalize().to_vec(),
            _ => panic!("Incompatible algorithm!"),
        };

        String::from_utf8(bytes).expect("Cant convert hash to String!")
    }

    /// Generate byte string containing ``entropy`` bytes
    /// 
    /// The string has ``entropy`` random bytes, each byte
    /// converted to two hex digits.
    /// 
    /// # Arguments
    /// * `entropy` - Number of bytes (default: 32)
    pub fn token_bytes(entropy: usize) -> Vec<u8> {
        urandom(entropy)
    }

    /// Return a random text string, in hexadecimal
    /// 
    /// The string has *entropy* random bytes, each byte converted to two
    /// hex digits.  If *entropy* is ``None`` or not supplied, a reasonable
    /// default is used.
    /// 
    /// # Arguments
    /// * `entropy` - Number of bytes (default: 32)
    pub fn token_hex(entropy: usize) -> String {
        hex::encode(Self::token_bytes(entropy))
    }

    /// Return a random URL-safe text string, in Base64 encoding
    /// 
    /// # Arguments
    /// * `entropy` - Number of bytes (default: 32)
    pub fn token_urlsafe(entropy: usize) -> String {
        let bytes = Self::token_bytes(entropy);
        let engine = base64::engine::fast_portable::FastPortable::from(
            &base64::alphabet::URL_SAFE,
            base64::engine::fast_portable::NO_PAD,
        ); 

        base64::encode_engine(bytes, &engine)
    }

    /// Generate BIP-39-compatible mnemonic phrase
    pub fn mnemonic_phrase() -> String {
        (0..randint(12, 24)).map(|_| get_random_element(WORDLIST.iter()))
            .join(" ")
    }
}
