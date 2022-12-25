use super::dependencies::*;


#[cfg(feature = "path")]
lazy_static! {
    pub static ref PLATFORMS: RecDict = dict! {
        "linux" => dict! {
            "home" => "/home/"
        },
        "darwin" => dict! {
            "home" => "/home/"
        },
        "win32" => dict! {
            "home" => "C:\\Users\\"
        },
        "win64" => dict! {
            "home" => "C:\\Users\\"
        }
    };
}
