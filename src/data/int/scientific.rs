use super::dependencies::*;


lazy_static! {
    pub static ref SI_PREFIXES: ListDict = dict! {
        "negative" => list! [
            "deci",
            "centi",
            "milli",
            "micro",
            "nano",
            "pico",
            "femto",
            "atto",
            "zepto",
            "yocto"
        ],
        "positive" => list! [
            "yotta",
            "zetta",
            "exa",
            "peta",
            "tera",
            "giga",
            "mega",
            "kilo",
            "hecto",
            "deca"
        ]
    };
    
    pub static ref SI_PREFIXES_SYM: ListDict = dict! {
        "negative" => list! ["d", "c", "m", "μ", "n", "p", "f", "a", "z", "y"],
        "positive" => list! ["Y", "Z", "E", "P", "T", "G", "M", "k", "h", "da"]
    };
}
