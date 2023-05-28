use digest::DigestAuth;
use log::{trace, warn};
use reqwest::{
    multipart::{Form, Part},
    redirect::Policy,
    Client,
};

use crate::HError;

#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
pub enum AuthType {
    Bearer,
    Digest,
    Basic,
    No,
}

#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
pub enum RequestType {
    Reqwest,
    Curl,
    All,
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
pub enum Header {
    JSON,
    USER_AGENT,
}

impl Header {
    pub fn to_curl(&self) -> &str {
        use Header::*;

        match self {
            JSON => "Content-Type: application/json",
            USER_AGENT => "User-Agent: api-test-agent",
        }
    }
}

#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
pub enum Method {
    GET,
    PUT,
    POST,
    DELETE,
    PATCH,
    HEAD,
    CONNECT,
    OPTIONS,
    TRACE,
}

impl ToString for Method {
    fn to_string(&self) -> String {
        use Method::*;

        match self {
            GET => "GET".to_owned(),
            PUT => "PUT".to_owned(),
            POST => "POST".to_owned(),
            DELETE => "DELETE".to_owned(),
            PATCH => "PATCH".to_owned(),
            HEAD => "HEAD".to_owned(),
            CONNECT => "CONNECT".to_owned(),
            OPTIONS => "OPTIONS".to_owned(),
            TRACE => "TRACE".to_owned(),
        }
    }
}

pub async fn request(
    rt: RequestType,
    url: String,
    params: Option<String>,
    auth: (Option<String>, Option<String>),
    auth_type: AuthType,
    method: Method,
    headers: Option<Vec<Header>>,
) -> Result<String, HError> {
    use RequestType::*;
    match rt {
        Reqwest => r_reqwest(url, params, auth, auth_type, method, headers).await,
        Curl => r_curl(url, params, auth, auth_type, method, headers).await,
        All => r_curl(
            url.clone(),
            params.clone(),
            auth.clone(),
            auth_type,
            method,
            headers.clone(),
        )
        .await
        .and(r_reqwest(url, params, auth, auth_type, method, headers).await),
    }
}

async fn r_curl(
    url: String,
    params: Option<String>,
    auth: (Option<String>, Option<String>),
    auth_type: AuthType,
    method: Method,
    headers: Option<Vec<Header>>,
) -> Result<String, HError> {
    let mut cmd = tokio::process::Command::new("curl");

    cmd.arg(url).arg("-X").arg(method.to_string());

    if params.is_some() {
        cmd.arg("-d").arg(params.unwrap());
    }

    if headers.is_some() {
        let headers = headers.unwrap();

        for h in headers {
            cmd.arg("-H").arg(h.to_curl());
        }
    }

    // Wait for 5 second
    cmd.arg("--max-time").arg(5u32.to_string());

    if let (Some(username), Some(password)) = auth {
        let auth = format!("{}:{}", username, password);

        match auth_type {
            AuthType::Digest => {
                cmd.arg("--digest").arg("--user").arg(auth);
            }
            AuthType::Basic => {
                cmd.arg("--basic").arg("--user").arg(auth);
            }
            AuthType::Bearer => {
                cmd.arg("--bearer").arg("--user").arg(auth);
            }
            _ => (),
        };
    }

    let output = cmd
        .output()
        .await
        .map_err(|source| HError::Std { source })?
        .stdout;

    Ok(String::from_utf8(output).map_err(|source| HError::Utf8 { source })?)
}

async fn r_reqwest(
    url: String,
    params: Option<String>,
    auth: (Option<String>, Option<String>),
    auth_type: AuthType,
    method: Method,
    headers: Option<Vec<Header>>,
) -> Result<String, HError> {
    use Method::*;

    trace!("Full url to req: {url}");

    let client = Client::builder()
        .redirect(Policy::custom(|attempt| {
            if attempt.previous().len() > 30 {
                attempt.error("too many redirects")
            } else {
                attempt.follow()
            }
        }))
        .build()?;

    let params = params.unwrap_or_default();
    let (username, password) = auth;

    let rb = match method {
        GET => client.get(url),
        PUT => client.put(url).body(params),
        POST => client.post(url).body(params),
        DELETE => client.delete(url).body(params),
        HEAD => client.head(url),
        _ => {
            warn!("Not supported HTTP request {:?}", method);
            return Err(HError::NotSet);
        }
    };

    let mut rb = if let Some(h_s) = headers {
        let mut rb_h = rb;
        for h in h_s {
            rb_h = match h {
                Header::JSON => rb_h.header(reqwest::header::CONTENT_TYPE, "application/json"),
                Header::USER_AGENT => rb_h.header(reqwest::header::USER_AGENT, "api-test-agent"),
            };
        }

        rb_h
    } else {
        rb
    };

    rb = match (username.as_ref(), password.as_ref()) {
        (Some(username), Some(password)) => match auth_type {
            AuthType::Digest => rb.digest_auth(&username, &password).await?,
            AuthType::Bearer => rb.bearer_auth(&password),
            AuthType::Basic => rb.basic_auth(&username, Some(password)),
            AuthType::No => rb,
        },
        _ => rb,
    };

    Ok(rb.send().await?.text().await?)
}

#[macro_export]
macro_rules! request_and_convert {
    (url: $raw_url:expr, method: $method:ident, access_token: $access_token:ident, optional $query:ident, $out_type:ty $(, $body:ident)?) => {{
        let url = if let Some(q) = $query {
            let ser_q = q.into_query_string()?;
            format!("{}?{}", $raw_url, ser_q)
        } else {
            $raw_url
        };
        #[allow(unused_variables)]
        let body : Option<String> = None;
        $( let body = Some($body.into_query_string()?); )?


        let req = crate::utils::request(
            crate::utils::RequestType::Reqwest,
            url,
            body,
            (None, $access_token),
            crate::utils::AuthType::No,
            crate::utils::Method::$method,
            Some(vec![crate::utils::Header::USER_AGENT]),
        )
        .await?;

        ::log::trace!("Req is {:?}", req);

        Ok(::serde_json::from_str::<$out_type>(&req)?)
    }};
    (url: $raw_url:expr, method: $method:ident, access_token: $access_token:ident, optional $query:ident, $out_type:ty $(, optional $body:ident)?) => {{
        let url = if let Some(q) = $query {
            let ser_q = q.into_query_string()?;
            format!("{}?{}", $raw_url, ser_q)
        } else {
            $raw_url
        };

        let body : Option<String> = None;
        $( let body = if let Some(b) = $body {
            Some(b.into_query_string()?)
        } else {
            None
        }; )?

        let req = crate::utils::request(
            crate::utils::RequestType::Reqwest,
            url,
            body,
            (None, $access_token),
            crate::utils::AuthType::No,
            crate::utils::Method::$method,
            Some(vec![crate::utils::Header::USER_AGENT]),
        )
        .await?;

        ::log::trace!("Req is {:?}", req);

        Ok(::serde_json::from_str::<$out_type>(&req)?)
    }};
    (url: $raw_url:expr, method: $method:ident, access_token: $access_token:ident, $query:ident, $out_type:ty $(, $body:ident)?) => {{
        let ser_q = $query.into_query_string()?;
        let url = format!("{}?{}", $raw_url, ser_q);

        #[allow(unused_variables)]
        let body : Option<String> = None;
        $( let body = Some($body.into_query_string()?); )?

        let req = crate::utils::request(
            crate::utils::RequestType::Reqwest,
            url,
            body,
            (None, $access_token),
            crate::utils::AuthType::No,
            crate::utils::Method::$method,
            Some(vec![crate::utils::Header::USER_AGENT]),
        )
        .await?;

        ::log::trace!("Req is {:?}", req);

        Ok(::serde_json::from_str::<$out_type>(&req)?)
    }};
}
