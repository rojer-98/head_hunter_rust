use crate::{
    phone::{
        PhoneConfirmQuery, ResumeGeneratedCode, ResumePhone, ResumePhoneConfirmBody,
        ResumePhoneGenerateCodeBody, ResumeShouldSendSMSQuery,
    },
    request_and_convert,
    utils::{HError, QueryHandler, RequestError},
};

pub async fn resume_phone_confirm(
    access_token: Option<String>,
    query: PhoneConfirmQuery,
    body: ResumePhoneConfirmBody,
) -> Result<RequestError, HError> {
    request_and_convert!(
        url: "https://api.hh.ru/resume_phone_confirm",
        method: POST,
        access_token: access_token,
        query,
        RequestError,
        body
    )
}

pub async fn resume_should_send_sms(
    access_token: Option<String>,
    query: ResumeShouldSendSMSQuery,
) -> Result<ResumePhone, HError> {
    request_and_convert!(
        url: "https://api.hh.ru/resume_should_send_sms",
        method: GET,
        access_token: access_token,
        query,
        ResumePhone
    )
}

pub async fn resume_phone_generate_code(
    access_token: Option<String>,
    query: PhoneConfirmQuery,
    body: ResumePhoneGenerateCodeBody,
) -> Result<ResumeGeneratedCode, HError> {
    request_and_convert!(
        url: "https://api.hh.ru/resume_phone_generate_code",
        method: POST,
        access_token: access_token,
        query,
        ResumeGeneratedCode,
        body
    )
}
