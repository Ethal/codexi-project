// src/core/serde_nulid.rs

use nulid::Nulid;
use serde::{Deserialize, Deserializer, Serializer};
use std::str::FromStr;

// for Nulid (no-optional)
pub fn serialize<S>(val: &Nulid, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    s.serialize_str(&val.to_string())
}

pub fn deserialize<'de, D>(d: D) -> Result<Nulid, D::Error>
where
    D: Deserializer<'de>,
{
    // String owned — compatible TOML 0.8 ET JSON ET CSV
    let s = String::deserialize(d)?;
    Nulid::from_str(&s).map_err(serde::de::Error::custom)
}

// for Option<Nulid>
pub mod option {
    use nulid::Nulid;
    use serde::{Deserialize, Deserializer, Serializer};
    use std::str::FromStr;

    pub fn serialize<S>(val: &Option<Nulid>, s: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match val {
            Some(id) if id.is_nil() => s.serialize_none(),
            Some(id) => s.serialize_str(&id.to_string()),
            None => s.serialize_none(),
        }
    }

    pub fn deserialize<'de, D>(d: D) -> Result<Option<Nulid>, D::Error>
    where
        D: Deserializer<'de>,
    {
        // deserialize_option laisse le deserializer gérer l'absence
        // correctement pour TOML, JSON et CSV
        struct NulidVisitor;

        impl<'de> serde::de::Visitor<'de> for NulidVisitor {
            type Value = Option<Nulid>;

            fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "a Nulid string or nothing")
            }

            fn visit_none<E: serde::de::Error>(self) -> Result<Self::Value, E> {
                Ok(None)
            }

            fn visit_some<D: Deserializer<'de>>(self, d: D) -> Result<Self::Value, D::Error> {
                let s = String::deserialize(d)?;
                if s.is_empty() || s.len() != 26 || s.chars().all(|c| c == '0') {
                    return Ok(None);
                }
                Ok(Nulid::from_str(&s).ok())
            }

            fn visit_str<E: serde::de::Error>(self, s: &str) -> Result<Self::Value, E> {
                if s.is_empty() || s.len() != 26 || s.chars().all(|c| c == '0') {
                    return Ok(None);
                }
                Ok(Nulid::from_str(s).ok())
            }
        }

        d.deserialize_option(NulidVisitor)
    }
}
