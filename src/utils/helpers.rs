use log::trace;
use serde::{
    de::{Deserialize, Deserializer, Error as _},
    ser::Serializer,
};
use url::Url;

use std::borrow::Cow;

pub fn deserialize_url<'de, D: Deserializer<'de>>(de: D) -> Result<Option<Url>, D::Error> {
    trace!("Start deserialize url");

    let ret = match <Option<Cow<'de, str>>>::deserialize(de)?.as_deref() {
        None | Some("") => Ok(None),
        Some(non_empty_string) => Url::parse(non_empty_string)
            .map(Some)
            .map_err(D::Error::custom),
    };

    trace!("End deserialize url: {:?}", ret);

    ret
}

pub fn serialize_url<S>(url: &Option<Url>, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    trace!("Start serialize url");

    let ret = if let Some(u) = url {
        s.serialize_str(u.as_str())
    } else {
        s.serialize_none()
    };

    trace!("End serialize url");

    ret
}
