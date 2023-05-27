use serde::{Deserialize, Serialize};

use crate::utils::RequestError;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResumeConditions {
    pub access: Option<ResumeConditionAccess>,
    pub area: Option<ResumeConditionArea>,
    pub birth_date: Option<BirthDate>,
    pub business_trip_readiness: Option<ResumeConditionArea>,
    pub citizenship: Box<Option<ResumeConditionCitizenship>>,
    pub contact: Box<Option<ResumeConditionCitizenship>>,
    pub district: Option<ResumeConditionArea>,
    pub driver_license_types: Box<Option<ResumeConditionCitizenship>>,
    pub education: Option<ResumeConditionEducation>,
    pub employment: Option<ResumeConditionArea>,
    pub employments: Box<Option<ResumeConditionCitizenship>>,
    pub experience: Box<Option<ResumeConditionCitizenship>>,
    pub first_name: Option<FirstName>,
    pub gender: Option<ResumeConditionArea>,
    pub has_vehicle: Option<ResumeConditionArea>,
    pub hidden_fields: Box<Option<ResumeConditionCitizenship>>,
    pub language: Box<Option<ResumeConditionCitizenship>>,
    pub last_name: Option<FirstName>,
    pub metro: Option<ResumeConditionArea>,
    pub middle_name: Option<FirstName>,
    pub photo: Option<ResumeConditionArea>,
    pub portfolio: Box<Option<ResumeConditionCitizenship>>,
    pub professional_roles: Box<Option<ResumeConditionCitizenship>>,
    pub recommendation: Box<Option<ResumeConditionCitizenship>>,
    pub relocation: Option<ResumeConditionRelocation>,
    pub resume_locale: Option<ResumeConditionArea>,
    pub salary: Option<ResumeConditionSalary>,
    pub schedule: Option<ResumeConditionArea>,
    pub schedules: Box<Option<ResumeConditionCitizenship>>,
    pub site: Box<Option<ResumeConditionCitizenship>>,
    pub skill_set: Option<SkillSet>,
    pub skills: Option<FirstName>,
    pub title: Option<FirstName>,
    pub travel_time: Option<ResumeConditionArea>,
    pub work_ticket: Box<Option<ResumeConditionCitizenship>>,
    #[serde(flatten)]
    pub error: Option<RequestError>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResumeConditionAccess {
    pub fields: Option<ResumeConditionAccessFields>,
    pub required: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResumeConditionAccessFields {
    #[serde(rename = "type")]
    pub fields_type: Option<ResumeConditionArea>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResumeConditionArea {
    pub required: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BirthDate {
    pub max_date: Option<String>,
    pub min_date: Option<String>,
    pub required: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CitizenshipFields {
    pub comment: Option<FirstName>,
    pub preferred: Option<ResumeConditionArea>,
    #[serde(rename = "type")]
    pub fields_type: Option<ResumeConditionArea>,
    pub value: Option<ResumeConditionArea>,
    pub name: Option<FirstName>,
    pub organization: Option<FirstName>,
    pub result: Option<FirstName>,
    pub year: Option<Year>,
    pub area: Option<ResumeConditionArea>,
    pub company: Option<FirstName>,
    pub company_url: Option<FirstName>,
    pub description: Option<FirstName>,
    pub end: Option<BirthDate>,
    pub industries: Box<Option<ResumeConditionCitizenship>>,
    pub industry: Option<ResumeConditionArea>,
    pub position: Option<FirstName>,
    pub start: Option<BirthDate>,
    pub id: Option<ResumeConditionArea>,
    pub level: Option<ResumeConditionArea>,
    pub url: Option<FirstName>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResumeConditionCitizenship {
    pub max_count: Option<i64>,
    pub min_count: Option<i64>,
    pub required: Option<bool>,
    pub fields: Option<CitizenshipFields>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FirstName {
    pub max_length: Option<i64>,
    pub min_length: Option<i64>,
    pub required: Option<bool>,
    pub regexp: Option<String>,
    pub not_in: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Year {
    pub max_value: Option<i64>,
    pub min_value: Option<i64>,
    pub required: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResumeConditionEducation {
    pub fields: Option<EducationFields>,
    pub required: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EducationFields {
    pub additional: Box<Option<ResumeConditionCitizenship>>,
    pub attestation: Box<Option<ResumeConditionCitizenship>>,
    pub elementary: Box<Option<ResumeConditionCitizenship>>,
    pub level: Option<ResumeConditionArea>,
    pub primary: Box<Option<ResumeConditionCitizenship>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResumeConditionRelocation {
    pub fields: Option<RelocationFields>,
    pub required: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelocationFields {
    pub area: Box<Option<ResumeConditionCitizenship>>,
    pub district: Box<Option<ResumeConditionCitizenship>>,
    #[serde(rename = "type")]
    pub fields_type: Option<ResumeConditionArea>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResumeConditionSalary {
    pub fields: Option<SalaryFields>,
    pub required: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SalaryFields {
    pub amount: Option<Year>,
    pub currency: Option<ResumeConditionArea>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillSet {
    pub max_count: Option<i64>,
    pub max_length: Option<i64>,
    pub min_count: Option<i64>,
    pub min_length: Option<i64>,
    pub required: Option<bool>,
}
