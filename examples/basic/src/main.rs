use hh_rust::{
    auth::take_auth,
    vacancies::{get_all_vacancies, get_vacancy_id, VacanciesQuery},
};

use simple_logger::SimpleLogger;

pub fn init_logger() {
    let _ = SimpleLogger::new().with_utc_timestamps().init();
}

#[tokio::main]
async fn main() {
    init_logger();

    let auth = take_auth(None).await;
    println!("Auth is {:?}", auth);
    let all_v_query = VacanciesQuery {
        page: Some(5),
        per_page: Some(20),
        ..Default::default()
    };

    if let Ok(all) = get_all_vacancies(Some(all_v_query)).await {
        for v in all.items {
            println!("Name of vacancy is {}={}", v.id.unwrap(), v.name.unwrap());
        }
    }

    if let Ok(v) = get_vacancy_id(80377829, None).await {
        println!("Name of specific vacancy is {}", v.name.unwrap());
    }
}
