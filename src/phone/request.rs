use crate::{
    phone::{
        PhoneConfirmQuery, ResumeGeneratedCode, ResumePhone, ResumePhoneConfirmBody,
        ResumePhoneGenerateCodeBody, ResumeShouldSendSMSQuery,
    },
    request_and_convert,
    utils::{HError, QueryHandler, RequestError},
};

pub async fn resume_phone_confirm(
    query: PhoneConfirmQuery,
    body: ResumePhoneConfirmBody,
) -> Result<RequestError, HError> {
    request_and_convert!(
        url: "https://api.hh.ru/resume_phone_confirm",
        method: POST,
        query,
        RequestError,
        body
    )
}

pub async fn resume_should_send_sms(
    query: ResumeShouldSendSMSQuery,
) -> Result<ResumePhone, HError> {
    request_and_convert!(
        url: "https://api.hh.ru/resume_should_send_sms",
        method: GET,
        query,
        ResumePhone
    )
}

pub async fn resume_phone_generate_code(
    query: PhoneConfirmQuery,
    body: ResumePhoneGenerateCodeBody,
) -> Result<ResumeGeneratedCode, HError> {
    request_and_convert!(
        url: "https://api.hh.ru/resume_phone_generate_code",
        method: POST,
        query,
        ResumeGeneratedCode,
        body
    )
}
