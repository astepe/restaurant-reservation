use chrono::prelude::*;
use serde::de;
use serde::{Deserialize, Serialize};
use std::fmt;

// The derive "attribute macro" (i.e. any macro that is invoked by an attribute)
// takes a trait (AKA interface) -- in this case Serialize --
// and figures out how to implement the required methods
// of the trait for the struct that is defined below the macro
// and adds this implementation to the struct at compile time.
#[derive(Serialize)]
pub struct GenericResponse {
    pub status: String,
    pub message: String,
}

struct NaiveDateTimeVisitor;

impl<'de> de::Visitor<'de> for NaiveDateTimeVisitor {
    type Value = NaiveDateTime;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "a string represents chrono::NaiveDateTime")
    }

    fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        match NaiveDateTime::parse_from_str(s, "%Y-%m-%dT%H:%M:%S.%f") {
            Ok(t) => Ok(t),
            Err(_) => Err(de::Error::invalid_value(de::Unexpected::Str(s), &self)),
        }
    }
}

fn from_timestamp<'de, D>(d: D) -> Result<NaiveDateTime, D::Error>
where
    D: de::Deserializer<'de>,
{
    d.deserialize_str(NaiveDateTimeVisitor)
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Reservation {
    pub id: i32,
    pub name: String,
    pub email: String,
    #[serde(deserialize_with = "from_timestamp")]
    pub date: NaiveDateTime,
    pub quantity: u8,
}
