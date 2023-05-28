use hh_rust::vacancies::{get_all_vacancies, get_vacancy_id, VacanciesQuery};

use simple_logger::SimpleLogger;

pub fn init_logger() {
    let _ = SimpleLogger::new().with_utc_timestamps().init();
}

#[tokio::main]
async fn main() {
    init_logger();

    let all_v_query = Some(VacanciesQuery {
        page: Some(3),
        per_page: Some(10),
        ..Default::default()
    });

    if let Ok(all) = get_all_vacancies(all_v_query).await {
        for v in all.items {
            println!("Name of vacancy is {}={}", v.id.unwrap(), v.name.unwrap());
        }
    }

    if let Ok(v) = get_vacancy_id(80377829, None).await {
        println!("Name of specific vacancy is {}", v.name.unwrap());
    }
}
