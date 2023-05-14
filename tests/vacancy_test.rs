use std::env;
use std::fs::read_to_string;

use log::info;
use serde_json::from_str;
use tokio::runtime::Builder;

use hh_rust::vacancies::*;

mod common;
use common::init_logger;

#[test]
fn test_vacancy_deserialize() {
    init_logger();

    let out_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let vacancy_file = read_to_string(format!("{out_dir}/data/vacancy.json")).unwrap();

    let vacancy_ser: Vacancy = from_str(&vacancy_file).unwrap();

    info!("Vacancy deserialize is {:?}", vacancy_ser);

    assert_eq!(1, 1);
}

#[test]
fn test_get_all_vacancies() {
    init_logger();

    let all_v = Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async { get_all_vacancies().await })
        .unwrap();

    info!("Get all vacancies is {:?}", all_v);

    assert_eq!(1, 1);
}

#[test]
fn test_get_vacancy_id() {
    init_logger();

    let v_id = Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async { get_vacancy_id(79725422).await })
        .unwrap();

    info!("Get vacancy id is {:?}", v_id);

    assert_eq!(1, 1);
}


#[test]
fn test_get_vacancy_id_visitors() {
    init_logger();

    let v_id = Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async { get_vacancy_id_visitors(79725422).await })
        .unwrap();

    info!("Get vacancy id visitors is {:?}", v_id);

    assert_eq!(1, 1);
}
