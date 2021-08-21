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

        fn expecting(&self, f: &mut Formatter) -> FmtResult {
            f.write_str("string or integer snowflake")
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
            value.parse().map(T::from).map_err(DeError::custom)
        }
    }
}
