use std::env;
use std::fs::read_to_string;

use log::info;
use serde_json::from_str;
use tokio::runtime::Builder;

use hh_rust::vacancies::*;

#[test]
fn test_vacancy_deserialize() {
    let out_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let vacancy_file = read_to_string(format!("{out_dir}/data/vacancy.json")).unwrap();

    let vacancy_ser: Vacancy = from_str(&vacancy_file).unwrap();

    info!("{:?}", vacancy_ser);

    assert_eq!(1, 1);
}

#[test]
fn test_get_all_vacancies() {
    let all_v = Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async { get_all_vacancies().await }).unwrap();

    info!("{:?}", all_v);

    assert_eq!(1, 1);
}
