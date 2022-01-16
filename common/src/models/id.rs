use crate::{Error, Result};
use std::{
    hash::{Hash, Hasher},
    str::FromStr,
};

pub type Id = Xid;

#[derive(Debug, Clone, Copy, Eq)]
pub struct Xid(pub [u8; 12]);

impl Xid {
    pub fn next() -> Xid {
        Xid(xid::new().0)
    }
    pub fn parse<S: Into<String>>(s: S) -> Result<Self> {
        let id = xid::Id::from_str(s.into().as_str())
            .map_err(|e| Error::bad_format("id").set_message(e.to_string()))?;

        Ok(Xid(id.0))
    }
}

impl PartialEq for Xid {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Hash for Xid {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

impl AsRef<[u8]> for Xid {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl From<Vec<u8>> for Xid {
    fn from(id: Vec<u8>) -> Self {
        let mut xid: [u8; 12] = [Default::default(); 12];
        xid[..id.len()].copy_from_slice(&id);

        Xid(xid)
    }
}

impl ToString for Xid {
    fn to_string(&self) -> String {
        xid::Id(self.0).to_string()
    }
}
