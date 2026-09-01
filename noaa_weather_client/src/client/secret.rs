//! Redacted storage for the optional API key.

use std::fmt;

/// An API key that never appears in `Debug` output.
///
/// The value is only readable through [`Secret::expose`], which the request
/// pipeline calls when it sets the `X-Api-Key` header.
#[derive(Clone, PartialEq, Eq)]
pub(crate) struct Secret(Box<str>);

impl Secret {
    pub(crate) fn new(value: impl Into<String>) -> Self {
        Self(value.into().into_boxed_str())
    }

    /// Returns the protected value for header construction.
    pub(crate) fn expose(&self) -> &str {
        &self.0
    }
}

impl fmt::Debug for Secret {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("[redacted]")
    }
}

#[cfg(test)]
mod tests {
    use super::Secret;

    #[test]
    fn debug_never_prints_the_value() {
        let secret = Secret::new("top-secret-key");
        assert_eq!(format!("{secret:?}"), "[redacted]");
        assert_eq!(format!("{:#?}", Some(&secret)), "Some(\n    [redacted],\n)");
        assert_eq!(secret.expose(), "top-secret-key");
    }
}
