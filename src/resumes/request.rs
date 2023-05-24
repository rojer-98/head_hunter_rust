use crate::{
    request_and_convert,
    resumes::{ResumeMine, ResumeQuery, ResumeStatus, ResumeViews},
    utils::{HError, QueryHandler},
};

pub async fn get_resume_status(
    access_token: Option<String>,
    resume_id: usize,
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

pub async fn get_resumes_views(
    resume_id: usize,
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
