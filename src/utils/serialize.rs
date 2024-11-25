use std::str::FromStr;

use rbatis::rbdc::DateTime;
use serde::de::{Error, Visitor};
use serde::ser::SerializeSeq;
use serde::{de, Deserialize, Deserializer, Serializer};

///
/// ## 序列化相关的内容
///

pub fn serialize_id<S>(data: &u64, s: S) -> Result<S::Ok, S::Error>
    where S: Serializer, {
    s.serialize_str(data.to_string().as_str())
}

pub fn serialize_vec_id<S>(data: &Vec<u64>, s: S) -> Result<S::Ok, S::Error>
    where S: Serializer, {
    let mut seq = s.serialize_seq(Some(data.len()))?;
    for element in data {
        seq.serialize_element(&element.to_string())?;
    }
    seq.end()
}

struct VecIdOptionVisitor;

impl<'de> Visitor<'de> for VecIdOptionVisitor {
    type Value = Option<Vec<u64>>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("an optional vector of strings that can be parsed into u64")
    }

    fn visit_none<E>(self) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(None)
    }

    fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        let strings: Vec<String> = Vec::deserialize(deserializer)?;
        let data: Result<Vec<u64>, _> = strings.into_iter().map(|s| s.parse()).collect();
        data.map(Some).map_err(de::Error::custom)
    }
}
pub fn deserialize_vec_id_option<'de, D>(deserializer: D) -> Result<Option<Vec<u64>>, D::Error>
where D: Deserializer<'de>, {
    deserializer.deserialize_option(VecIdOptionVisitor)

}

pub fn deserialize_id<'de, D>(deserializer: D) -> Result<u64, D::Error>
    where D: Deserializer<'de>, {
    let id = String::deserialize(deserializer)?;
    let id = u64::from_str(id.as_str()).unwrap();
    Ok(id)
}

pub fn deserialize_bool<'de, D>(deserializer: D) -> Result<bool, D::Error>
    where D: Deserializer<'de>, {

    let result = u8::deserialize(deserializer)?;
    return if result >= 1 {
        Ok(true)
    } else {
        Ok(false)
    }
}

pub fn serialize_bool<S>(data: &bool, s: S) -> Result<S::Ok, S::Error>
    where S: Serializer, {
    s.serialize_u8(if *data {1}else{0})
}

pub fn deserialize_bool_option<'de, D>(deserializer: D) -> Result<Option<bool>, D::Error>
    where D: Deserializer<'de>, {

    let result = u8::deserialize(deserializer)?;
    return if result >= 1 {
        Ok(Some(true))
    } else {
        Ok(Some(true))
    }
}

const FORMAT: &str = "YYYY-MM-DD hh:mm:ss";
pub fn serialize_datetime<S>(date_time: &DateTime, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
{
    serializer.serialize_str(&date_time.format(FORMAT))
}

pub fn serialize_option_datetime<S>(date_time: &Option<DateTime>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
{
    match date_time {
        None => {
            serializer.serialize_none()
        }
        Some(format) => {
            serializer.serialize_str(&format.format(FORMAT))
        }
    }
}

pub fn deserialize_date<'de, D>(deserializer: D) -> Result<DateTime, D::Error>
    where D: Deserializer<'de>, {
    let date = String::deserialize(deserializer)?;
    let result = DateTime::parse(FORMAT, date.as_str());
    match result {
        Ok(date) => {
            Ok(date)
        }
        Err(_) => {
            Err(D::Error::custom("解析字段失败"))
        }
    }
}

pub fn deserialize_date_option<'de, D>(deserializer: D) -> Result<Option<DateTime>, D::Error>
    where D: Deserializer<'de>, {
    let date = String::deserialize(deserializer)?;
    let result = DateTime::parse(FORMAT, date.as_str());
    match result {
        Ok(date) => {
            Ok(Some(date))
        }
        Err(_) => {
            Err(Error::custom("解析字段失败"))
        }
    }
}