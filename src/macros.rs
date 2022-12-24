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
