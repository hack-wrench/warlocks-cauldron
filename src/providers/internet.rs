use chrono::Datelike;
pub use ipaddress::*;

use super::dependencies::*;
use super::{Datetime, File, Text};


#[derive(Debug)]
pub enum StockType {
    URL(String),
    Image(Vec<u8>),
}

/// Methods collection for generating data related to the internet
pub struct Internet;

impl Internet {
    /// Get a random HTTP content type
    ///
    /// return example: Content-Type: application/json
    ///
    /// # Arguments
    /// * `mime_type` - MimeType enum
    pub fn content_type(mime_type: Option<MimeType>) -> String {
        format!("Content-Type: {}", File::mime_type(mime_type))
    }

    /// Get a random HTTP status message
    ///
    /// return example: 200 OK
    pub fn http_status_message() -> &'static str {
        get_random_element(HTTP_STATUS_MSGS.iter())
    }

    /// Get a random HTTP status message
    ///
    /// return example: 200
    pub fn http_status_code() -> &'static str {
        get_random_element(HTTP_STATUS_CODES.iter())
    }

    /// Get a random HTTP status message
    ///
    /// return example: POST
    pub fn http_method() -> &'static str {
        get_random_element(HTTP_METHODS.iter())
    }

    /// Generate random port
    /// 
    /// return example: 8000
    ///
    /// # Arguments
    /// * `port_range` - PortRange enum
    pub fn port(port_range: Option<PortRange>) -> u16 {
        let range: (u16, u16) = match port_range {
            Some(x) => x.into(),
            None => PortRange::ALL.into(),
        };

        randint(range.0, range.1)
    }

    /// Get a random v4 IPAddress struct
    ///
    /// return example: IPAddress
    pub fn ip_v4_object() -> IPAddress {
        ipv4::from_u32(randint(0, 4294967295), 32).expect("Cant create v4 IPAddress!")
    }

    /// Get a random v4 IP address
    ///
    /// return example: 127.0.0.1
    pub fn ip_v4() -> String {
        Self::ip_v4_object().to_s()
    }

    /// Get a random v4 IP address with port
    ///
    /// return example: 127.0.0.1:666
    ///
    /// # Arguments
    /// * `port_range` - PortRange enum
    pub fn ip_v4_with_port(port_range: Option<PortRange>) -> String {
        format!("{}:{}", Self::ip_v4(), Self::port(port_range))
    }

    /// Get a random v6 IPAddress struct
    ///
    /// return example: IPAddress
    pub fn ip_v6_object() -> IPAddress {
        ipv6::from_int(randbigint(u128::MIN, u128::MAX), 32).expect("Cant create v6 IPAddress!")
    }

    /// Get a random v6 IP address
    ///
    /// return example: 0000:0000:0000:0000:0000:0000:0000:0001
    pub fn ip_v6() -> String {
        Self::ip_v6_object().to_s()
    }

    /// Get a random mac address
    ///
    /// return example: 00:16:3e:25:e7:f1
    pub fn mac() -> String {
        vec![
            0x00,
            0x16,
            0x3E,
            randint(0x00, 0x7F),
            randint(0x00, 0xFF),
            randint(0x00, 0xFF),
        ].iter().map(|i| format!("{:02x}", i))
            .join(":")
    }

    /// Get a random emoji shortcut code
    ///
    /// return example: :smirk:
    pub fn emoji() -> &'static str {
        get_random_element(EMOJI.iter())
    }

    /// Generate random stock image (JPG/JPEG) hosted on Unsplash
    ///
    /// return example: StockType::URL("https://source.unsplash.com/666x666?test")
    ///
    /// # Arguments
    /// * `width` - Width of the image
    /// * `height` - Height of the image
    /// * `keywords` - List of search keywords
    /// * `to_image` - Return image as vec of u8
    pub fn stock_image(width: u32, height: u32, keywords: Vec<&str>, to_image: bool) -> StockType {
        let keywords_str = keywords.join(",");
        let url = format!("https://source.unsplash.com/{width}x{height}?{keywords_str}");

        match to_image {
            true => {
                let r = reqwest::blocking::get(url).expect("Cant fetch images from unsplash!")
                    .bytes().expect("Cant get output!");
                
                StockType::Image(r.to_vec())
            },
            false => StockType::URL(url),
        }
    }

    /// Generate a list of hashtags
    ///
    /// return example: vec!\['#some', '#random', '#things'\]
    ///
    /// # Arguments
    /// * `quantity` - The quantity of hashtags
    pub fn hashtags(quantity: i32) -> Vec<String> {
        let locale = Text(Locale::EN);
        (0..quantity).map(|_| format!("#{}", locale.word())).collect()
    }

    /// Generates random top level domain
    ///
    /// return example: com
    ///
    /// # Arguments
    /// * `tld_type` - TLDType provide hostname domain
    pub fn top_level_domain(tld_type: Option<TLDType>) -> &'static str {
        let tld = match tld_type {
            Some(x) => x.value(),
            None => get_random_element(TLDType::variants().iter()).value(),
        };

        get_random_element(TLD.get(tld).expect("Cant get TLD type!").iter())
    }

    /// Generates random top level domain | *An allias for .top_level_domain()*
    ///
    /// return example: com
    ///
    /// # Arguments
    /// * `tld_type` - TLDType provide hostname domain
    pub fn tld(tld_type: Option<TLDType>) -> &'static str {
        Self::top_level_domain(tld_type)
    }

    /// Get a random user agent
    ///
    /// return example: Gecko/20100101 Firefox/15.0.1
    pub fn user_agent() -> &'static str {
        get_random_element(USER_AGENTS.iter())
    }

    /// Get a random user agent
    ///
    /// return example: 1.1.1.1
    pub fn public_dns() -> &'static str {
        get_random_element(PUBLIC_DNS.iter())
    }

    /// Generate a random slug of given parts count
    /// 
    /// return example: some-slug-here
    pub fn slug(parts_count: Option<usize>) -> String {
        let parts = match parts_count {
            Some(p) => p,
            None => randint(2, 12),
        };

        if parts > 12 {
            panic!("Slug's parts count must be <= 12");
        }

        if parts < 2 {
            panic!("Slug must contain more than 2 parts");
        }

        Text(Locale::EN).words(parts).iter().join("-")
    }

    /// Generate a random hostname without scheme
    ///
    /// return example: sub.domain
    ///
    /// # Arguments
    /// * `tld_type` - TLDType provide hostname domain
    /// * `subdomains` - vec of subdomains
    pub fn hostname(tld_type: Option<TLDType>, subdomains: Option<Vec<&str>>) -> String {
        let tld = Self::top_level_domain(tld_type);
        let host = get_random_element(USERNAMES.iter());

        match subdomains {
            None => format!("{host}{tld}"),
            Some(v) => format!("{}.{host}", get_random_element(v.iter())),
        }
    }

    /// Generate random URL
    ///
    /// return example: https://sub.domain.com:8000/
    ///
    /// # Arguments
    /// * `scheme` - URLScheme provide url scheme
    /// * `port_range` - PortRange enum
    /// * `tld_type` - TLDType provide hostname domain
    /// * `subdomains` - vec of subdomains
    pub fn url(scheme: Option<URLScheme>, port_range: Option<PortRange>, tld_type: Option<TLDType>, subdomains: Option<Vec<&str>>) -> String {
        let hostname = Self::hostname(tld_type, subdomains);

        let scheme = match scheme {
            Some(x) => x.value(),
            None => get_random_element(URLScheme::variants().iter()).value(),
        };

        let mut url = format!("{scheme}://{hostname}");

        if port_range.is_some() {
            url = format!("{url}:{}", Self::port(port_range));
        }

        format!("{url}/")
    }

    /// Generate a random URI
    ///
    /// return example: https://sub.domain.com:8000/2013/6/6/?some-things&test-test
    ///
    /// # Arguments
    /// * `scheme` - URLScheme for url prefix
    /// * `port_range` - PortRange enum for port
    /// * `tld_type` - TLDType provide hostname domain
    /// * `subdomains` - vec of subdomains
    /// * `query_params_count` - Query params count
    pub fn uri(scheme: Option<URLScheme>, port_range: Option<PortRange>, tld_type: Option<TLDType>, subdomains: Option<Vec<&str>>, query_params_count: Option<usize>) -> String {
        let directory = Datetime::date(2010, chrono::Local::now().year()).format("%Y/%m/%d");

        let mut url = format!("{}{directory}/{}", Self::url(scheme, port_range, tld_type, subdomains), Self::slug(None));

        if query_params_count.is_some() {
            url = format!("{url}?{}", Self::query_string(query_params_count));
        }

        url
    }

    /// Generate arbitrary query string of given length
    ///
    /// return example: some-things&test-test
    ///
    /// # Arguments
    /// * `length` - Query params count
    pub fn query_string(length: Option<usize>) -> String {
        let formated = Self::query_parameters(length).iter().map(|p| {
            format!("{}={}", p.0, p.1)
        }).join("&");
    
        urlencoding::encode(&formated).into_owned()
    }

    /// Generate arbitrary query parameters as a set
    ///
    /// return example: vec![('some', 'things'), ('test', 'test')]
    ///
    /// # Arguments
    /// * `length` - Query params count
    pub fn query_parameters(length: Option<usize>) -> Vec<(&'static str, &'static String)> {
        let text = &Text(Locale::RU);
        let length = match length {
            Some(len) => len,
            None => randint(1, 10),
        };

        if length > 32 {
            panic!("Length should be less than 32!")
        }

        let mut unique_words: Vec<&'static str> = vec![];
        while unique_words.len() < length {
            let word = text.word();
            if !unique_words.contains(&word) {
                unique_words.push(word)
            }
        }

        unique_words.into_iter().zip(text.words(length)).collect()
    }
}
