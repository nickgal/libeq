use std::any::Any;

use super::{
    fragment_ref, Fragment, FragmentRef, FragmentType, MeshAnimatedVerticesFragment,
    StringReference,
};

use nom::number::complete::le_u32;
use nom::sequence::tuple;
use nom::IResult;

#[derive(Debug)]
/// A reference to a [VertexColorFragment].
///
/// **Type ID:** 0x33
pub struct VertexColorReferenceFragment {
    pub name_reference: StringReference,

    /// The [MeshAnimatedVerticesFragment] reference.
    pub reference: FragmentRef<MeshAnimatedVerticesFragment>,

    /// _Unknown_ - Usually contains 0.
    pub flags: u32,
}

impl FragmentType for VertexColorReferenceFragment {
    type T = Self;

    const TYPE_ID: u32 = 0x33;

    fn parse(input: &[u8]) -> IResult<&[u8], VertexColorReferenceFragment> {
        let (remaining, (name_reference, reference, flags)) =
            tuple((StringReference::parse, fragment_ref, le_u32))(input)?;
        Ok((
            remaining,
            VertexColorReferenceFragment {
                name_reference,
                reference,
                flags,
            },
        ))
    }
}

impl Fragment for VertexColorReferenceFragment {
    fn serialize(&self) -> Vec<u8> {
        [
            &self.name_reference.serialize()[..],
            &self.reference.serialize()[..],
            &self.flags.to_le_bytes()[..],
        ]
        .concat()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn name_ref(&self) -> &StringReference {
        &self.name_reference
    }
}
