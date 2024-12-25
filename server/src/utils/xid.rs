use std::str::FromStr;

use serde::Serialize;

#[derive(Debug)]
pub struct Xid(xid::Id);

impl Xid {
    #[allow(clippy::inherent_to_string)]
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
