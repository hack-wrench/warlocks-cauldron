use crate::Internet;

use super::dependencies::*;


/// Methods collection for getting fake data for Developers
pub struct Development;

impl Development {
    /// Generates a random DSN (Data Source Name)
    ///
    /// return example: postgres://some.host:5432
    ///
    /// # Arguments
    /// * `tld_type` - DSNType provide service scheme and port
    /// * `tld_type` - TLDType provide hostname domain
    /// * `subdomains` - vec of subdomains
    pub fn dsn(dsn_type: Option<DSNType>, tld_type: Option<TLDType>, subdomains: Option<Vec<&str>>) -> String {
        let hostname = Internet::hostname(tld_type, subdomains);
        let (scheme, port) = validate_enum(dsn_type, None);
        format!("{scheme}://{hostname}:{port}")
    }

    /// Generate a random software license
    ///
    /// return example: GNU General Public License (GPL)
    pub fn software_license() -> &'static str {
        get_random_element(LICENSES.iter())
    }

    /// Generate version number
    ///
    /// return example: 6.6.6
    /// 
    /// # Arguments
    /// * `calver` - ISBN format
    /// * `pre_release` - Locale code from enum
    pub fn version(calver: bool, pre_release: bool) -> String {
        let (major, minor, patch) = match calver {
            true => (randint(2016, 2018), randint(1, 10), randint(1, 10)),
            false => (randint(1, 10), randint(1, 10), randint(1, 10)),
        };

        let version = format!("{major}.{minor}.{patch}");

        if pre_release {
            let suffix = get_random_element(vec!["alpha", "beta", "rc"].into_iter());
            let number = randint(1, 11);
            format!("{version}-{suffix}.{number}")
        } else {
            version
        }
    }

    /// Get a random programming language
    /// 
    /// return example: Rust
    pub fn programming_language() -> &'static str {
        get_random_element(PROGRAMMING_LANGS.iter())
    }

    /// Get a random operating system or distributive name
    /// 
    /// return example: Windows 11
    pub fn os() -> &'static str {
        get_random_element(OS.iter())
    }

    /// Generate a random PIN code
    /// 
    /// return example: pin string
    pub fn boolean() -> bool {
        rand_bool(0.5)
    }
}
