use core::{fmt, str};

impl fmt::Display for crate::NamespaceUuidError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self {
            crate::NamespaceUuidError::WellKnownUuid(given) => {
                write!(
                    f,
                    "expected `@dns`, `@oid`, `@url`, `x500` or a uuid string, found `{}`",
                    given
                )?;
                Ok(())
            }

            crate::NamespaceUuidError::Uuid(parse_err) => fmt::Display::fmt(parse_err, f),
        }
    }
}

impl str::FromStr for crate::NamespaceUuid {
    type Err = crate::NamespaceUuidError;

    #[inline]
    fn from_str(string: &str) -> Result<Self, Self::Err> {
        Self::parse(string)
    }
}

impl From<crate::NamespaceUuid> for uuid::Uuid {
    fn from(ns_uuid: crate::NamespaceUuid) -> Self {
        ns_uuid.0
    }
}
