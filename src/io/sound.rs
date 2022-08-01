//! Module for sound registers.

use super::*;

//TODO within these "read/write" registers only some bits are actually read/write!

/// Sound Channel 1 Sweep Register (`NR10`). Read/Write.
pub const SOUND1CNT_L: VolAddress<SweepRegisterSetting, Safe, Safe> =
  unsafe { VolAddress::new(0x400_0060) };

/// TODO: docs
#[bitfield(bits = 16)]
#[repr(u16)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct SweepRegisterSetting {
  pub sweep_shift: B3,
  pub sweep_decreasing: bool,
  /// units of 7.8ms (0-7, min=7.8ms, max=54.7ms)
  pub sweep_time: B3,
  #[skip] __0: B9,
}

/// Sound Channel 1 Duty/Length/Envelope (`NR11`, `NR12`). Read/Write.
pub const SOUND1CNT_H: VolAddress<DutyLenEnvelopeSetting, Safe, Safe> =
  unsafe { VolAddress::new(0x400_0062) };

/// TODO: docs
#[bitfield(bits = 16)]
#[repr(u16)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct DutyLenEnvelopeSetting {
  /// units of (64-n)/256s
  pub sound_length: B6,
  pub wave_pattern_duty: WaveDuty,
  /// units of n/64s
  pub envelope_step_time: B3,
  pub envelope_increasing: bool,
  pub initial_envelope_volume: B4,
}

#[derive(BitfieldSpecifier, Debug, Clone, Copy, PartialEq, Eq)]
#[bits = 2]
pub enum WaveDuty {
  /// ` -_______-_______-_______ `
  OneEighth = 0,
  /// ` --______--______--______ `
  OneQuarter = 1,
  /// ` ----____----____----____ ` (normal)
  OneHalf = 2,
  /// ` ------__------__------__ `
  ThreeQuarters = 3,
}

/// Sound Channel 1 Frequency/Control (`NR13`, `NR14`). Read/Write.
pub const SOUND1CNT_X: VolAddress<FrequencyControlSetting, Safe, Safe> =
  unsafe { VolAddress::new(0x400_0064) };

/// TODO: docs
#[bitfield(bits = 16)]
#[repr(u16)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct FrequencyControlSetting {
  /// 131072/(2048-n)Hz  (0-2047)
  pub frequency: B11,
  #[skip] __0: B3,
  /// Stop output when [DutyLenEnvelopeSetting::sound_length] expires
  pub length_flag: bool,
  /// Restart Sound
  pub is_initial: bool,
}

/// Sound Channel 2 Channel 2 Duty/Length/Envelope (`NR21`, `NR22`). Read/Write.
pub const SOUND2CNT_L: VolAddress<DutyLenEnvelopeSetting, Safe, Safe> =
  unsafe { VolAddress::new(0x400_0068) };

/// Sound Channel 2 Frequency/Control (`NR23`, `NR24`). Read/Write.
pub const SOUND2CNT_H: VolAddress<FrequencyControlSetting, Safe, Safe> =
  unsafe { VolAddress::new(0x400_006C) };

/// Sound Channel 3 Stop/Wave RAM select (`NR23`, `NR24`). Read/Write.
pub const SOUND3CNT_L: VolAddress<StopWaveRAMSelectSetting, Safe, Safe> =
  unsafe { VolAddress::new(0x400_0070) };

/// TODO: docs
#[bitfield(bits = 16)]
#[repr(u16)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct StopWaveRAMSelectSetting {
  #[skip] __0: B5,
  pub wave_ram_dimension_2d: bool,
  pub wave_ram_bank_number: bool,
  pub sound_channel_3_playing: bool,
  #[skip] __1: B8,
}

/// Sound Channel 3 Length/Volume (`NR23`, `NR24`). Read/Write.
pub const SOUND3CNT_H: VolAddress<LengthVolumeSetting, Safe, Safe> =
  unsafe { VolAddress::new(0x400_0072) };

