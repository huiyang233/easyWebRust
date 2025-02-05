use serde::de::Visitor;
use serde::ser::SerializeSeq;
use serde::{de, Deserialize, Deserializer, Serializer};
use std::str::FromStr;

///
/// ## 序列化相关的内容
///

pub fn serialize_id<S>(data: &i64, s: S) -> Result<S::Ok, S::Error>
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
    type Value = Option<Vec<i64>>;

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
        let data: Result<Vec<i64>, _> = strings.into_iter().map(|s| s.parse()).collect();
        data.map(Some).map_err(de::Error::custom)
    }
}
pub fn deserialize_vec_id_option<'de, D>(deserializer: D) -> Result<Option<Vec<i64>>, D::Error>
where D: Deserializer<'de>, {
    deserializer.deserialize_option(VecIdOptionVisitor)

}

pub fn deserialize_id<'de, D>(deserializer: D) -> Result<i64, D::Error>
    where D: Deserializer<'de>, {
    let id = String::deserialize(deserializer)?;
    let id = i64::from_str(id.as_str()).unwrap();
    Ok(id)
}

pub fn deserialize_vec_id<'de, D>(deserializer: D) -> Result<Vec<i64>, D::Error>
where D: Deserializer<'de>, {
    let mut i64_vec = vec![];
    let vec = Vec::<String>::deserialize(deserializer)?;
    vec.iter().for_each(|id| {
        i64_vec.push(i64::from_str(id.as_str()).unwrap())
    });
    Ok(i64_vec)
}
