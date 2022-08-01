//! Module that holds stuff for the color blending ability.

use super::*;

/// Color Special Effects Selection (R/W)
pub const BLDCNT: VolAddress<ColorEffectSetting, Safe, Safe> =
  unsafe { VolAddress::new(0x400_0050) };

/// TODO: docs
#[bitfield(bits = 16)]
#[repr(u16)]
pub struct ColorEffectSetting {
  pub bg0_1st_target_pixel: bool,
  pub bg1_1st_target_pixel: bool,
  pub bg2_1st_target_pixel: bool,
  pub bg3_1st_target_pixel: bool,
  pub obj_1st_target_pixel: bool,
  pub backdrop_1st_target_pixel: bool,
  pub color_special_effect: ColorSpecialEffect,
  pub bg0_2nd_target_pixel: bool,
  pub bg1_2nd_target_pixel: bool,
  pub bg2_2nd_target_pixel: bool,
  pub bg3_2nd_target_pixel: bool,
  pub obj_2nd_target_pixel: bool,
  pub backdrop_2nd_target_pixel: bool,
  #[skip] __0: B2,
}

/// TODO: docs
#[derive(BitfieldSpecifier, Debug, Clone, Copy, PartialEq, Eq)]
#[bits = 2]
pub enum ColorSpecialEffect {
  /// TODO: docs
  None = 0,
  /// TODO: docs
  AlphaBlending = 1,
  /// TODO: docs
  BrightnessIncrease = 2,
  /// TODO: docs
  BrightnessDecrease = 3,
}

/// Alpha Blending Coefficients (R/W)
pub const BLDALPHA: VolAddress<AlphaBlendingSetting, Safe, Safe> =
  unsafe { VolAddress::new(0x400_0052) };

/// TODO: docs
#[bitfield(bits = 16)]
#[repr(u16)]
pub struct AlphaBlendingSetting {
  pub eva_coefficient: B5,
  #[skip] __0: B3,
  pub evb_coefficient: B5,
  #[skip] __1: B3,
}

/// Brightness (Fade-In/Out) Coefficient (W) (not R/W)
pub const BLDY: VolAddress<BrightnessSetting, Safe, Safe> = unsafe { VolAddress::new(0x400_0054) };

/// TODO: docs
#[bitfield(bits = 16)]
#[repr(u16)]
pub struct BrightnessSetting {
  pub evy_coefficient: B5,
  #[skip] __0: B11,
}
