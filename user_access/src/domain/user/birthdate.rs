use common::{
    chrono::{DateTime, Datelike, Utc},
    Error, Result,
};
use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct Birthdate(pub DateTime<Utc>);

impl Birthdate {
    pub fn parse(birthdate: DateTime<Utc>) -> Result<Birthdate> {
        let now = Utc::now();
        let diff = now.year() - birthdate.year();

        // 暂定16
        if diff < 16 {
            return Err(Error::bad_format("birthdate").set_message("age must be greater than 16"));
        }

        Ok(Birthdate(birthdate))
    }
}

impl AsRef<DateTime<Utc>> for Birthdate {
    fn as_ref(&self) -> &DateTime<Utc> {
        &self.0
    }
}

impl FromStr for Birthdate {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        Self::parse(
            DateTime::parse_from_rfc3339(s)
                .map(DateTime::<Utc>::from)
                .map_err(|err| Error::bad_format("birthdate").set_message(err.to_string()))?,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn too_old() {
        let date: DateTime<Utc> = DateTime::parse_from_rfc3339("1886-12-19T16:39:57+00:00")
            .unwrap()
            .into();
        assert!(Birthdate::parse(date).is_err());
    }

    #[test]
    fn too_young() {
        let date: DateTime<Utc> = DateTime::parse_from_rfc3339("2009-07-21T16:39:57+00:00")
            .unwrap()
            .into();
        assert!(Birthdate::parse(date).is_err());
    }

    #[test]
    fn from_str() {
        assert!(Birthdate::from_str("1994-12-32T16:39:57+00:00").is_err());
        assert!(Birthdate::from_str("1994-12-20T16:61:57+00:00").is_err());
        assert!(Birthdate::from_str("1994-12-20T16:39:57-03:00").is_ok());
    }
}
