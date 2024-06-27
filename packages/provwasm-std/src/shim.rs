use std::fmt;
use std::str::FromStr;

use ::serde::{ser, Deserialize, Deserializer, Serialize, Serializer};
use chrono::{DateTime, Utc};
use cosmwasm_std::{Binary, StdResult};
use serde::de;
use serde::de::Visitor;
use serde::ser::SerializeMap;

#[derive(Clone, PartialEq, Eq, ::prost::Message, schemars::JsonSchema)]
pub struct Timestamp {
    /// Represents seconds of UTC time since Unix epoch
    /// 1970-01-01T00:00:00Z. Must be from 0001-01-01T00:00:00Z to
    /// 9999-12-31T23:59:59Z inclusive.
    #[prost(int64, tag = "1")]
    pub seconds: i64,
    /// Non-negative fractions of a second at nanosecond resolution. Negative
    /// second values with fractions must still have non-negative nanos values
    /// that count forward in time. Must be from 0 to 999,999,999
    /// inclusive.
    #[prost(int32, tag = "2")]
    pub nanos: i32,
}

impl Serialize for Timestamp {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        let mut ts = prost_types::Timestamp {
            seconds: self.seconds,
            nanos: self.nanos,
        };
        ts.normalize();
        let dt = DateTime::from_timestamp(ts.seconds, ts.nanos as u32).ok_or_else(|| {
            ser::Error::custom(format!(
                "failed to parse as NativeDateTime: seconds: {}, nanos: {}",
                self.seconds, self.nanos
            ))
        })?;
        serializer.serialize_str(format!("{:?}", dt).as_str())
    }
}

impl<'de> Deserialize<'de> for Timestamp {
    fn deserialize<D>(deserializer: D) -> Result<Self, <D as Deserializer<'de>>::Error>
    where
        D: Deserializer<'de>,
    {
        struct TimestampVisitor;

        impl<'de> Visitor<'de> for TimestampVisitor {
            type Value = Timestamp;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("Timestamp in RFC3339 format")
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                let utc: DateTime<Utc> = chrono::DateTime::from_str(value).map_err(|err| {
                    serde::de::Error::custom(format!(
                        "Failed to parse {} as datetime: {:?}",
                        value, err
                    ))
                })?;
                let ts = Timestamp::from(utc);
                Ok(ts)
            }
        }
        deserializer.deserialize_str(TimestampVisitor)
    }
}

impl From<DateTime<Utc>> for Timestamp {
    fn from(dt: DateTime<Utc>) -> Self {
        Timestamp {
            seconds: dt.timestamp(),
            nanos: dt.timestamp_subsec_nanos() as i32,
        }
    }
}
#[derive(Clone, PartialEq, Eq, ::prost::Message, schemars::JsonSchema)]
pub struct Duration {
    /// Signed seconds of the span of time. Must be from -315,576,000,000
    /// to +315,576,000,000 inclusive. Note: these bounds are computed from:
    /// 60 sec/min * 60 min/hr * 24 hr/day * 365.25 days/year * 10000 years
    #[prost(int64, tag = "1")]
    pub seconds: i64,
    /// Signed fractions of a second at nanosecond resolution of the span
    /// of time. Durations less than one second are represented with a 0
    /// `seconds` field and a positive or negative `nanos` field. For durations
    /// of one second or more, a non-zero value for the `nanos` field must be
    /// of the same sign as the `seconds` field. Must be from -999,999,999
    /// to +999,999,999 inclusive.
    #[prost(int32, tag = "2")]
    pub nanos: i32,
}

impl Serialize for Duration {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        let mut d = prost_types::Duration::from(self.to_owned());
        d.normalize();

        serializer.serialize_str(d.to_string().as_str())
    }
}

impl<'de> Deserialize<'de> for Duration {
    fn deserialize<D>(deserializer: D) -> Result<Self, <D as Deserializer<'de>>::Error>
    where
        D: Deserializer<'de>,
    {
        struct DurationVisitor;

        impl<'de> Visitor<'de> for DurationVisitor {
            type Value = Duration;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("Timestamp in RFC3339 format")
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                value
                    .parse::<prost_types::Duration>()
                    .map(Into::into)
                    .map_err(de::Error::custom)
            }
        }
        deserializer.deserialize_str(DurationVisitor)
    }
}

#[derive(Clone, PartialEq, Eq, ::prost::Message, schemars::JsonSchema)]
pub struct Any {
    /// A URL/resource name that uniquely identifies the type of the serialized
    /// protocol buffer message. This string must contain at least
    /// one "/" character. The last segment of the URL's path must represent
    /// the fully qualified name of the type (as in
    /// `path/google.protobuf.Duration`). The name should be in a canonical form
    /// (e.g., leading "." is not accepted).
    ///
    /// In practice, teams usually precompile into the binary all types that they
    /// expect it to use in the context of Any. However, for URLs which use the
    /// scheme `http`, `https`, or no scheme, one can optionally set up a type
    /// server that maps type URLs to message definitions as follows:
    ///
    /// * If no scheme is provided, `https` is assumed.
    /// * An HTTP GET on the URL must yield a \[google.protobuf.Type][\]
    ///   value in binary format, or produce an error.
    /// * Applications are allowed to cache lookup results based on the
    ///   URL, or have them precompiled into a binary to avoid any
    ///   lookup. Therefore, binary compatibility needs to be preserved
    ///   on changes to types. (Use versioned type names to manage
    ///   breaking changes.)
    ///
    /// Note: this functionality is not currently available in the official
    /// protobuf release, and it is not used for type URLs beginning with
    /// type.googleapis.com.
    ///
    /// Schemes other than `http`, `https` (or the empty scheme) might be
    /// used with implementation specific semantics.
    ///
    #[prost(string, tag = "1")]
    pub type_url: ::prost::alloc::string::String,
    /// Must be a valid serialized protocol buffer of the above specified type.
    #[prost(bytes = "vec", tag = "2")]
    pub value: ::prost::alloc::vec::Vec<u8>,
}

