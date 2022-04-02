use std::any::Any;

use super::{
    fragment_ref, Fragment, FragmentRef, FragmentType, SkeletonTrackSetFragment, StringHash,
};

use nom::number::complete::le_u32;
use nom::sequence::tuple;
use nom::IResult;

#[derive(Debug)]
/// A reference to a [SkeletonTrackSetFragment].
///
/// **Type ID:** 0x11
pub struct SkeletonTrackSetReferenceFragment {
    /// The [SkeletonTrackSetFragment] reference.
    pub reference: FragmentRef<SkeletonTrackSetFragment>,

    /// _Unknown_ Seems to always contain 0.
    pub params1: u32,
}

impl FragmentType for SkeletonTrackSetReferenceFragment {
    type T = Self;

    const TYPE_ID: u32 = 0x11;

    fn parse(input: &[u8]) -> IResult<&[u8], SkeletonTrackSetReferenceFragment> {
        let (remaining, (reference, params1)) = tuple((fragment_ref, le_u32))(input)?;
        Ok((
            remaining,
            SkeletonTrackSetReferenceFragment { reference, params1 },
        ))
    }
}

impl Fragment for SkeletonTrackSetReferenceFragment {
    fn serialize(&self) -> Vec<u8> {
        [
            &self.reference.serialize()[..],
            &self.params1.to_le_bytes()[..],
        ]
        .concat()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn name(&self, string_hash: &StringHash) -> String {
        String::new()
    }
}
