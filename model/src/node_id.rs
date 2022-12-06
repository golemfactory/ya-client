use serde::{de, Deserialize, Deserializer, Serialize, Serializer};
use std::borrow::Cow;
use std::convert::TryFrom;
use std::str::FromStr;
use std::{fmt, str};

const NODE_ID_LENGTH: usize = 20;

#[derive(Clone, Debug, thiserror::Error, PartialEq, Serialize, Deserialize)]
#[error("NodeId `{original_str}` parsing error: {msg}")]
pub struct ParseError {
    original_str: String,
    msg: String,
}

impl ParseError {
    fn new(original_str: impl Into<String>, msg: impl Into<String>) -> Self {
        Self {
            original_str: original_str.into(),
            msg: msg.into(),
        }
    }
}

#[derive(Clone, Debug, thiserror::Error, PartialEq, Serialize, Deserialize)]
#[error("NodeId parsing error: {msg}")]
pub struct InvalidLengthError {
    msg: String,
}

/// Yagna node identity compliant with [Ethereum addresses](https://en.wikipedia.org/wiki/Ethereum#Addresses)
#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct NodeId {
    inner: [u8; NODE_ID_LENGTH],
}

impl NodeId {
    #[inline(always)]
    fn with_hex<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&str) -> R,
    {
        let mut hex_str = [0u8; 42];

        hex_str[0] = '0' as u8;
        hex_str[1] = 'x' as u8;

        let mut ptr = 2;
        for it in &self.inner {
            let hi = (it >> 4) & 0xfu8;
            let lo = it & 0xf;
            hex_str[ptr] = HEX_CHARS[hi as usize];
            hex_str[ptr + 1] = HEX_CHARS[lo as usize];
            ptr += 2;
        }
        assert_eq!(ptr, hex_str.len());

        let hex_str = unsafe { str::from_utf8_unchecked(&hex_str) };
        f(hex_str)
    }

    #[inline]
    pub fn into_array(self) -> [u8; NODE_ID_LENGTH] {
        self.inner
    }
}

impl Default for NodeId {
    fn default() -> Self {
        NodeId {
            inner: [0; NODE_ID_LENGTH],
        }
    }
}

impl AsRef<[u8]> for NodeId {
    fn as_ref(&self) -> &[u8] {
        &self.inner
    }
}

impl AsRef<[u8; NODE_ID_LENGTH]> for NodeId {
    fn as_ref(&self) -> &[u8; NODE_ID_LENGTH] {
        &self.inner
    }
}

impl From<[u8; NODE_ID_LENGTH]> for NodeId {
    fn from(inner: [u8; NODE_ID_LENGTH]) -> Self {
        NodeId { inner }
    }
}

impl TryFrom<&Vec<u8>> for NodeId {
    type Error = InvalidLengthError;
    fn try_from(inner: &Vec<u8>) -> Result<Self, InvalidLengthError> {
        if inner.len() != NODE_ID_LENGTH {
            return Err(InvalidLengthError {
                msg: format!(
                    "Invalid length: {}, NodeId requires {}.",
                    inner.len(),
                    NODE_ID_LENGTH
                ),
            });
        }
        Ok(Self::from(inner.as_ref()))
    }
}

impl<'a> From<&'a [u8]> for NodeId {
    fn from(it: &'a [u8]) -> Self {
        let mut inner = [0; NODE_ID_LENGTH];
        inner.copy_from_slice(it);

        NodeId { inner }
    }
}

impl<'a> From<Cow<'a, [u8]>> for NodeId {
    fn from(it: Cow<'a, [u8]>) -> Self {
        it.as_ref().into()
    }
}

#[inline]
fn hex_to_dec(hex: u8, s: &str) -> Result<u8, ParseError> {
    match hex {
        b'A'..=b'F' => Ok(hex - b'A' + 10),
        b'a'..=b'f' => Ok(hex - b'a' + 10),
        b'0'..=b'9' => Ok(hex - b'0'),
        _ => Err(ParseError::new(
            s,
            format!("expected hex chars, but got: `{}`", char::from(hex)),
        )),
    }
}

impl str::FromStr for NodeId {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, ParseError> {
        let bytes = s.as_bytes();

        // String representation is 2x the byte length + 2 extra for prefix
        if bytes.len() != 2 + NODE_ID_LENGTH * 2 {
            return Err(ParseError::new(s, "expected length is 42 chars"));
        }

        if bytes[0] != b'0' || bytes[1] != b'x' {
            return Err(ParseError::new(s, "expected 0x prefix"));
        }

        let mut inner = [0u8; NODE_ID_LENGTH];
        let mut p = 0;

        for b in bytes[2..].chunks(2) {
            let (hi, lo) = (hex_to_dec(b[0], s)?, hex_to_dec(b[1], s)?);
            inner[p] = (hi << 4) | lo;
            p += 1;
        }
        assert_eq!(p, NODE_ID_LENGTH);

        Ok(NodeId { inner })
    }
}

static HEX_CHARS: [u8; 16] = [
    b'0', b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9', b'a', b'b', b'c', b'd', b'e', b'f',
];

