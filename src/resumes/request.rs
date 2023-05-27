use std::fmt::Display;

use crate::{
    dictionary::DefaultQuery,
    employers::EmployerVisibility,
    request_and_convert,
    resumes::{ResumeConditions, ResumeMine, ResumeStatus, ResumeViews},
    utils::{HError, QueryHandler},
    vacancies::{Vacancies, VacanciesQuery},
};

pub async fn get_resume_status<T: Display>(
    access_token: Option<String>,
    resume_id: T,
    query: Option<DefaultQuery>,
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
    query: Option<DefaultQuery>,
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
    query: Option<DefaultQuery>,
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

pub async fn get_new_resume_conditions(
    access_token: Option<String>,
    query: Option<DefaultQuery>,
) -> Result<ResumeConditions, HError> {
    request_and_convert!(
        url: format!("https://api.hh.ru/resume_conditions"),
        method: GET,
        access_token: access_token,
        optional query,
        ResumeConditions
    )
}

pub async fn get_resume_conditions<T: Display>(
    access_token: Option<String>,
    resume_id: T,
    query: Option<DefaultQuery>,
) -> Result<ResumeConditions, HError> {
    request_and_convert!(
        url: format!("https://api.hh.ru/resumes/{resume_id}/conditions"),
        method: GET,
        access_token: access_token,
        optional query,
        ResumeConditions
    )
}

pub async fn get_resume_visibility_list<T: Display>(
    access_token: Option<String>,
    resume_id: T,
    list_type: T,
    query: Option<DefaultQuery>,
) -> Result<EmployerVisibility, HError> {
    request_and_convert!(
        url: format!("https://api.hh.ru/resumes/{resume_id}/{list_type}"),
        method: GET,
        access_token: access_token,
        optional query,
        EmployerVisibility
    )
}
