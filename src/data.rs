use std::fmt;

/// A byte buffer used for serialization to and from the plist data type.
///
/// You use it in types with derived `Serialize`/`Deserialize` traits.
///
/// ## Examples
///
/// ```rust
/// use serde::{Deserialize, Serialize};
///
/// # fn main() {
/// #[derive(Deserialize, Serialize)]
/// struct Info {
///     blob: plist::Data,
/// }
///
/// let actual = Info { blob: plist::Data::new(vec![1, 2, 3, 4]) };
///
/// let mut xml_byte_buffer: Vec<u8> = vec![];
/// plist::to_writer_xml(&mut xml_byte_buffer, &actual)
///     .expect("serialize into xml");
///
/// let expected: Info = plist::from_reader_xml(xml_byte_buffer.as_slice())
///     .expect("deserialize from xml");
///
/// assert_eq!(actual.blob, expected.blob);
/// # }
/// ```
#[derive(Clone, PartialEq, Eq)]
pub struct Data {
    inner: Vec<u8>,
}

impl Data {
    /// Creates a new `Data` from vec of bytes.
    pub fn new(bytes: Vec<u8>) -> Self {
        Data { inner: bytes }
    }
}

impl From<Vec<u8>> for Data {
    fn from(from: Vec<u8>) -> Self {
        Data { inner: from }
    }
}

impl From<Data> for Vec<u8> {
    fn from(from: Data) -> Self {
        from.inner
    }
}

impl AsRef<[u8]> for Data {
    fn as_ref(&self) -> &[u8] {
        self.inner.as_ref()
    }
}

impl AsMut<[u8]> for Data {
    fn as_mut(&mut self) -> &mut [u8] {
        self.inner.as_mut()
    }
}

impl fmt::Debug for Data {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.inner.fmt(f)
    }
}

pub mod serde_impls {
    use serde::{de, ser};
    use std::fmt;

    use crate::Data;

    impl ser::Serialize for Data {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ser::Serializer,
        {
            serializer.serialize_bytes(self.as_ref())
        }
    }

    struct DataVisitor;

    impl<'de> de::Visitor<'de> for DataVisitor {
        type Value = Data;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a byte array")
        }

        fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            self.visit_byte_buf(v.to_owned())
        }

        fn visit_byte_buf<E>(self, v: Vec<u8>) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(v.into())
        }
    }

    impl<'de> de::Deserialize<'de> for Data {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: de::Deserializer<'de>,
        {
            deserializer.deserialize_byte_buf(DataVisitor)
        }
    }
}
