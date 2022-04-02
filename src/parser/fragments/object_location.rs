use std::any::Any;

use super::{Fragment, FragmentType};

use nom::number::complete::{le_f32, le_u32};
use nom::sequence::tuple;
use nom::IResult;

#[derive(Debug)]
/// **Type ID:** 0x15
pub struct ObjectLocationFragment {
    /// Typically 0x2E when used in main zone files and 0x32E when
    /// used for placeable objects.
    pub flags: u32,

    /// When used in main zone files, points to a 0x16 fragment.
    /// When used for placeable objects, seems to always contain 0.
    /// This might be due to the difference in the Flags value.
    pub fragment1: u32,

    /// When used in main zone files, contains the minimum X value of the
    /// entire zone. When used for placeable objects, contains the X value
    /// of the object’s location.
    pub x: f32,

    /// When used in main zone files, contains the minimum Y value of the
    /// entire zone. When used for placeable objects, contains the Y value
    /// of the object’s location.
    pub y: f32,

    /// When used in main zone files, contains the minimum Z value of the
    /// entire zone. When used for placeable objects, contains the Z value
    /// of the object’s location.
    pub z: f32,

    /// When used in main zone files, typically contains 0. When used for
    /// placeable objects, contains a value describing rotation around the Z
    /// axis, scaled as Degrees x (512 / 360).
    pub rotate_z: f32,

    /// When used in main zone files, typically contains 0. When used for
    /// placeable objects, contains a value describing rotation around the Y
    /// axis, scaled as Degrees x (512 / 360).
    pub rotate_y: f32,

    /// When used in main zone files, typically contains 0. When used for
    /// placeable objects, contains a value describing rotation around the X
    /// axis, scaled as Degrees x (512 / 360).
    pub rotate_x: f32,

    /// _Unknown_ - Typically contains 0 (though might be more significant for placeable objects).
    pub params1: u32,

    /// When used in main zone files, typically contains 0.5. When used for
    /// placeable objects, contains the object’s scaling factor in the Y direction
    /// (e.g. 2.0 would make the object twice as big in the Y direction).
    pub scale_y: f32,

    /// When used in main zone files, typically contains 0.5. When used for
    /// placeable objects, contains the object’s scaling factor in the X direction
    /// (e.g. 2.0 would make the object twice as big in the X direction).
    pub scale_x: f32,

    /// When used in main zone files, typically contains 0 (might be related to
    /// the Flags value). When used for placeable objects, points to a 0x33 Vertex
    /// Color Reference fragment.
    pub fragment2: u32,

    /// Typically contains 30 when used in main zone files and 0 when used for
    /// placeable objects. This field only exists if `fragment2` points to a fragment.
    pub params2: u32,
}

impl FragmentType for ObjectLocationFragment {
    type T = Self;

    const TYPE_ID: u32 = 0x15;

    fn parse(input: &[u8]) -> IResult<&[u8], ObjectLocationFragment> {
        let (
            remaining,
            (
                flags,
                fragment1,
                x,
                y,
                z,
                rotate_z,
                rotate_y,
                rotate_x,
                params1,
                scale_y,
                scale_x,
                fragment2,
                params2,
            ),
        ) = tuple((
            le_u32, le_u32, le_f32, le_f32, le_f32, le_f32, le_f32, le_f32, le_u32, le_f32, le_f32,
            le_u32, le_u32,
        ))(input)?;
        Ok((
            remaining,
            ObjectLocationFragment {
                flags,
                fragment1,
                x,
                y,
                z,
                rotate_z,
                rotate_y,
                rotate_x,
                params1,
                scale_y,
                scale_x,
                fragment2,
                params2,
            },
        ))
    }
}

impl Fragment for ObjectLocationFragment {
    fn serialize(&self) -> Vec<u8> {
        [
            &self.flags.to_le_bytes()[..],
            &self.fragment1.to_le_bytes()[..],
            &self.x.to_le_bytes()[..],
            &self.y.to_le_bytes()[..],
            &self.z.to_le_bytes()[..],
            &self.rotate_z.to_le_bytes()[..],
            &self.rotate_y.to_le_bytes()[..],
            &self.rotate_x.to_le_bytes()[..],
            &self.params1.to_le_bytes()[..],
            &self.scale_y.to_le_bytes()[..],
            &self.scale_x.to_le_bytes()[..],
            &self.fragment2.to_le_bytes()[..],
            &self.params2.to_le_bytes()[..],
        ]
        .concat()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
