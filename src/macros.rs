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
    ($lang_code:literal) => {(
        $lang_code,
        include_str!(concat!("jsons/", $lang_code, "/address.json")),
        include_str!(concat!("jsons/", $lang_code, "/datetime.json")),
        include_str!(concat!("jsons/", $lang_code, "/finance.json")),
        include_str!(concat!("jsons/", $lang_code, "/food.json")),
        include_str!(concat!("jsons/", $lang_code, "/person.json")),
        include_str!(concat!("jsons/", $lang_code, "/text.json")),
        None,
    )};

    ($lang_code:literal, with_builtin) => {(
        $lang_code,
        include_str!(concat!("jsons/", $lang_code, "/address.json")),
        include_str!(concat!("jsons/", $lang_code, "/datetime.json")),
        include_str!(concat!("jsons/", $lang_code, "/finance.json")),
        include_str!(concat!("jsons/", $lang_code, "/food.json")),
        include_str!(concat!("jsons/", $lang_code, "/person.json")),
        include_str!(concat!("jsons/", $lang_code, "/text.json")),
        Some(include_str!(concat!("jsons/", $lang_code, "/builtin.json"))),
    )};
);
