//! Module for Background controls

use super::*;

/// BG0 Control. Read/Write. Display Mode 0/1 only.
pub const BG0CNT: VolAddress<BackgroundControlSetting, Safe, Safe> =
  unsafe { VolAddress::new(0x400_0008) };
/// BG1 Control. Read/Write. Display Mode 0/1 only.
pub const BG1CNT: VolAddress<BackgroundControlSetting, Safe, Safe> =
  unsafe { VolAddress::new(0x400_000A) };
/// BG2 Control. Read/Write. Display Mode 0/1/2 only.
pub const BG2CNT: VolAddress<BackgroundControlSetting, Safe, Safe> =
  unsafe { VolAddress::new(0x400_000C) };
/// BG3 Control. Read/Write.  Display Mode 0/2 only.
pub const BG3CNT: VolAddress<BackgroundControlSetting, Safe, Safe> =
  unsafe { VolAddress::new(0x400_000E) };

/// Allows configuration of a background layer.
///
/// Bits 0-1: BG Priority (lower number is higher priority, like an index)
/// Bits 2-3: Character Base Block (0 through 3, 16k each)
/// Bit 6: Mosaic mode
/// Bit 7: is 8bpp
/// Bit 8-12: Screen Base Block (0 through 31, 2k each)
/// Bit 13: Display area overflow wraps (otherwise transparent, affine BG only)
/// Bit 14-15: Screen Size
#[bitfield(bits = 16)]
#[repr(u16)]
pub struct BackgroundControlSetting {
  pub bg_priority: B2,
  pub char_base_block: B2,
  #[skip] __0: B2,
  pub mosaic: bool,
  pub is_8bpp: bool,
  pub screen_base_block: B5,
  pub affine_display_overflow_wrapping: bool,
  pub size: BGSize,
}

/// The size of a background.
///
/// The meaning changes depending on if the background is Text or Affine mode.
///
/// * In text mode, the screen base block determines where to start reading the
///   tile arrangement data (2k). Size Zero gives one screen block of use. Size
///   One and Two cause two of them to be used (horizontally or vertically,
///   respectively). Size Three is four blocks used, [0,1] above and then [2,3]
///   below. Each screen base block used is always a 32x32 tile grid.
/// * In affine mode, the screen base block determines where to start reading
///   data followed by the size of data as shown. The number of tiles varies
///   according to the size used.
#[derive(BitfieldSpecifier, Debug, Clone, Copy, PartialEq, Eq)]
#[bits = 2]
pub enum BGSize {
  /// * Text: 256x256px (2k)
  /// * Affine: 128x128px (256b)
  Zero = 0,
  /// * Text: 512x256px (4k)
  /// * Affine: 256x256px (1k)
  One = 1,
  /// * Text: 256x512px (4k)
  /// * Affine: 512x512px (4k)
  Two = 2,
  /// * Text: 512x512px (8k)
  /// * Affine: 1024x1024px (16k)
  Three = 3,
}

/// BG0 X-Offset. Write only. Text mode only. 9 bits.
pub const BG0HOFS: VolAddress<u16, (), Safe> = unsafe { VolAddress::new(0x400_0010) };
/// BG0 Y-Offset. Write only. Text mode only. 9 bits.
pub const BG0VOFS: VolAddress<u16, (), Safe> = unsafe { VolAddress::new(0x400_0012) };

/// BG1 X-Offset. Write only. Text mode only. 9 bits.
pub const BG1HOFS: VolAddress<u16, (), Safe> = unsafe { VolAddress::new(0x400_0014) };
/// BG1 Y-Offset. Write only. Text mode only. 9 bits.
pub const BG1VOFS: VolAddress<u16, (), Safe> = unsafe { VolAddress::new(0x400_0016) };

/// BG2 X-Offset. Write only. Text mode only. 9 bits.
pub const BG2HOFS: VolAddress<u16, (), Safe> = unsafe { VolAddress::new(0x400_0018) };
/// BG2 Y-Offset. Write only. Text mode only. 9 bits.
pub const BG2VOFS: VolAddress<u16, (), Safe> = unsafe { VolAddress::new(0x400_001A) };

/// BG3 X-Offset. Write only. Text mode only. 9 bits.
pub const BG3HOFS: VolAddress<u16, (), Safe> = unsafe { VolAddress::new(0x400_001C) };
/// BG3 Y-Offset. Write only. Text mode only. 9 bits.
pub const BG3VOFS: VolAddress<u16, (), Safe> = unsafe { VolAddress::new(0x400_001E) };

// TODO: fixed point format
pub const BG2PA: VolAddress<u16, (), Safe> = unsafe { VolAddress::new(0x400_0020) };
pub const BG2PB: VolAddress<u16, (), Safe> = unsafe { VolAddress::new(0x400_0022) };
pub const BG2PC: VolAddress<u16, (), Safe> = unsafe { VolAddress::new(0x400_0024) };
pub const BG2PD: VolAddress<u16, (), Safe> = unsafe { VolAddress::new(0x400_0026) };
pub const BG2X_L: VolAddress<u16, (), Safe> = unsafe { VolAddress::new(0x400_0028) };
pub const BG2X_H: VolAddress<u16, (), Safe> = unsafe { VolAddress::new(0x400_002A) };
pub const BG2Y_L: VolAddress<u16, (), Safe> = unsafe { VolAddress::new(0x400_002C) };
pub const BG2Y_H: VolAddress<u16, (), Safe> = unsafe { VolAddress::new(0x400_002E) };

pub const BG3PA: VolAddress<u16, (), Safe> = unsafe { VolAddress::new(0x400_0030) };
pub const BG3PB: VolAddress<u16, (), Safe> = unsafe { VolAddress::new(0x400_0032) };
pub const BG3PC: VolAddress<u16, (), Safe> = unsafe { VolAddress::new(0x400_0034) };
pub const BG3PD: VolAddress<u16, (), Safe> = unsafe { VolAddress::new(0x400_0036) };
pub const BG3X_L: VolAddress<u16, (), Safe> = unsafe { VolAddress::new(0x400_0038) };
pub const BG3X_H: VolAddress<u16, (), Safe> = unsafe { VolAddress::new(0x400_003A) };
pub const BG3Y_L: VolAddress<u16, (), Safe> = unsafe { VolAddress::new(0x400_003C) };
pub const BG3Y_H: VolAddress<u16, (), Safe> = unsafe { VolAddress::new(0x400_003E) };
