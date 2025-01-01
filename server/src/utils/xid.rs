use std::{fmt, str::FromStr};

use serde::{
    de::{self, Visitor},
    Deserialize, Deserializer, Serialize,
};

/**
 * Wrapper around xid::Id with improved ergonomics
 */
#[derive(Debug, Copy, Clone)]
pub struct Xid(xid::Id);

impl Xid {
    #[allow(clippy::inherent_to_string, clippy::wrong_self_convention)]
    pub fn to_string(&self) -> String {
        self.0.to_string()
    }

    pub fn as_bytes(&self) -> &[u8; 12] {
        self.0.as_bytes()
    }

    pub fn new() -> Xid {
        Xid(xid::new())
    }
}

impl From<Vec<u8>> for Xid {
    fn from(value: Vec<u8>) -> Self {
        let mut id_bytes = [0_u8; 12];
        id_bytes.clone_from_slice(&value);

        Xid(xid::Id(id_bytes))
    }
}

impl TryFrom<&str> for Xid {
    type Error = xid::ParseIdError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        xid::Id::from_str(value).map(Xid)
    }
}

impl Serialize for Xid {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

struct XidVisitor;

impl<'de> Visitor<'de> for XidVisitor {
    type Value = Xid;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("A valid string representation of an XID")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Xid::try_from(v).map_err(|_| E::custom("Expected valid XID string representation"))
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Xid::try_from(v.as_str()).map_err(|_| E::custom("Expected valid XID string representation"))
    }

    fn visit_borrowed_str<E>(self, v: &'de str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Xid::try_from(v).map_err(|_| E::custom("Expected valid XID string representation"))
    }
}

impl<'de> Deserialize<'de> for Xid {
    fn deserialize<D>(deserializer: D) -> Result<Xid, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(XidVisitor)
    }
}
