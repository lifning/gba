//! Types and declarations for the Object Attribute Memory (`OAM`).

use super::*;

/// 0th part of an object's attributes.
///
/// * Bits 0-7: row-coordinate
/// * Bits 8-9: Rendering style: Normal, Affine, Disabled, Double Area Affine
/// * Bits 10-11: Object mode: Normal, SemiTransparent, Object Window
/// * Bit 12: Mosaic
/// * Bit 13: is 8bpp
/// * Bits 14-15: Object Shape: Square, Horizontal, Vertical
#[bitfield(bits = 16)]
#[repr(u16)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct OBJAttr0 {
  pub row_coordinate: B8,
  pub obj_rendering: ObjectRender,
  pub obj_mode: ObjectMode,
  pub mosaic: bool,
  pub is_8bpp: bool,
  pub obj_shape: ObjectShape,
}

/// What style of rendering for this object
#[derive(BitfieldSpecifier, Debug, Clone, Copy, PartialEq, Eq)]
#[bits = 2]
pub enum ObjectRender {
  /// Standard, non-affine rendering
  Normal = 0,
  /// Affine rendering
  Affine = 1,
  /// Object disabled (saves cycles for elsewhere!)
  Disabled = 2,
  /// Affine with double render space (helps prevent clipping)
  DoubleAreaAffine = 3,
}

/// What mode to ues for the object.
#[derive(BitfieldSpecifier, Debug, Clone, Copy, PartialEq, Eq)]
#[bits = 2]
pub enum ObjectMode {
  /// Show the object normally
  Normal = 0,
  /// The object becomes the "Alpha Blending 1st target" (see Alpha Blending)
  SemiTransparent = 1,
  /// Use the object's non-transparent pixels as part of a mask for the object
  /// window (see Windows).
  OBJWindow = 2,
}

/// What shape the object's appearance should be.
///
/// The specifics also depend on the `ObjectSize` set.
#[derive(BitfieldSpecifier, Debug, Clone, Copy, PartialEq, Eq)]
#[bits = 2]
pub enum ObjectShape {
  /// Equal parts wide and tall
  Square = 0,
  /// Wider than tall
  Horizontal = 1,
  /// Taller than wide
  Vertical = 2,
}

/// 1st part of an object's attributes.
///
/// * Bits 0-8: column coordinate
/// * Bits 9-13:
///   * Normal render: Bit 12 holds hflip and 13 holds vflip.
///   * Affine render: The affine parameter selection.
/// * Bits 14-15: Object Size
#[bitfield(bits = 16)]
#[repr(u16)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct OBJAttr1 {
  pub col_coordinate: B9,
  pub affine_index: B5,
  pub obj_size: ObjectSize,
}

impl OBJAttr1 {
  pub const fn hflip(&self) -> bool {
    (self.affine_index() & 0b01000) != 0
  }
  pub const fn vflip(&self) -> bool {
    (self.affine_index() & 0b10000) != 0
  }
  pub const fn with_hflip(self, hflip: bool) -> Self {
    if hflip {
      self.with_affine_index(self.affine_index() | 0b01000)
    } else {
      self.with_affine_index(self.affine_index() & 0b10111)
    }
  }
  pub const fn with_vflip(self, vflip: bool) -> Self {
    if vflip {
      self.with_affine_index(self.affine_index() | 0b10000)
    } else {
      self.with_affine_index(self.affine_index() & 0b01111)
    }
  }
  pub fn set_hflip(&mut self, hflip: bool) {
    if hflip {
      self.set_affine_index(self.affine_index() | 0b01000)
    } else {
      self.set_affine_index(self.affine_index() & 0b10111)
    }
  }
  pub fn set_vflip(&mut self, vflip: bool) {
    if vflip {
      self.set_affine_index(self.affine_index() | 0b10000)
    } else {
      self.set_affine_index(self.affine_index() & 0b01111)
    }
  }
}

