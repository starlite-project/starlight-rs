use bincode::serialize;
use serde::{Deserialize, Serialize};
use serde_cbor::to_vec;
use std::{error::Error, fmt::{Display, Formatter, Result as FmtResult}};

mod string {
    use serde::{
        de::{Deserializer, Error as DeError, Visitor},
        ser::Serializer,
    };
    use std::{
        fmt::{Display, Formatter, Result as FmtResult},
        marker::PhantomData,
    };

    struct IdVisitor<T: From<u64>>(PhantomData<T>);

    impl<'de, T: From<u64>> Visitor<'de> for IdVisitor<T> {
        type Value = T;

        fn expecting(&self, formatter: &mut Formatter) -> FmtResult {
            formatter.write_str("string or integer snowflake")
        }

        fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
        where
            E: DeError,
        {
            Ok(T::from(value))
        }

        fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
        where
            E: DeError,
        {
            value.parse().map(T::from).map_err(E::custom)
        }
    }

    pub fn serialize<T: Display, S: Serializer>(
        value: &T,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        serializer.collect_str(value)
    }

    pub fn deserialize<'de, T: From<u64>, D: Deserializer<'de>>(deserializer:D) -> Result<T, D::Error> {
        deserializer.deserialize_any(IdVisitor(PhantomData))
    }
}

#[derive(
    Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
)]
struct Id(#[serde(with = "string")] pub u64);

impl Display for Id {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        Display::fmt(&self.0, f)
    }
}
fn main() -> Result<(), Box<dyn Error>> {
    let id = Id::default();

    dbg!(serialize(&id).unwrap());

    dbg!(format!("at:{}", id).into_bytes());

    Ok(())
}