impl Serialize for NodeId {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        self.with_hex(|hex_str| serializer.serialize_str(hex_str))
    }
}

impl fmt::Debug for NodeId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl fmt::Display for NodeId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.with_hex(|hex_str| write!(f, "{}", hex_str))
    }
}

struct NodeIdVisit;

impl<'de> de::Visitor<'de> for NodeIdVisit {
    type Value = NodeId;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "a nodeId")
    }

    fn visit_str<E>(self, v: &str) -> Result<<Self as de::Visitor<'de>>::Value, E>
    where
        E: de::Error,
    {
        match NodeId::from_str(v) {
            Ok(node_id) => Ok(node_id),
            Err(_) => Err(de::Error::custom("invalid format")),
        }
    }

    fn visit_bytes<E>(self, v: &[u8]) -> Result<<Self as de::Visitor<'de>>::Value, E>
    where
        E: de::Error,
    {
        if v.len() == NODE_ID_LENGTH {
            let mut inner = [0u8; NODE_ID_LENGTH];
            inner.copy_from_slice(v);
            Ok(NodeId { inner })
        } else {
            Err(de::Error::custom("invalid format"))
        }
    }
}

impl<'de> Deserialize<'de> for NodeId {
    fn deserialize<D>(deserializer: D) -> Result<Self, <D as Deserializer<'de>>::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(NodeIdVisit)
    }
}

#[cfg(feature = "with-diesel")]
#[allow(dead_code)]
mod sql {
    use super::NodeId;
    use diesel::backend::Backend;
    use diesel::deserialize::FromSql;
    use diesel::expression::bound::Bound;
    use diesel::expression::AsExpression;
    use diesel::serialize::{IsNull, Output, ToSql};
    use diesel::sql_types::Text;
    use diesel::*;

    impl AsExpression<Text> for NodeId {
        type Expression = Bound<Text, String>;

        fn as_expression(self) -> Self::Expression {
            Bound::new(self.to_string())
        }
    }

    impl AsExpression<Text> for &NodeId {
        type Expression = Bound<Text, String>;

        fn as_expression(self) -> Self::Expression {
            Bound::new(self.to_string())
        }
    }

    impl<DB> FromSql<Text, DB> for NodeId
    where
        DB: Backend,
        String: FromSql<Text, DB>,
    {
        fn from_sql(bytes: Option<&<DB as Backend>::RawValue>) -> deserialize::Result<Self> {
            let s: String = FromSql::from_sql(bytes)?;
            Ok(s.parse()?)
        }
    }

    impl<DB> ToSql<Text, DB> for NodeId
    where
        DB: Backend,
        for<'b> &'b str: ToSql<Text, DB>,
    {
        fn to_sql<W: std::io::Write>(
            &self,
            out: &mut Output<'_, W, DB>,
        ) -> deserialize::Result<IsNull> {
            self.with_hex(move |s: &str| ToSql::<Text, DB>::to_sql(s, out))
        }
    }

    #[derive(FromSqlRow)]
    #[diesel(foreign_derive)]
    struct NodeIdProxy(NodeId);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryInto;

    #[test]
    fn parse_empty_str() {
        assert_eq!(
            "".parse::<NodeId>().unwrap_err().to_string(),
            "NodeId `` parsing error: expected length is 42 chars".to_string()
        );
    }

    #[test]
    fn parse_short_str() {
        assert_eq!(
            "short".parse::<NodeId>().unwrap_err().to_string(),
            "NodeId `short` parsing error: expected length is 42 chars".to_string()
        );
    }

    #[test]
    fn parse_long_str() {
        assert_eq!(
            "0123456789012345678901234567890123456789123"
                .parse::<NodeId>()
                .unwrap_err()
                .to_string(),
            "NodeId `0123456789012345678901234567890123456789123` parsing error: expected length is 42 chars".to_string()
        );
    }

    #[test]
    fn parse_wo_0x_str() {
        assert_eq!(
            "012345678901234567890123456789012345678912"
                .parse::<NodeId>()
                .unwrap_err()
                .to_string(),
            "NodeId `012345678901234567890123456789012345678912` parsing error: expected 0x prefix"
                .to_string()
        );
    }

    #[test]
    fn parse_non_hex_str() {
        assert_eq!(
            "0xx000000000000000000000000000000000000000"
                .parse::<NodeId>()
                .unwrap_err()
                .to_string(),
            "NodeId `0xx000000000000000000000000000000000000000` parsing error: expected hex chars, but got: `x`".to_string()
        );
    }

    #[test]
    fn parse_proper_str() {
        assert_eq!(
            "0xbabe000000000000000000000000000000000000"
                .parse::<NodeId>()
                .unwrap()
                .to_string(),
            "0xbabe000000000000000000000000000000000000".to_string()
        );
    }

    #[test]
    fn try_from_too_long_vec() {
        let test_vec: Vec<u8> = vec![
            0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
        ];
        let result: Result<NodeId, InvalidLengthError> = (&test_vec).try_into();
        assert_eq!(
            result.unwrap_err().to_string(),
            format!(
                "NodeId parsing error: Invalid length: 22, NodeId requires {}.",
                NODE_ID_LENGTH
            )
        );
    }
}