impl Serialize for Any {
    fn serialize<S>(
        &self,
        serializer: S,
    ) -> Result<<S as ::serde::Serializer>::Ok, <S as ::serde::Serializer>::Error>
    where
        S: ::serde::Serializer,
    {
        let mut map = serializer.serialize_map(Some(2))?;
        map.serialize_entry("@type", &self.type_url)?;
        map.serialize_entry("value", &Binary::from(self.value.clone()))?;
        map.end()
    }
}

impl<'de> Deserialize<'de> for Any {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Get raw value from deserializer
        let raw = match serde_cw_value::Value::deserialize(deserializer) {
            Ok(raw) => raw,
            Err(err) => {
                return Err(err);
            }
        };

        // Turn raw value into deserialize map value
        let map = match raw.clone() {
            serde_cw_value::Value::Map(m) => Ok(m),
            _ => Err(serde::de::Error::custom("data must have map structure")),
        }?;

        // Get type url from map
        let type_url = map
            .get(&serde_cw_value::Value::String("@type".to_string()))
            .map(|t| match t.to_owned() {
                serde_cw_value::Value::String(s) => Ok(s),
                _ => Err(serde::de::Error::custom("type_url must be String")),
            })
            .transpose()?
            .unwrap_or_default();

        // Get base64 encoded value from deserialize map
        let value_in_base64 = map
            .get(&serde_cw_value::Value::String("value".to_string()))
            .map(|t| match t.to_owned() {
                serde_cw_value::Value::String(s) => Ok(s),
                _ => Err(serde::de::Error::custom("value must be String")),
            })
            .transpose()?
            .unwrap_or_default();

        // Convert base64 encoded value into vector
        let value = match Binary::from_base64(&value_in_base64) {
            Ok(v) => Ok(v),
            _ => Err(serde::de::Error::custom("value must be base64 encoded")),
        }?
        .to_vec();

        Ok(Any { type_url, value })
    }
}

macro_rules! impl_prost_types_exact_conversion {
    ($t:ident | $($arg:ident),*) => {
        impl From<$t> for prost_types::$t {
            fn from(src: $t) -> Self {
                prost_types::$t {
                    $(
                        $arg: src.$arg,
                    )*
                }
            }
        }

        impl From<prost_types::$t> for $t {
            fn from(src: prost_types::$t) -> Self {
                $t {
                    $(
                        $arg: src.$arg,
                    )*
                }
            }
        }
    };
}

impl_prost_types_exact_conversion! { Timestamp | seconds, nanos }
impl_prost_types_exact_conversion! { Duration | seconds, nanos }
impl_prost_types_exact_conversion! { Any | type_url, value }

impl From<cosmwasm_std::Coin> for crate::types::cosmos::base::v1beta1::Coin {
    fn from(cosmwasm_std::Coin { denom, amount }: cosmwasm_std::Coin) -> Self {
        crate::types::cosmos::base::v1beta1::Coin {
            denom,
            amount: amount.into(),
        }
    }
}

impl TryFrom<crate::types::cosmos::base::v1beta1::Coin> for cosmwasm_std::Coin {
    type Error = cosmwasm_std::StdError;

    fn try_from(
        crate::types::cosmos::base::v1beta1::Coin { denom, amount }: crate::types::cosmos::base::v1beta1::Coin,
    ) -> StdResult<Self> {
        Ok(cosmwasm_std::Coin {
            denom,
            amount: amount.parse()?,
        })
    }
}

/// Convert a list of `Coin` from provenance proto generated `Coin` type to cosmwasm `Coin` type
pub fn try_proto_to_cosmwasm_coins(
    coins: impl IntoIterator<Item = crate::types::cosmos::base::v1beta1::Coin>,
) -> StdResult<Vec<cosmwasm_std::Coin>> {
    coins.into_iter().map(|c| c.try_into()).collect()
}

/// Convert a list of `Coin` from cosmwasm `Coin` type to proto generated `Coin` type
pub fn cosmwasm_to_proto_coins(
    coins: impl IntoIterator<Item = cosmwasm_std::Coin>,
) -> Vec<crate::types::cosmos::base::v1beta1::Coin> {
    coins.into_iter().map(|c| c.into()).collect()
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::Uint128;

    use super::*;

    #[test]
    fn test_coins_conversion() {
        let coins = vec![
            cosmwasm_std::Coin {
                denom: "uatom".to_string(),
                amount: Uint128::new(100),
            },
            cosmwasm_std::Coin {
                denom: "nhash".to_string(),
                amount: Uint128::new(200),
            },
        ];

        let proto_coins = cosmwasm_to_proto_coins(coins.clone());
        let cosmwasm_coins = try_proto_to_cosmwasm_coins(proto_coins).unwrap();

        assert_eq!(coins, cosmwasm_coins);
    }
}
