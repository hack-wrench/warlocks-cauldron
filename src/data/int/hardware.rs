use super::dependencies::*;


lazy_static! {
    pub static ref RESOLUTIONS: List = list! [
        "1152x768",
        "1280x854",
        "1440x960",
        "2880x1920",
        "1024x768",
        "1152x864",
        "1280x960",
        "1400x1050",
        "1600x1200",
        "2048x1536",
        "3200x2400",
        "1280x768",
        "1280x1024",
        "2560x2048",
        "1280x720",
        "1365x768",
        "1600x900",
        "1920x1080",
        "1280x800",
        "1440x900",
        "1680x1050",
        "1920x1200",
        "2560x1600",
        "3840x2400"
    ];
    
    pub static ref SCREEN_SIZES: List = list! [
        "14″",
        "12.1″",
        "12″",
        "14.4″",
        "15″",
        "15.7″",
        "13.3″",
        "13″",
        "17″",
        "15.4″",
        "14.1″",
        "16″",
        "27″",
        "29″",
        "34″",
        "32″",
        "40″"
    ];
    
    pub static ref CPU: List = list! [
        "AMD Ryzen 7 1800X",
        "AMD Ryzen 7 1700",
        "AMD Ryzen™ Threadripper™",
        "Intel® Core i3",
        "Intel® Core i5",
        "Intel® Core i7",
        "Intel® Core i9",
        "Apple M1",
        "Apple M1 Pro",
        "Apple M1 Max",
        "Apple M2"
    ];
    
    pub static ref RAM_TYPES: List = list! [
        "SDRAM",
        "DDR",
        "DDR2",
        "DDR3",
        "DDR4",
        "DDR5"
    ];
    
    pub static ref RAM_SIZES: List = list! [
        "4GB",
        "8GB",
        "12GB",
        "16GB",
        "32GB",
        "64GB",
        "128GB"
    ];

    pub static ref GENERATION: List = list! [
        "2nd Generation",
        "3rd Generation",
        "4th Generation",
        "5th Generation",
        "6th Generation",
        "7th Generation",
        "8th Generation",
        "9th Generation"
    ];
    
    pub static ref CPU_CODENAMES: List = list! [
        "Ivytown",
        "Haswell",
        "Fortville",
        "Devil's Canyon",
        "Valley Island",
        "Broadwell",
        "Bay Trail",
        "Skylake",
        "Orchid Island",
        "Bear Ridge",
        "Cannonlake"
    ];
    
    pub static ref HDD_SSD_MANUFACTURERS: List = list! [
        "Western Digital",
        "Seagate",
        "Samsung",
        "Intel",
        "Micron",
        "Kingston",
        "SanDisk"
    ];
    
    pub static ref CAPACITY: List = list! [
        "64GB SSD",
        "128GB SSD",
        "256GB SDD",
        "512GB SSD",
        "1TB SSD",
        "2TB SSD",
        "4TB SSD",
        "256GB HDD",
        "512GB HDD",
        "1TB HDD",
        "2TB HDD",
        "4TB HDD",
        "8TB HDD"
    ];
    
    pub static ref HDD_SSD: List = iproduct!(HDD_SSD_MANUFACTURERS.iter(), CAPACITY.iter())
        .map(|(m, c)| format!("{m} {c}")).collect();
    
    pub static ref GRAPHICS: List = list! [
        "AMD Radeon PRO WX 8200",
        "AMD Radeon Pro W5700",
        "AMD Radeon RX 5500 XT",
        "AMD Radeon RX 560",
        "AMD Radeon RX 5600 XT",
        "AMD Radeon RX 570",
        "AMD Radeon RX 5700",
        "AMD Radeon RX 5700 XT",
        "AMD Radeon RX 580",
        "AMD Radeon RX 590",
        "AMD Radeon RX 6500 XT",
        "AMD Radeon RX 6600 XT",
        "AMD Radeon RX 6700 XT",
        "AMD Radeon RX 6750 XT",
        "AMD Radeon RX 6800",
        "AMD Radeon RX 6800 XT",
        "AMD Radeon RX 6900 XT",
        "AMD Radeon RX 6950 XT",
        "AMD Radeon RX Vega 56",
        "AMD Radeon RX Vega 64",
        "AMD Radeon VII",
        "Intel® HD Graphics 3000",
        "Intel® HD Graphics 4000",
        "Intel® HD Graphics 4400",
        "Intel® HD Graphics 4600",
        "Intel® HD Graphics 5000",
        "Intel® HD Graphics 520",
        "Intel® HD Graphics 5300 ",
        "Intel® HD Graphics 5500",
        "Intel® HD Graphics 6000",
        "Intel® HD Graphics 615",
        "Intel® HD Graphics 620",
        "Intel® Iris™ Graphics 5100",
        "Intel® Iris™ Graphics 550",
        "Intel® Iris™ Graphics 6100",
        "Intel® Iris™ Pro Graphics 5200",
        "Intel® Iris™ Pro Graphics 580",
        "Intel® Iris™ Pro Graphics 6200",
        "Nvidia GTX 1050",
        "Nvidia GTX 1050 Ti",
        "Nvidia GTX 1060",
        "Nvidia GTX 1070",
        "Nvidia GTX 1070 Ti",
        "Nvidia GTX 1080",
        "Nvidia GTX 1080 Ti",
        "Nvidia GTX 1650",
        "Nvidia GTX 1650 SUPER",
        "Nvidia GTX 1660",
        "Nvidia GTX 1660 SUPER",
        "Nvidia GTX 1660 Ti",
        "Nvidia GTX 960",
        "Nvidia GTX 980",
        "Nvidia GTX 980 Ti",
        "Nvidia Quadro RTX A4000",
        "Nvidia Quadro RTX A5000",
        "Nvidia Quadro RTX A6000",
        "Nvidia RTX 2060",
        "Nvidia RTX 2060 SUPER",
        "Nvidia RTX 2070",
        "Nvidia RTX 2070 SUPER",
        "Nvidia RTX 2080",
        "Nvidia RTX 2080 SUPER",
        "Nvidia RTX 2080 Ti",
        "Nvidia RTX 3050",
        "Nvidia RTX 3050",
        "Nvidia RTX 3050 Ti",
        "Nvidia RTX 3060",
        "Nvidia RTX 3060",
        "Nvidia RTX 3060 Ti",
        "Nvidia RTX 3070",
        "Nvidia RTX 3070",
        "Nvidia RTX 3070 Ti",
        "Nvidia RTX 3080",
        "Nvidia RTX 3080",
        "Nvidia RTX 3080 Ti",
        "Nvidia RTX 3090",
        "Nvidia RTX 3090 Ti",
        "Nvidia RTX 4090",
        "Nvidia RTX Titan",
        "Nvidia Titan Pascal",
        "Nvidia Titan V"
    ];
    
    pub static ref MANUFACTURERS: List = list! [
        "Apple",
        "Acer",
        "Dell",
        "ASUS",
        "VAIO",
        "Lenovo",
        "HP",
        "Toshiba",
        "Sony",
        "Samsung",
        "Fujitsu",
        "Xiomi"
    ];
    
    pub static ref PHONE_MODELS: List = list! [
        "iPhone SE",
        "iPhone X",
        "iPhone XS",
        "iPhone 11",
        "iPhone 11 Pro",
        "iPhone 11 Pro Max",
        "iPhone 12",
        "iPhone 12 Pro",
        "iPhone 12 Pro Max",
        "iPhone 13",
        "iPhone 13 Pro",
        "iPhone 13 Pro Max",
        "iPhone 14",
        "iPhone 14 Plus",
        "iPhone 14 Pro",
        "iPhone 14 Pro Max",
        "Nothing Phone",
        "Samsung Galaxy S22 Ultra",
        "Samsung Galaxy S22 Plus",
        "Samsung Galaxy Fold 4",
        "Samsung Galaxy Z Flip 4",
        "Xiaomi Redmi Note 11",
        "Xiaomi 12 Pro",
        "Google Pixel 6",
        "Google Pixel 6 Pro",
        "Google Pixel 7",
        "Google Pixel 7 Pro",
        "Vivo X80 Pro",
        "OnePlus 10 Pro"
    ];
}
