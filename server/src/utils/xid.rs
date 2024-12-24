#[derive(Debug)]
pub struct Xid(xid::Id);

impl Xid {
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
