//! Allows access to the keypad.

use super::*;

/// The Key Input Register.
///
/// This register follows the "low-active" convention. If you want your code to
/// follow the "high-active" convention (hint: you probably do, it's far easier
/// to work with) then call `read_key_input()` rather than reading this register
/// directly. It will perform the necessary bit flip operation for you.
pub const KEYINPUT: VolAddress<u16, Safe, ()> = unsafe { VolAddress::new(0x400_0130) };

/// A "tribool" value helps us interpret the arrow pad.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum TriBool {
  /// -1
  Minus = -1,
  /// +0
  Neutral = 0,
  /// +1
  Plus = 1,
}

/// Records a particular key press combination.
///
/// Methods here follow the "high-active" convention, where a bit is enabled
/// when it's part of the set.
#[bitfield(bits = 16)]
#[repr(u16)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct KeyInput {
  pub a: bool,
  pub b: bool,
  pub select: bool,
  pub start: bool,
  pub right: bool,
  pub left: bool,
  pub up: bool,
  pub down: bool,
  pub r: bool,
  pub l: bool,
  #[skip] __0: B6,
}

impl KeyInput {
  /// Takes the set difference between these keys and another set of keys.
  pub fn difference(self, other: Self) -> Self {
    KeyInput::from(u16::from(self) ^ u16::from(other))
  }

  pub fn pressed_since(self, previous: Self) -> Self {
    KeyInput::from(u16::from(self.difference(previous)) & u16::from(self))
  }

  pub fn released_since(self, previous: Self) -> Self {
    KeyInput::from(u16::from(self.difference(previous)) & u16::from(previous))
  }

  /// Right/left tribool.
  ///
  /// Right is Plus and Left is Minus
  pub fn x_tribool(self) -> TriBool {
    if self.right() {
      TriBool::Plus
    } else if self.left() {
      TriBool::Minus
    } else {
      TriBool::Neutral
    }
  }

  /// Up/down tribool.
  ///
  /// Down is Plus and Up is Minus
  pub fn y_tribool(self) -> TriBool {
    if self.down() {
      TriBool::Plus
    } else if self.up() {
      TriBool::Minus
    } else {
      TriBool::Neutral
    }
  }
}

/// Gets the current state of the keys
pub fn read_key_input() -> KeyInput {
  // Note(Lokathor): The 10 used bits are "low when pressed" style, but the 6
  // unused bits are always low, so we XOR with this mask to get a result where
  // the only active bits are currently pressed keys.
  KeyInput::from(KEYINPUT.read() ^ 0b0000_0011_1111_1111)
}

/// Use this to configure when a keypad interrupt happens.
///
/// See the `KeyInterruptSetting` type for more.
pub const KEYCNT: VolAddress<KeyInterruptSetting, Safe, Safe> =
  unsafe { VolAddress::new(0x400_0132) };

/// Allows configuration of when a keypad interrupt fires.
///
/// * The most important bit here is the `irq_enabled` bit, which determines
///   if an interrupt happens at all.
/// * The second most important bit is the `irq_logical_and` bit. If this bit
///   is set, _all_ the selected buttons are required to be set for the
///   interrupt to be fired (logical AND). If it's not set then _any_ of the
///   buttons selected can be pressed to fire the interrupt (logical OR).
/// * All other bits select a particular button to be required or not as part
///   of the interrupt firing.
///
/// NOTE: This _only_ configures the operation of when keypad interrupts can
/// fire. You must still set the [`IME`](irq::IME) to have interrupts at all,
/// and you must further set [`IE`](irq::IE) for keypad interrupts to be
/// possible.
#[allow(missing_docs)]
#[bitfield(bits = 16)]
#[repr(u16)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct KeyInterruptSetting {
  pub a: bool,
  pub b: bool,
  pub select: bool,
  pub start: bool,
  pub right: bool,
  pub left: bool,
  pub up: bool,
  pub down: bool,
  pub r: bool,
  pub l: bool,
  #[skip] __0: B4,
  pub irq_enabled: bool,
  pub irq_logical_and: bool,
}
