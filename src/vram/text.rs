//! Module for tiled mode types and operations.

use super::*;

/// A screenblock entry for use in Text mode.
#[bitfield(bits = 16)]
#[repr(u16)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct TextScreenblockEntry {
  pub tile_id: B10,
  pub hflip: bool,
  pub vflip: bool,
  pub palbank: B4,
}

impl TextScreenblockEntry {
  /// Generates a default entry with the specified tile index.
  pub const fn from_tile_id(id: u16) -> Self {
    Self::new().with_tile_id(id)
  }
}

newtype! {
  /// A screenblock for use in Text mode.
  #[derive(Clone, Copy)]
  TextScreenblock, [TextScreenblockEntry; 32 * 32], no frills
}

#[test]
pub fn test_text_screen_block_size() {
  assert_eq!(core::mem::size_of::<TextScreenblock>(), 0x800);
}
