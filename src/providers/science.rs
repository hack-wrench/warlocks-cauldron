use super::dependencies::*;


/// Methods collection for generating pseudo-scientific data
pub struct Science;

impl Science {
    /// Generate a random RNA sequence
    /// 
    /// Arguments:
    /// * `length` - Length of block
    pub fn rna_sequence(length: usize) -> String {
        generate_string("UCGA", length)
    }

    /// Generate a random DNA sequence
    /// 
    /// Arguments:
    /// * `length` - Length of block
    pub fn dna_sequence(length: usize) -> String {
        generate_string("TCGA", length)
    }

    /// Get unit name from International System of Units
    /// 
    /// ex: ("gram", "gr")
    pub fn measure_unit() -> (&'static str, &'static str) {
        get_random_element(MeasureUnit::values().into_iter())
    }

    /// Get a random prefix for the International System of Units
    /// 
    /// ex: nano
    /// 
    /// Arguments:
    /// * `sign` - Sing of prefix (positive/negative) or None for random
    /// * `symbol` - Return the symbol of the prefix
    pub fn metric_prefix(sign: Option<MetricPrefixSign>, symbol: bool) -> &'static str {
        let key = validate_enum(sign, None);
        match symbol {
            true => get_random_element(SI_PREFIXES_SYM.get(key).expect("Cant find SI_PREFIX_SYM!").iter()),
            false => get_random_element(SI_PREFIXES.get(key).expect("Cant find SI_PREFIX!").iter()),
        }
    }
}
