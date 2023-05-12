use std::env;
use std::fs::read_to_string;

use serde_json::from_str;

use hh_rust::vacancies::*;

#[test]
fn vacancy_deserialize() {
    let out_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let vacancy_file = read_to_string(format!("{out_dir}/data/vacancy.json")).unwrap();

    let vacancy_ser: Vacancy = from_str(&vacancy_file).unwrap();

    println!("{:?}", vacancy_ser);

    assert_eq!(1, 1);
}