/// The object's size.
///
/// Also depends on the `ObjectShape` set.
#[derive(BitfieldSpecifier, Debug, Clone, Copy, PartialEq, Eq)]
#[bits = 2]
pub enum ObjectSize {
  /// * Square: 8x8px
  /// * Horizontal: 16x8px
  /// * Vertical: 8x16px
  Zero = 0,
  /// * Square: 16x16px
  /// * Horizontal: 32x8px
  /// * Vertical: 8x32px
  One = 1,
  /// * Square: 32x32px
  /// * Horizontal: 32x16px
  /// * Vertical: 16x32px
  Two = 2,
  /// * Square: 64x64px
  /// * Horizontal: 64x32px
  /// * Vertical: 32x64px
  Three = 3,
}

/// 2nd part of an object's attributes.
///
/// * Bits 0-9: Base Tile Index (tile offset from CBB4)
/// * Bits 10-11: Priority
/// * Bits 12-15: Palbank (if using 4bpp)
#[bitfield(bits = 16)]
#[repr(u16)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct OBJAttr2 {
  pub tile_id: B10,
  pub priority: B2,
  pub palbank: B4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ObjectAttributes {
  pub attr0: OBJAttr0,
  pub attr1: OBJAttr1,
  pub attr2: OBJAttr2,
}

/// The object attributes, but there are gaps in the array, so we must not
/// expose this directly.
const OBJ_ATTR_APPROX: VolBlock<[u16; 4], Safe, Safe, 128> = unsafe { VolBlock::new(0x700_0000) };
// TODO: VolSeries

pub fn write_obj_attributes(slot: usize, attributes: ObjectAttributes) -> Option<()> {
  OBJ_ATTR_APPROX.get(slot).map(|va| unsafe {
    let va_u16 = va.cast::<u16>();
    va_u16.cast::<OBJAttr0>().write(attributes.attr0);
    va_u16.offset(1).cast::<OBJAttr1>().write(attributes.attr1);
    va_u16.offset(2).cast::<OBJAttr2>().write(attributes.attr2);
  })
}

pub fn read_obj_attributes(slot: usize) -> Option<ObjectAttributes> {
  OBJ_ATTR_APPROX.get(slot).map(|va| unsafe {
    let va_u16 = va.cast::<u16>();
    let attr0 = va_u16.cast::<OBJAttr0>().read();
    let attr1 = va_u16.offset(1).cast::<OBJAttr1>().read();
    let attr2 = va_u16.offset(2).cast::<OBJAttr2>().read();
    ObjectAttributes { attr0, attr1, attr2 }
  })
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AffineParameters {
  pub pa: i16,
  pub pb: i16,
  pub pc: i16,
  pub pd: i16,
}
// TODO: find the correct fixed-point type here.

/// The object attributes, but there are gaps in the array, so we must not
/// expose this directly.
const AFFINE_PARAMS_APPROX: VolBlock<[i16; 16], Safe, Safe, 32> =
  unsafe { VolBlock::new(0x700_0000) };
// TODO: VolSeries

pub fn write_affine_parameters(slot: usize, params: AffineParameters) -> Option<()> {
  AFFINE_PARAMS_APPROX.get(slot).map(|va| unsafe {
    let va_i16 = va.cast::<i16>();
    va_i16.offset(3).write(params.pa);
    va_i16.offset(7).write(params.pb);
    va_i16.offset(11).write(params.pc);
    va_i16.offset(15).write(params.pd);
  })
}

pub fn read_affine_parameters(slot: usize) -> Option<AffineParameters> {
  AFFINE_PARAMS_APPROX.get(slot).map(|va| unsafe {
    let va_i16 = va.cast::<i16>();
    let pa = va_i16.offset(3).read();
    let pb = va_i16.offset(7).read();
    let pc = va_i16.offset(11).read();
    let pd = va_i16.offset(15).read();
    AffineParameters { pa, pb, pc, pd }
  })
}
