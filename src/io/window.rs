//! Module that holds stuff for the Window ability.

use super::*;

/// Window 0 Horizontal Dimensions (W)
pub const WIN0H: VolAddress<HorizontalWindowSetting, Safe, Safe> =
  unsafe { VolAddress::new(0x400_0040) };

/// Window 1 Horizontal Dimensions (W)
pub const WIN1H: VolAddress<HorizontalWindowSetting, Safe, Safe> =
  unsafe { VolAddress::new(0x400_0042) };

/// Allows control of the Window filters' horizontal dimensions.
/// * Bits 0-7: Right boundary pixel of window (exclusive)
/// * Bits 8-15: Left boundary pixel of window (inclusive)
#[bitfield(bits = 16)]
#[repr(u16)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct HorizontalWindowSetting {
  pub col_end: B8,
  pub col_start: B8,
}

/// Window 0 Vertical Dimensions (W)
pub const WIN0V: VolAddress<VerticalWindowSetting, Safe, Safe> =
  unsafe { VolAddress::new(0x400_0044) };

/// Window 1 Vertical Dimensions (W)
pub const WIN1V: VolAddress<VerticalWindowSetting, Safe, Safe> =
  unsafe { VolAddress::new(0x400_0046) };

/// Allows control of the Window filters' vertical dimensions.
/// * Bits 0-7: Bottom boundary pixel of window (exclusive)
/// * Bits 8-15: Top boundary pixel of window (inclusive)
#[bitfield(bits = 16)]
#[repr(u16)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct VerticalWindowSetting {
  pub row_end: B8,
  pub row_start: B8,
}

/// Control of Inside of Window(s) (R/W)
pub const WININ: VolAddress<InsideWindowSetting, Safe, Safe> =
  unsafe { VolAddress::new(0x400_0048) };

/// TODO: docs
#[bitfield(bits = 16)]
#[repr(u16)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct InsideWindowSetting {
  pub win0_bg0: bool,
  pub win0_bg1: bool,
  pub win0_bg2: bool,
  pub win0_bg3: bool,
  pub win0_obj: bool,
  pub win0_color_special: bool,
  #[skip] __0: B2,
  pub win1_bg0: bool,
  pub win1_bg1: bool,
  pub win1_bg2: bool,
  pub win1_bg3: bool,
  pub win1_obj: bool,
  pub win1_color_special: bool,
  #[skip] __1: B2,
}

///  Control of Outside of Windows & Inside of OBJ Window (R/W)
pub const WINOUT: VolAddress<OutsideWindowSetting, Safe, Safe> =
  unsafe { VolAddress::new(0x400_004A) };

/// TODO:
#[bitfield(bits = 16)]
#[repr(u16)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct OutsideWindowSetting {
  pub outside_bg0: bool,
  pub outside_bg1: bool,
  pub outside_bg2: bool,
  pub outside_bg3: bool,
  pub outside_obj: bool,
  pub outside_color_special: bool,
  #[skip] __0: B2,
  pub obj_win_bg0: bool,
  pub obj_win_bg1: bool,
  pub obj_win_bg2: bool,
  pub obj_win_bg3: bool,
  pub obj_win_obj: bool,
  pub obj_win_color_special: bool,
  #[skip] __1: B2,
}
