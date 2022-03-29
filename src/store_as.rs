use std::{convert::TryFrom, marker::PhantomData};

use serde::{Deserialize, Serialize};

#[derive(Default, Debug)]
struct StoredAs<ValueType, StoredType>
where
    ValueType: Into<StoredType> + TryFrom<StoredType>,
{
    value: ValueType,
    stored: PhantomData<StoredType>,
}

impl<ValueType, StoredType> StoredAs<ValueType, StoredType>
where
    ValueType: Into<StoredType> + TryFrom<StoredType>,
{
    fn new(value: ValueType) -> Self {
        Self {
            value,
            stored: PhantomData::default(),
        }
    }
}

// impl<ValueType, StoredType> TryFrom<StoredType> for StoredAs<ValueType, StoredType> {
//     type Error;

//     fn try_from(value: StoredType) -> Result<Self, Self::Error> {
//         todo!()
//     }
// }

use serde::de;

impl<'de, ValueType, StoredType> Deserialize<'de> for StoredAs<ValueType, StoredType>
where
    ValueType: Into<StoredType> + TryFrom<StoredType>,
    StoredType: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        match StoredType::deserialize(deserializer) {
            Ok(stored) => match ValueType::try_from(stored) {
                Ok(value) => Ok(Self::new(value)),
                Err(_) => Err(de::Error::custom("failed to convert from given value")),
            },
            Err(error) => Err(de::Error::custom("not of stored type")),
        }
    }
}

impl<ValueType, StoredType> Serialize for StoredAs<ValueType, StoredType>
where
    ValueType: Into<StoredType> + TryFrom<StoredType>,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        todo!()
    }
}

impl<ValueType, StoredType> std::ops::Deref for StoredAs<ValueType, StoredType>
where
    ValueType: Into<StoredType> + TryFrom<StoredType>,
{
    type Target = ValueType;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}
