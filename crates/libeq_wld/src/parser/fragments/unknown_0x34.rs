use std::any::Any;

use super::{Fragment, FragmentParser, StringReference, WResult};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, PartialEq)]
/// Unknown
///
/// **Type ID:** 0x34
pub struct Unknown0x34Fragment {
    pub name_reference: StringReference,
}

impl FragmentParser for Unknown0x34Fragment {
    type T = Self;

    const TYPE_ID: u32 = 0x34;
    const TYPE_NAME: &'static str = "Unknown0x34";

    fn parse(input: &[u8]) -> WResult<Self> {
        let (i, name_reference) = StringReference::parse(input)?;

        Ok((i, Self { name_reference }))
    }
}

impl Fragment for Unknown0x34Fragment {
    fn into_bytes(&self) -> Vec<u8> {
        [&self.name_reference.into_bytes()[..]].concat()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn name_ref(&self) -> &StringReference {
        &self.name_reference
    }

    fn type_id(&self) -> u32 {
        Self::TYPE_ID
    }
}