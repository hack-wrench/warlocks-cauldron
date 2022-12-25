use crate::data::parsed::TEST;
use crate::data::serializers::{NationalityOption, SurnamesOption};


#[test]
fn serialize_data() {
    assert_eq!(TEST.lang_code, "locale_template");

    let adr = &TEST.address;
    assert_eq!(adr.city, vec!["Test"]);
    assert_eq!(adr.continent, vec!["Test"]);
    assert_eq!(adr.country.current_locale, "Test");
    assert_eq!(adr.country.name, vec!["Test"]);
    assert_eq!(adr.postal_code_fmt, "#####");
    assert_eq!(adr.state.abbr, vec!["Test"]);
    assert_eq!(adr.state.name, vec!["Test"]);
    assert_eq!(adr.street.name, vec!["Test"]);
    assert_eq!(adr.street.suffix, vec!["Test"]);

    let dtm = &TEST.datetime;
    assert_eq!(dtm.day.abbr, vec!["Test"]);
    assert_eq!(dtm.day.name, vec!["Test"]);
    assert_eq!(dtm.formats.date, "%m/%d/%Y");
    assert_eq!(dtm.formats.time, "%H:%M:%S");
    assert_eq!(dtm.month.abbr, vec!["Test"]);
    assert_eq!(dtm.month.name, vec!["Test"]);
    assert_eq!(dtm.periodicity, vec!["Test"]);

    let fin = &TEST.finance;
    assert_eq!(fin.company.name, vec!["Test"]);
    assert_eq!(fin.company._type.abbr, vec!["Test"]);
    assert_eq!(fin.company._type.title, vec!["Test"]);
    assert_eq!(fin.currency_code, "Test");
    assert_eq!(fin.price_format, Some("# Test".to_string()));
    assert_eq!(fin.numeric_decimal, Some(".".to_string()));
    assert_eq!(fin.numeric_thousands, Some(",".to_string()));
    assert_eq!(fin.numeric_frac_digits, Some(2));

    let food = &TEST.food;
    assert_eq!(food.dishes, vec!["Test"]);
    assert_eq!(food.drinks, vec!["Test"]);
    assert_eq!(food.fruits, vec!["Test"]);
    assert_eq!(food.spices, vec!["Test"]);
    assert_eq!(food.vegetables, vec!["Test"]);

    let prsn = &TEST.person;
    assert_eq!(prsn.academic_degree, vec!["Test"]);
    assert_eq!(prsn.gender, vec!["Test"]);
    assert_eq!(prsn.language, vec!["Test"]);
    assert_eq!(prsn.names.female, vec!["Test"]);
    assert_eq!(prsn.names.male, vec!["Test"]);

    assert_eq!(prsn.get_nationality(), NationalityOption::Sequence(vec!["Test".to_string()]));

    assert_eq!(prsn.occupation, vec!["Test"]);
    assert_eq!(prsn.political_views, vec!["Test"]);

    assert_eq!(prsn.get_surnames(), SurnamesOption::Sequence(vec!["Test".to_string()]));

    assert_eq!(prsn.title.female.typical, vec!["Test"]);
    assert_eq!(prsn.title.female.academic, vec!["Test"]);
    assert_eq!(prsn.title.male.typical, vec!["Test"]);
    assert_eq!(prsn.title.male.academic, vec!["Test"]);
    assert_eq!(prsn.university, vec!["Test"]);
    assert_eq!(prsn.views_on, vec!["Test"]);
    assert_eq!(prsn.worldview, vec!["Test"]);
    assert_eq!(prsn.telephone_fmt, vec!["###-###-####", "(###) ###-####", "1-###-###-####"]);

    let txt = &TEST.text;
    assert_eq!(txt.alphabet.uppercase, vec!["Test"]);
    assert_eq!(txt.alphabet.lowercase, vec!["Test"]);
    assert_eq!(txt.answers, vec!["Yes", "No", "Maybe"]);
    assert_eq!(txt.color, vec!["Test"]);
    assert_eq!(txt.level, vec!["low", "moderate", "high", "very high", "extreme", "critical"]);
    assert_eq!(txt.quotes, vec!["Test"]);
    assert_eq!(txt.text, vec!["Test"]);
    assert_eq!(txt.words.bad, vec!["Test"]);
    assert_eq!(txt.words.normal, vec!["Test"]);
}
