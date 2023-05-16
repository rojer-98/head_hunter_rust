use hh_rust::vacancies::{get_all_vacancies, get_vacancy_id, VacanciesQuery};

#[tokio::main]
async fn main() {
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
