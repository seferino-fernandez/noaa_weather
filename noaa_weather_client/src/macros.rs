//! Crate-private macros shared by the typed value modules.
//!
//! Every typed value in [`crate::ids`], [`crate::geo`], and [`crate::time`]
//! is a string on the wire: it implements `Display` and `FromStr`, converts
//! from `&str`/`String`, serializes through its string form, and describes
//! itself to `schemars` as a string schema. These macros keep that surface
//! identical across types.

/// Implements `TryFrom<&str>` and `TryFrom<String>` in terms of `FromStr`.
macro_rules! impl_try_from_str {
    ($ty:ident) => {
        impl TryFrom<&str> for $ty {
            type Error = $crate::ids::InvalidValue;

            fn try_from(value: &str) -> Result<Self, Self::Error> {
                value.parse()
            }
        }

        impl TryFrom<String> for $ty {
            type Error = $crate::ids::InvalidValue;

            fn try_from(value: String) -> Result<Self, Self::Error> {
                value.parse()
            }
        }
    };
}

/// Implements `schemars::JsonSchema` as an inline string schema with a
/// description and an optional regular expression pattern.
///
/// Without the `schemars` feature the macro expands to nothing, so call
/// sites need no `cfg` attribute of their own.
#[cfg(feature = "schemars")]
macro_rules! impl_string_schema {
    ($ty:ident, $description:expr) => {
        impl schemars::JsonSchema for $ty {
            fn inline_schema() -> bool {
                true
            }

            fn schema_name() -> std::borrow::Cow<'static, str> {
                std::borrow::Cow::Borrowed(stringify!($ty))
            }

            fn schema_id() -> std::borrow::Cow<'static, str> {
                std::borrow::Cow::Borrowed(concat!(module_path!(), "::", stringify!($ty)))
            }

            fn json_schema(_generator: &mut schemars::SchemaGenerator) -> schemars::Schema {
                schemars::json_schema!({
                    "type": "string",
                    "description": $description,
                })
            }
        }
    };
    ($ty:ident, $description:expr, $pattern:expr) => {
        impl schemars::JsonSchema for $ty {
            fn inline_schema() -> bool {
                true
            }

            fn schema_name() -> std::borrow::Cow<'static, str> {
                std::borrow::Cow::Borrowed(stringify!($ty))
            }

            fn schema_id() -> std::borrow::Cow<'static, str> {
                std::borrow::Cow::Borrowed(concat!(module_path!(), "::", stringify!($ty)))
            }

            fn json_schema(_generator: &mut schemars::SchemaGenerator) -> schemars::Schema {
                schemars::json_schema!({
                    "type": "string",
                    "description": $description,
                    "pattern": $pattern,
                })
            }
        }
    };
}

#[cfg(not(feature = "schemars"))]
macro_rules! impl_string_schema {
    ($($tt:tt)*) => {};
}

/// Defines a validated, uppercase-or-preserved ASCII identifier newtype over
/// `Box<str>` with the full string-value surface: `Debug` printing the inner
/// string, `Display`, `FromStr`, `TryFrom<&str>`, `TryFrom<String>`,
/// `AsRef<str>`, `as_str()`, serde through the string form, and a
/// `schemars` string schema.
///
/// `$parse` is a `fn(&str) -> Result<Box<str>, InvalidValue>` that performs
/// validation and normalization.
macro_rules! str_id {
    (
        $(#[$meta:meta])*
        $ty:ident, $parse:path, $description:expr, $pattern:expr
    ) => {
        $(#[$meta])*
        #[derive(Clone, PartialEq, Eq, Hash, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]
        #[serde(try_from = "String", into = "String")]
        pub struct $ty(Box<str>);

        impl $ty {
            /// Returns the normalized identifier as a string slice.
            #[must_use]
            pub fn as_str(&self) -> &str {
                &self.0
            }
        }

        impl std::fmt::Debug for $ty {
            fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                std::fmt::Debug::fmt(self.as_str(), formatter)
            }
        }

        impl std::fmt::Display for $ty {
            fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str(self.as_str())
            }
        }

        impl std::str::FromStr for $ty {
            type Err = $crate::ids::InvalidValue;

            fn from_str(input: &str) -> Result<Self, Self::Err> {
                $parse(input).map(Self)
            }
        }

        impl AsRef<str> for $ty {
            fn as_ref(&self) -> &str {
                self.as_str()
            }
        }

        impl From<$ty> for String {
            fn from(value: $ty) -> Self {
                value.0.into_string()
            }
        }

        impl_try_from_str!($ty);
        impl_string_schema!($ty, $description, $pattern);
    };
}
