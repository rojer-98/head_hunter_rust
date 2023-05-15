use hh_rust::vacancies::{get_all_vacancies, get_vacancy_id};

#[tokio::main]
async fn main() {
    if let Ok(all) = get_all_vacancies().await {
        for v in all.items {
            println!("Name of vacancy is {:?}", v.name);
        }
    }

    if let Ok(v) = get_vacancy_id(80377829).await {
        println!("Name of specific vacancy is {:?}", v.name);
    }
}
