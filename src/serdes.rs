pub(crate) mod serfloats {
    use serde::{Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(value: &Option<f32>, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        if let Some(ref v) = *value {
            return serializer.collect_str(&v.to_string());
        }
        serializer.serialize_none()
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<f32>, D::Error> where D: Deserializer<'de>, {
        let s: Option<String> = Option::deserialize(deserializer)?;
        if let Some(s) = s {
            return Ok(Some(s.parse::<f32>().map_err(serde::de::Error::custom)?));
        }
        Ok(None)
    }
}

pub(crate) mod serdates {
    use chrono::prelude::*;
    use serde::{Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(value: &Option<DateTime<Utc>>, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer, {
        if let Some(ref v) = *value {
            return serializer.collect_str(&v.to_rfc3339());
        }
        serializer.serialize_none()
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<DateTime<Utc>>, D::Error> where D: Deserializer<'de>, {
        let s: Option<String> = Option::deserialize(deserializer)?;
        if let Some(s) = s {
            return if s == "0" {
                Ok(None)
            } else {
                let d = DateTime::from_utc(
                    DateTime::<FixedOffset>::parse_from_rfc3339(&s).unwrap().naive_utc(),
                    Utc,
                );
                Ok(Some(d))
            };
        }
        Ok(None)
    }
}
