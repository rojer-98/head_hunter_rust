use hh_rust::{
    auth::{authorize, oauth, AuthorizeQuery, TakeAuthQuery},
    HError,
};

use simple_logger::SimpleLogger;

pub fn init_logger() {
    let _ = SimpleLogger::new().with_utc_timestamps().init();
}

#[tokio::main]
async fn main() {
    init_logger();
    let taq = AuthorizeQuery::new(
        "code".to_string(),
        "HDC9C48GQMSE5BEPG40STPVDKGRDKPK4TS7CBR1OOPFOEJM3KRUQ0L10B4DT81EF".to_string(),
    );
    let auth = authorize(taq).await;

    println!("Auth is {:?}", auth);
}
