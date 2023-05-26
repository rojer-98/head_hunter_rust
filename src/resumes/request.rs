use std::fmt::Display;

use crate::{
    request_and_convert,
    resumes::{ResumeMine, ResumeQuery, ResumeStatus, ResumeViews, VisibilityResumeQuery},
    utils::{HError, QueryHandler},
    vacancies::{Vacancies, VacanciesQuery},
};

pub async fn get_resume_status<T: Display>(
    access_token: Option<String>,
    resume_id: T,
    query: Option<ResumeQuery>,
) -> Result<ResumeStatus, HError> {
    request_and_convert!(
        url: format!("https://api.hh.ru/resumes/{resume_id}/status"),
        method: GET,
        access_token: access_token,
        optional query,
        ResumeStatus
    )
}

pub async fn get_mine_resumes(
    access_token: Option<String>,
    query: Option<ResumeQuery>,
) -> Result<ResumeMine, HError> {
    request_and_convert!(
        url: format!("https://api.hh.ru/resumes/mine"),
        method: GET,
        access_token: access_token,
        optional query,
        ResumeMine
    )
}

pub async fn get_resumes_views<T: Display>(
    resume_id: T,
    access_token: Option<String>,
    query: Option<ResumeQuery>,
) -> Result<ResumeViews, HError> {
    request_and_convert!(
        url: format!("https://api.hh.ru/resumes/{resume_id}/views"),
        method: GET,
        access_token: access_token,
        optional query,
        ResumeViews
    )
}

pub async fn get_similar_vacancy_by_resume_id<T: Display>(
    access_token: Option<String>,
    resume_id: T,
    query: Option<VacanciesQuery>,
) -> Result<Vacancies, HError> {
    request_and_convert!(
        url: format!("https://api.hh.ru/resumes/{resume_id}/similar_vacancies"),
        method: GET,
        access_token: access_token,
        optional query,
        Vacancies
    )
}

/*
pub async fn get_resume_visibility_list<T: Display>(
    access_token: Option<String>,
    resume_id: T,
    list_type: T,
    query: VisibilityResumeQuery,
) -> Result<Vacancies, HError> {
    request_and_convert!(
        url: format!("https://api.hh.ru/resumes/{resume_id}/{list_type}/search"),
        method: GET,
        access_token: access_token,
        query,
        Vacancies
    )
}
*/
