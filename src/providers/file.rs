use regex::Regex;

use super::dependencies::*;
use crate::Text;


/// Methods collection for generate data related to files
pub struct File;

impl File {
    /// Get a random file extension from list
    ///
    /// # Arguments
    /// * `file_type` - FileType enum
    pub fn extension(file_type: Option<FileType>) -> &'static str {
        let ftype = match file_type {
            Some(x) => x.value(),
            None => get_random_element(FileType::variants().iter()).value()
        };

        get_random_element(EXTENSIONS.get(ftype).expect("Cant find file_type extensions!").iter())
    }

    /// Get a random mime type from list
    ///
    /// # Arguments
    /// * `file_type` - MimeType enum
    pub fn mime_type(mime_type: Option<MimeType>) -> &'static str {
        let mtype = match mime_type {
            Some(x) => x.value(),
            None => get_random_element(MimeType::variants().iter()).value()
        };

        get_random_element(MIME_TYPES.get(mtype).expect("Cant find mime_type!").iter())
    }

    /// Get a random file extension from list
    ///
    /// # Arguments
    /// * `minimum` - Minimum value
    /// * `maximum` - Maximum value
    pub fn size(minimum: i32, maximum: i32) -> String {
        format!("{} {}", randint(minimum, maximum), 
            get_random_element(vec!["bytes", "kB", "MB", "GB", "TB"].into_iter()))
    }

    /// Get a random file name with some extension
    ///
    /// # Arguments
    /// * `file_type` - FileType enum
    pub fn file_name(file_type: Option<FileType>) -> String {
        let replacer = get_random_element(vec!["_", "-"].into_iter());
        let word = Text(Locale::EN).word().trim();
        
        let re = Regex::new(r"\s+").unwrap();
        let name = re.replace_all(word, replacer);
        let ext = Self::extension(file_type);

        format!("{name}{ext}")
    }
}