/// TODO: docs
#[bitfield(bits = 16)]
#[repr(u16)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct LengthVolumeSetting {
  pub sound_length: B8,
  #[skip] __0: B5,
  pub sound_volume: B2,
  pub force_75percent: bool,
}

/// Sound Channel 3 Frequency/Control (`NR33`, `NR34`). Read/Write.
pub const SOUND3CNT_X: VolAddress<FrequencyControlSetting, Safe, Safe> =
  unsafe { VolAddress::new(0x400_0074) };

/// Channel 3 Wave Pattern RAM (W/R)
pub const WAVE_RAM0_L: VolAddress<u16, Safe, Safe> = unsafe { VolAddress::new(0x400_0090) };
/// Channel 3 Wave Pattern RAM (W/R)
pub const WAVE_RAM0_H: VolAddress<u16, Safe, Safe> = unsafe { VolAddress::new(0x400_0092) };
/// Channel 3 Wave Pattern RAM (W/R)
pub const WAVE_RAM1_L: VolAddress<u16, Safe, Safe> = unsafe { VolAddress::new(0x400_0094) };
/// Channel 3 Wave Pattern RAM (W/R)
pub const WAVE_RAM1_H: VolAddress<u16, Safe, Safe> = unsafe { VolAddress::new(0x400_0096) };
/// Channel 3 Wave Pattern RAM (W/R)
pub const WAVE_RAM2_L: VolAddress<u16, Safe, Safe> = unsafe { VolAddress::new(0x400_0098) };
/// Channel 3 Wave Pattern RAM (W/R)
pub const WAVE_RAM2_H: VolAddress<u16, Safe, Safe> = unsafe { VolAddress::new(0x400_009A) };
/// Channel 3 Wave Pattern RAM (W/R)
pub const WAVE_RAM3_L: VolAddress<u16, Safe, Safe> = unsafe { VolAddress::new(0x400_009C) };
/// Channel 3 Wave Pattern RAM (W/R)
pub const WAVE_RAM3_H: VolAddress<u16, Safe, Safe> = unsafe { VolAddress::new(0x400_009E) };

/// Sound Channel 4 Length/Envelope (`NR41`, `NR42`). Read/Write.
pub const SOUND4CNT_L: VolAddress<LengthEnvelopeSetting, Safe, Safe> =
  unsafe { VolAddress::new(0x400_0078) };

/// TODO: docs
#[bitfield(bits = 16)]
#[repr(u16)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct LengthEnvelopeSetting {
  pub sound_length: B6,
  #[skip] __0: B2,
  pub envelope_step_time: B3,
  pub envelope_increasing: bool,
  pub initial_envelope_volume: B4,
}

/// Sound Channel 4 Frequency/Control (`NR43`, `NR44`). Read/Write.
pub const SOUND4CNT_H: VolAddress<NoiseFrequencySetting, Safe, Safe> =
  unsafe { VolAddress::new(0x400_007C) };

#[bitfield(bits = 16)]
#[repr(u16)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct NoiseFrequencySetting {
  pub frequency_divide_ratio: B3,
  pub counter_step_width_7bit: bool,
  pub shift_clock_frequency: B4,
  #[skip] __0: B6,
  pub length_flag_stop: bool,
  pub initial_restart: bool,
}

// TODO: unify FIFO as

/// Sound A FIFO, Data 0 and Data 1 (W)
pub const FIFO_A_L: VolAddress<u16, Safe, Safe> = unsafe { VolAddress::new(0x400_00A0) };
/// Sound A FIFO, Data 2 and Data 3 (W)
pub const FIFO_A_H: VolAddress<u16, Safe, Safe> = unsafe { VolAddress::new(0x400_00A2) };
/// Sound B FIFO, Data 0 and Data 1 (W)
pub const FIFO_B_L: VolAddress<u16, Safe, Safe> = unsafe { VolAddress::new(0x400_00A4) };
/// Sound B FIFO, Data 2 and Data 3 (W)
pub const FIFO_B_H: VolAddress<u16, Safe, Safe> = unsafe { VolAddress::new(0x400_00A6) };

