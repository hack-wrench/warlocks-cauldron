use std::iter::zip;

use super::super::dependencies::*;


pub struct BrazilSpecProvider;

impl BrazilSpecProvider {
    /// Calculate the verifying digit for the CPF
    fn get_verifying_digit_cpf(cpf: &Vec<u32>, peso: u32) -> u32 {
        let mut soma = 0;
        for (index, digit) in cpf.iter().enumerate() {
            soma += digit * (peso - index as u32);
        }

        let resto = soma % 11;

        if resto == 0 || resto == 1 || resto >= 11 {
            0
        } else {
            11 - resto
        }
    }

    /// Get a random CPF
    pub fn cpf(with_mask: bool) -> String {
        let mut cpf_without_dv = randints(0, 9, 9);
        let first_dv = Self::get_verifying_digit_cpf(&cpf_without_dv, 10);

        cpf_without_dv.push(first_dv);
        let second_dv = Self::get_verifying_digit_cpf(&cpf_without_dv, 11);
        cpf_without_dv.push(second_dv);

        let cpf = cpf_without_dv.into_iter().join("");

        match with_mask {
            true => format!("{}.{}.{}-{}", &cpf[..3], &cpf[3..6], &cpf[6..9], &cpf[9..]),
            false => cpf,
        }
    }

    /// Calculate the verifying digit for the CNPJ
    fn get_verifying_digit_cnpj(cnpj: &Vec<u32>, peso: u32) -> u32 {
        let peso_list = match peso {
            5 => vec![5, 4, 3, 2, 9, 8, 7, 6, 5, 4, 3, 2],
            6 => vec![6, 5, 4, 3, 2, 9, 8, 7, 6, 5, 4, 3, 2],
            _ => panic!("Invalid peso!"),
        };

        let soma: u32 = zip(cnpj, peso_list).map(|(c, p)| c * p).sum();

        let resto = soma % 11;

        if resto < 2 {
            0
        } else {
            11 - resto
        }
    }

    /// Get a random CNPJ
    pub fn cnpj(with_mask: bool) -> String {
        let mut cnpj_without_dv = randints(0, 9, 12);

        let first_dv = Self::get_verifying_digit_cnpj(&cnpj_without_dv, 5);
        cnpj_without_dv.push(first_dv);

        let second_dv = Self::get_verifying_digit_cnpj(&cnpj_without_dv, 6);
        cnpj_without_dv.push(second_dv);

        let cnpj = cnpj_without_dv.into_iter().join("");

        match with_mask {
            true => format!("{}.{}.{}/{}-{}", &cnpj[..2], &cnpj[2..5], &cnpj[5..8], &cnpj[8..12], &cnpj[12..]),
            false => cnpj,
        }
    }
}
