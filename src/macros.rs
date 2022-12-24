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

#[macro_export]
macro_rules! valued_enum {
    {
       $(#[$meta:meta])*
       $v:vis enum $name:ident ( $valtype:ty ) {
           $(
               $(#[$item_meta:meta])*
               $id:ident = $val:literal
           )*
           $(,)?
      }
    } => {
        #[derive(PartialEq, Eq)]
        $(#[$meta])*
        $v struct $name($valtype);

        impl From<&$name> for $valtype {
            fn from(val: &$name) -> $valtype { val.0 }
        }

        impl Into<$valtype> for $name {
            fn into(self) -> $valtype {
                self.value()
            }
        }

        impl $name {
            $(
                $(#[$item_meta])*
                pub const $id: $name = $name($val);
            )*

            fn to_str(&self) -> Option<&'static str> {
                match self {
                    $( &$name::$id => Some(stringify!($id)), )*
                    _ => None,
                }
            }

            pub fn key(&self) -> String {
                self.to_string()
            }

            pub fn value(&self) -> $valtype {
                self.into()
            }

            $v fn is_recognized(&self) -> bool {
                matches!(self, $( &$name::$id )|*)
            }

            $v fn from_key(key: &str) -> Option<Self> {
                match key {
                    $( stringify!($id) => Some($name::$id), )*
                    _ => None
                }
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self.to_str() {
                    Some(s) => write!(f, "{}", s),
                    None => write!(f, "{}", self.0),
                }
            }
        }
        
        impl std::fmt::Debug for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{} = {}", stringify!($name), self)
            }
        }
    };
}
