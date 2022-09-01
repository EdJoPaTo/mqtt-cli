use std::str::Utf8Error;

use chrono::{DateTime, Local};
use json::JsonValue;
use rumqttc::{Publish, QoS};

#[derive(Debug, Clone, Copy)]
pub enum Time {
    Retained,
    Local(DateTime<Local>),
}

impl Time {
    pub const fn as_optional(&self) -> Option<DateTime<Local>> {
        if let Self::Local(time) = self {
            Some(*time)
        } else {
            None
        }
    }
}

impl ToString for Time {
    fn to_string(&self) -> String {
        match self {
            // TODO: lazy_static
            Self::Retained => String::from("RETAINED"),
            Self::Local(time) => time.format("%_H:%M:%S.%3f").to_string(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Payload {
    NotUtf8(Utf8Error),
    String(Box<str>),
    Json(JsonValue),
}

impl Payload {
    pub fn new(payload: &bytes::Bytes) -> Self {
        match String::from_utf8(payload.to_vec()) {
            Ok(str) => {
                if let Ok(json) = json::parse(&str) {
                    Self::Json(json)
                } else {
                    Self::String(str.into())
                }
            }
            Err(err) => Self::NotUtf8(err.utf8_error()),
        }
    }

    pub const fn as_optional_json(&self) -> Option<&JsonValue> {
        if let Self::Json(json) = self {
            Some(json)
        } else {
            None
        }
    }
}

pub struct HistoryEntry {
    pub qos: QoS,
    pub time: Time,
    pub payload_size: usize,
    pub payload: Payload,
}

impl HistoryEntry {
    pub fn new(packet: &Publish, time: DateTime<Local>) -> Self {
        let time = if packet.retain {
            Time::Retained
        } else {
            Time::Local(time)
        };
        Self {
            qos: packet.qos,
            time,
            payload_size: packet.payload.len(),
            payload: Payload::new(&packet.payload),
        }
    }
}

#[test]
fn time_optional_retained() {
    let time = Time::Retained;
    assert_eq!(time.as_optional(), None);
}


#[test]
fn time_optional_time() {
    let date = DateTime::parse_from_rfc3339("1996-12-19T16:39:57+01:00").unwrap().into();
    let time = Time::Local(date);
    assert_eq!(time.as_optional(), Some(date));
}

#[test]
fn time_retained_to_string() {
    let time = Time::Retained;
    assert_eq!(time.to_string(), "RETAINED");
}

#[test]
fn time_local_to_string() {
    let date = DateTime::parse_from_rfc3339("1996-12-19T16:39:57+01:00").unwrap().into();
    let time = Time::Local(date);
    assert_eq!(time.to_string(), "16:39:57.000");
}

#[cfg(test)]
fn json_macro(json_str: &'static str) -> Option<String> {
    let payload = Payload::new(&json_str.into());
    payload.as_optional_json().map(JsonValue::dump)
}

#[test]
fn payload_pretty_json_ignores_plain() {
    assert_eq!(None, json_macro("bob"));
}

#[test]
fn payload_pretty_json_object_works() {
    assert_eq!(
        json_macro(r#"{"a": "alpha", "b": "beta"}"#),
        Some(r#"{"a":"alpha","b":"beta"}"#.to_string())
    );
}

#[test]
fn payload_pretty_json_number_works() {
    assert_eq!(json_macro("42"), Some("42".to_string()));
}
