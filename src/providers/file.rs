use regex::Regex;

use super::dependencies::*;
use crate::Text;


/// A struct, which provides methods for generating codes
pub struct File;

impl File {
    pub fn extension(file_type: Option<FileType>) -> &'static str {
        let ftypes = FileType::variants();
        let ftype = match file_type {
            Some(x) => x.value(),
            None => get_random_element(ftypes.iter()).value()
        };

        get_random_element(EXTENSIONS.get(ftype).expect("Cant find file_type extensions!").iter())
    }

    pub fn mime_type(mime_type: Option<MimeType>) -> &'static str {
        let mtypes = MimeType::variants();
        let mtype = match mime_type {
            Some(x) => x.value(),
            None => get_random_element(mtypes.iter()).value()
        };

        get_random_element(MIME_TYPES.get(mtype).expect("Cant find mime_type!").iter())
    }

    pub fn size(minimum: i32, maximum: i32) -> String {
        format!("{} {}", randint(minimum, maximum), 
            get_random_element(vec!["bytes", "kB", "MB", "GB", "TB"].into_iter()))
    }

    pub fn file_name(file_type: Option<FileType>) -> String {
        let replacer = get_random_element(vec!["_", "-"].into_iter());
        let word = Text(Locale::EN).word().trim();
        
        let re = Regex::new(r"\s+").unwrap();
        let name = re.replace_all(word, replacer);
        let ext = Self::extension(file_type);

        format!("{}{}", name, ext)
    }
}