/// Channel L/R Volume/Enable (`NR50`, `NR51`). Read/Write.
pub const SOUNDCNT_L: VolAddress<NonWaveVolumeEnableSetting, Safe, Safe> =
  unsafe { VolAddress::new(0x400_0080) };

/// TODO: docs
#[bitfield(bits = 16)]
#[repr(u16)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct NonWaveVolumeEnableSetting {
  pub right_master_volume: B3,
  #[skip] __0: bool,
  pub left_master_volume: B3,
  #[skip] __1: bool,
  pub right_enable_flags: SoundEnableFlags,
  pub left_enable_flags: SoundEnableFlags,
}

#[bitfield(bits = 4)]
#[derive(BitfieldSpecifier, Debug, Copy, Clone, PartialEq, Eq)]
pub struct SoundEnableFlags {
  pub sound1: bool,
  pub sound2: bool,
  pub sound3: bool,
  pub sound4: bool,
}

/// DMA Sound Control/Mixing. Read/Write.
pub const SOUNDCNT_H: VolAddress<WaveVolumeEnableSetting, Safe, Safe> =
  unsafe { VolAddress::new(0x400_0082) };

/// TODO: docs
#[bitfield(bits = 16)]
#[repr(u16)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct WaveVolumeEnableSetting {
  pub sound_number_volume: NumberSoundVolume,
  pub dma_sound_a_full_volume: bool,
  pub dma_sound_b_full_volume: bool,
  #[skip] __0: B4,
  pub dma_sound_a_enable_right: bool,
  pub dma_sound_a_enable_left: bool,
  pub dma_sound_a_timer_select: bool,
  pub dma_sound_a_reset_fifo: bool,
  pub dma_sound_b_enable_right: bool,
  pub dma_sound_b_enable_left: bool,
  pub dma_sound_b_timer_select: bool,
  pub dma_sound_b_reset_fifo: bool,
}

/// TODO: docs
#[derive(BitfieldSpecifier, Debug, Clone, Copy, PartialEq, Eq)]
#[bits = 2]
enum NumberSoundVolume {
  /// TODO: docs
  Quarter = 0,
  /// TODO: docs
  Half = 1,
  /// TODO: docs
  Full = 2,
}

/// Sound on/off (`NR52`). Read/Write.
pub const SOUNDCNT_X: VolAddress<SoundMasterSetting, Safe, Safe> =
  unsafe { VolAddress::new(0x400_0084) };

/// TODO: docs
#[bitfield(bits = 16)]
#[repr(u16)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct SoundMasterSetting {
  pub sound1_on: bool,
  pub sound2_on: bool,
  pub sound3_on: bool,
  pub sound4_on: bool,
  #[skip] __0: B3,
  pub psg_fifo_master_enabled: bool,
  #[skip] __1: B8,
}

/// Sound on/off (`NR52`). Read/Write.
pub const SOUNDBIAS: VolAddress<SoundPWMSetting, Safe, Safe> =
  unsafe { VolAddress::new(0x400_0088) };

/// TODO: docs
#[bitfield(bits = 16)]
#[repr(u16)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct SoundPWMSetting {
  #[skip] __0: bool,
  pub bias_level: B9,
  #[skip] __1: B4,
  pub amplitude_resolution: AmplitudeResolution,
}

/// TODO: docs
#[derive(BitfieldSpecifier, Debug, Clone, Copy, PartialEq, Eq)]
#[bits = 2]
pub enum AmplitudeResolution {
  /// default, best for DMA channels A/B
  NineBit32768Hz = 0,
  EightBit65536Hz = 1,
  SevenBit131072Hz = 2,
  /// best for PSG channels 1-4
  SixBit262144Hz = 3,
}
