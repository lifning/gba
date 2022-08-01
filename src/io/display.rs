//! Contains types and definitions for display related IO registers.

use super::*;

/// LCD Control. Read/Write.
///
/// The "force vblank" bit is always set when your Rust code first executes.
pub const DISPCNT: VolAddress<DisplayControlSetting, Safe, Safe> =
  unsafe { VolAddress::new(0x400_0000) };

/// Setting for the display control register.
///
/// * 0-2: `DisplayMode`
/// * 3: CGB mode flag
/// * 4: Display frame 1 (Modes 4/5 only)
/// * 5: "hblank interval free", allows full access to OAM during hblank
/// * 6: Object tile memory 1-dimensional
/// * 7: Force vblank
/// * 8: Display bg0 layer
/// * 9: Display bg1 layer
/// * 10: Display bg2 layer
/// * 11: Display bg3 layer
/// * 12: Display objects layer
/// * 13: Window 0 display
/// * 14: Window 1 display
/// * 15: Object window
#[allow(missing_docs)]
#[bitfield(bits = 16)]
#[repr(u16)]
#[derive(Copy, Clone)]
pub struct DisplayControlSetting {
  pub mode: DisplayMode,
  #[skip] __0: bool,
  pub frame1: bool,
  pub hblank_interval_free: bool,
  pub oam_memory_1d: bool,
  pub force_vblank: bool,
  pub bg0: bool,
  pub bg1: bool,
  pub bg2: bool,
  pub bg3: bool,
  pub obj: bool,
  pub win0: bool,
  pub win1: bool,
  pub obj_window: bool,
}

/// The six display modes available on the GBA.
#[derive(BitfieldSpecifier, Debug, Clone, Copy, PartialEq, Eq)]
#[bits = 3]
pub enum DisplayMode {
  /// * Affine: No
  /// * Layers: 0/1/2/3
  /// * Size(px): 256x256 to 512x512
  /// * Tiles: 1024
  /// * Palette Modes: 4bpp or 8bpp
  Mode0 = 0,
  /// * BG0 / BG1: As Mode0
  /// * BG2: As Mode2
  Mode1 = 1,
  /// * Affine: Yes
  /// * Layers: 2/3
  /// * Size(px): 128x128 to 1024x1024
  /// * Tiles: 256
  /// * Palette Modes: 8bpp
  Mode2 = 2,
  /// * Affine: Yes
  /// * Layers: 2
  /// * Size(px): 240x160 (1 page)
  /// * Bitmap
  /// * Full Color
  Mode3 = 3,
  /// * Affine: Yes
  /// * Layers: 2
  /// * Size(px): 240x160 (2 pages)
  /// * Bitmap
  /// * Palette Modes: 8bpp
  Mode4 = 4,
  /// * Affine: Yes
  /// * Layers: 2
  /// * Size(px): 160x128 (2 pages)
  /// * Bitmap
  /// * Full Color
  Mode5 = 5,
}

/// Assigns the given display control setting.
pub fn set_display_control(setting: DisplayControlSetting) {
  DISPCNT.write(setting);
}
/// Obtains the current display control setting.
pub fn display_control() -> DisplayControlSetting {
  DISPCNT.read()
}

/// Display Status and IRQ Control. Read/Write.
pub const DISPSTAT: VolAddress<DisplayStatusSetting, Safe, Safe> =
  unsafe { VolAddress::new(0x400_0004) };

/// Display status and interrupt control values.
#[bitfield(bits = 16)]
#[repr(u16)]
pub struct DisplayStatusSetting {
  pub vblank_flag: bool,
  pub hblank_flag: bool,
  pub vcounter_flag: bool,
  pub vblank_irq_enable: bool,
  pub hblank_irq_enable: bool,
  pub vcounter_irq_enable: bool,
  #[skip] __0: B2,
  pub vcount_setting: B8,
}

/// Vertical Counter (LY). Read only.
///
/// Gives the current scanline that the display controller is working on. If
/// this is at or above the `VBLANK_SCANLINE` value then the display controller
/// is in a "vertical blank" period.
pub const VCOUNT: VolAddress<u16, Safe, ()> = unsafe { VolAddress::new(0x400_0006) };

/// If the `VCOUNT` register reads equal to or above this then you're in vblank.
pub const VBLANK_SCANLINE: u16 = 160;

/// Global mosaic effect control. Write-only.
pub const MOSAIC: VolAddress<MosaicSetting, Safe, Safe> = unsafe { VolAddress::new(0x400_004C) };

/// Allows control of the Mosaic effect.
///
/// Values are the _increase_ for each top-left pixel to be duplicated in the
/// final result. If you want to duplicate some other pixel than the top-left,
/// you can offset the background or object by an appropriate amount.
///
/// 0) No effect (1+0)
/// 1) Each pixel becomes 2 pixels (1+1)
/// 2) Each pixel becomes 3 pixels (1+2)
/// 3) Each pixel becomes 4 pixels (1+3)
///
/// * Bits 0-3: BG mosaic horizontal increase
/// * Bits 4-7: BG mosaic vertical increase
/// * Bits 8-11: Object mosaic horizontal increase
/// * Bits 12-15: Object mosaic vertical increase
#[bitfield(bits = 16)]
#[repr(u16)]
pub struct  MosaicSetting {
  pub bg_horizontal_inc: B4,
  pub bg_vertical_inc: B4,
  pub obj_horizontal_inc: B4,
  pub obj_vertical_inc: B4,
}
