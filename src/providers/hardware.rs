use super::dependencies::*;


/// Methods collection for generate data related to hardware
pub struct Hardware;

impl Hardware {
    /// Get a random screen resolution
    pub fn resolution() -> &'static str {
        get_random_element(RESOLUTIONS.iter())
    }

    /// Get a random size of screen in inch
    pub fn screen_size() -> &'static str {
        get_random_element(SCREEN_SIZES.iter())
    }

    /// Get a random CPU name
    pub fn cpu() -> &'static str {
        get_random_element(CPU.iter())
    }

    /// Get a random frequency of CPU
    pub fn cpu_frequency() -> String {
        format!("{:.1}GHz", uniform(1.5, 4.3))
    }

    /// Get a random generation
    pub fn generation() -> &'static str {
        get_random_element(GENERATION.iter())
    }

    /// Get a random CPU code name
    pub fn cpu_codename() -> &'static str {
        get_random_element(CPU_CODENAMES.iter())
    }
    
    /// Get a random RAM type
    pub fn ram_type() -> &'static str {
        get_random_element(RAM_TYPES.iter())
    }

    /// Get a random size of RAM
    pub fn ram_size() -> &'static str {
        get_random_element(RAM_SIZES.iter())
    }

    /// Get a random value from list
    pub fn ssd_or_hdd() -> &'static str {
        get_random_element(HDD_SSD.iter())
    }

    /// Get a random graphics
    pub fn graphics() -> &'static str {
        get_random_element(GRAPHICS.iter())
    }

    /// Get a random manufacturer
    pub fn manufacturer() -> &'static str {
        get_random_element(MANUFACTURERS.iter())
    }

    /// Get a random phone model
    pub fn phone_model() -> &'static str {
        get_random_element(PHONE_MODELS.iter())
    }
}
