use core::ops::Deref;

#[macro_export]
macro_rules! dict(
    { $($key:expr => $value:expr), + } => {
        {
            let mut m = HashMap::new();
            $(
                m.insert($key, $value);
            )+
            m
        }
     };
);

#[macro_export]
macro_rules! list(
    { $($value:expr), + } => {
        {
            let mut v = Vec::new();
            $(
                v.push($value.to_string());
            )+
            
            v
        }
     };
);


#[macro_export]
macro_rules! generate_payload(
    { $($lang_code:expr) + } => {
        ($(
            $lang_code,
            include_str!(concat!("jsons/", $lang_code, "/address.json")),
            include_str!(concat!("jsons/", $lang_code, "/datetime.json")),
            include_str!(concat!("jsons/", $lang_code, "/finance.json")),
            include_str!(concat!("jsons/", $lang_code, "/food.json")),
            include_str!(concat!("jsons/", $lang_code, "/person.json")),
            include_str!(concat!("jsons/", $lang_code, "/text.json")),
        )*)
     };
);

pub trait ValuedEnum<T> where Self: Sized {
    fn key(&self) -> &str;
    fn value(&self) -> T;

    fn key_variants() -> Vec<&'static str>;
    fn variants() -> Vec<Self>;

    fn from_key(key: &str) -> Option<Self>;
}

#[macro_export]
macro_rules! valued_enum {
    {
       $(#[$meta:meta])*
       $v:vis $name:ident ( $valtype:ty ):
           $(
               $(#[$item_meta:meta])*
               $id:ident = $val:expr
           )*
           $(,)?
    } => {
        #[derive(PartialEq, Eq)]
        $(#[$meta])*
        $v struct $name(&'static str, $valtype);

        impl $name {
            $(
                $(#[$item_meta])*
                pub const $id: $name = $name(stringify!($id), $val);
            )*
        }

        impl crate::macros::ValuedEnum<$valtype> for $name {
            fn key(&self) -> &str {
                self.0
            }

            fn value(&self) -> $valtype {
                self.1
            }

            fn key_variants() -> Vec<&'static str> {
                vec![
                    $( stringify!($id), )*
                ]
            }

            fn variants() -> Vec<$name> {
                vec![
                    $( $name::$id, )*
                ]
            }

            fn from_key(key: &str) -> Option<Self> {
                match key {
                    $( stringify!($id) => Some($name::$id), )*
                    _ => None
                }
            }
        }
    };
}
