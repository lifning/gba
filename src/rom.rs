//! Module for things related to ROM.

use super::*;

pub const WAITCNT: VolAddress<WaitstateControl, Safe, Safe> = unsafe { VolAddress::new(0x400_0204) };

#[bitfield(bits = 16)]
#[repr(u16)]
pub struct WaitstateControl {
  pub sram: WaitstateFirstAccess,
  pub ws0_first_access: WaitstateFirstAccess,
  pub ws0_second_access: bool, // true = 2, false = 1
  pub ws1_first_access: WaitstateFirstAccess,
  pub ws1_second_access: bool, // true = 4, false = 1
  pub ws2_first_access: WaitstateFirstAccess,
  pub ws2_second_access: bool, // true = 8, false = 1
  pub phi_terminal_output: PhiTerminalOutput,
  #[skip] __0: bool,
  pub game_pak_prefetch_buffer: bool,
  pub game_pak_is_cgb: bool,
}

#[derive(BitfieldSpecifier, Debug, Clone, Copy, PartialEq, Eq)]
#[bits = 2]
pub enum WaitstateFirstAccess {
  Cycles4 = 0,
  Cycles3 = 1,
  Cycles2 = 2,
  Cycles8 = 3,
}

#[derive(BitfieldSpecifier, Debug, Clone, Copy, PartialEq, Eq)]
#[bits = 2]
pub enum PhiTerminalOutput {
  Disabled = 0,
  Freq4MHz = 1,
  Freq8MHz = 2,
  Freq16MHz = 3,
}
