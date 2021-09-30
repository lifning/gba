//! This module contains wrappers for all GBA BIOS function calls.
//!
//! A GBA BIOS call has significantly more overhead than a normal function call,
//! so think carefully before using them too much.
//!
//! The actual content of each function here is generally a single inline asm
//! instruction to invoke the correct BIOS function (`swi x`, with `x` being
//! whatever value is necessary for that function). Some functions also perform
//! necessary checks to save you from yourself, such as not dividing by zero.

#![cfg_attr(not(target_arch = "arm"), allow(unused_variables))]

#[cfg(target_arch = "arm")]
use core::mem;

use super::*;
use io::irq::IrqFlags;

//TODO: ALL functions in this module should have `if cfg!(test)` blocks. The
//functions that never return must panic, the functions that return nothing
//should just do so, and the math functions should just return the correct math
//I guess.

/// (`swi 0x00`) SoftReset the device.
///
/// This function does not ever return.
///
/// Instead, it clears the top `0x200` bytes of IWRAM (containing stacks, and
/// BIOS IRQ vector/flags), re-initializes the system, supervisor, and irq stack
/// pointers (new values listed below), sets `r0` through `r12`, `LR_svc`,
/// `SPSR_svc`, `LR_irq`, and `SPSR_irq` to zero, and enters system mode. The
/// return address is loaded into `r14` and then the function jumps there with
/// `bx r14`.
///
/// * sp_svc: `0x300_7FE0`
/// * sp_irq: `0x300_7FA0`
/// * sp_sys: `0x300_7F00`
/// * Zero-filled Area: `0x300_7E00` to `0x300_7FFF`
/// * Return Address: Depends on the 8-bit flag value at `0x300_7FFA`. In either
///   case execution proceeds in ARM mode.
///   * zero flag: `0x800_0000` (ROM), which for our builds means that the
///     `crt0` program to execute (just like with a fresh boot), and then
///     control passes into `main` and so on.
///   * non-zero flag: `0x200_0000` (RAM), This is where a multiboot image would
///     go if you were doing a multiboot thing. However, this project doesn't
///     support multiboot at the moment. You'd need an entirely different build
///     pipeline because there's differences in header format and things like
///     that. Perhaps someday, but probably not even then. Submit the PR for it
///     if you like!
///
/// ## Safety
///
/// This functions isn't ever unsafe to the current iteration of the program.
/// However, because not all memory is fully cleared you theoretically could
/// threaten the _next_ iteration of the program that runs. I'm _fairly_
/// convinced that you can't actually use this to force purely safe code to
/// perform UB, but such a scenario might exist.
#[inline(always)]
#[cfg_attr(all(target_arch = "arm", target_env = ""), instruction_set(arm::a32))]
pub unsafe fn soft_reset() -> ! {
  #[cfg(not(target_arch = "arm"))]
  {
    unimplemented!("This function is not supported on this target.")
  }
  #[cfg(target_arch = "arm")]
  {
    asm!("swi 0x000000", options(noreturn))
  }
}

/// (`swi 0x01`) RegisterRamReset.
///
/// Clears the portions of memory given by the `flags` value, sets the Display
/// Control Register to `0x80` (forced blank and nothing else), then returns.
///
/// * Flag bits:
///   0) Clears the 256k of EWRAM (don't use if this is where your function call
///      will return to!)
///   1) Clears the 32k of IWRAM _excluding_ the last `0x200` bytes (see also:
///      the `soft_reset` function)
///   2) Clears all Palette data
///   3) Clears all VRAM
///   4) Clears all OAM (reminder: a zeroed object isn't disabled!)
///   5) Reset SIO registers (resets them to general purpose mode)
///   6) Reset Sound registers
///   7) Reset all IO registers _other than_ SIO and Sound
///
/// **Bug:** The LSB of `SIODATA32` is always zeroed, even if bit 5 was not
/// enabled. This is sadly a bug in the design of the GBA itself.
///
/// ## Safety
///
/// It is generally a safe operation to suddenly clear any part of the GBA's
/// memory, except in the case that you were executing out of EWRAM and clear
/// that. If you do then you return to nothing and have a bad time.
#[inline(always)]
#[cfg_attr(all(target_arch = "arm", target_env = ""), instruction_set(arm::a32))]
pub unsafe fn register_ram_reset(flags: RegisterRAMResetFlags) {
  #[cfg(not(target_arch = "arm"))]
  {
    unimplemented!("This function is not supported on this target.")
  }
  #[cfg(target_arch = "arm")]
  {
    asm!("swi 0x010000", in("r0") flags.0);
  }
}
newtype! {
  /// Flags for use with `register_ram_reset`.
  RegisterRAMResetFlags, u8
}
#[allow(missing_docs)]
impl RegisterRAMResetFlags {
  phantom_fields! {
    self.0: u8,
    ewram: 0,
    iwram: 1,
    palram: 2,
    vram: 3,
    oam: 4,
    sio: 5,
    sound: 6,
    other_io: 7,
  }
}

/// (`swi 0x02`) Halts the CPU until an interrupt occurs.
///
/// Components _other than_ the CPU continue to function. Halt mode ends when
/// any enabled interrupt triggers.
#[inline(always)]
#[cfg_attr(all(target_arch = "arm", target_env = ""), instruction_set(arm::a32))]
pub fn halt() {
  #[cfg(not(target_arch = "arm"))]
  {
    unimplemented!("This function is not supported on this target.")
  }
  #[cfg(target_arch = "arm")]
  {
    unsafe {
      asm!("swi 0x020000");
    }
  }
}

/// (`swi 0x03`) Stops the CPU as well as most other components.
///
/// Stop mode must be stopped by an interrupt, but can _only_ be stopped by a
/// Keypad, Game Pak, or General-Purpose-SIO interrupt.
///
/// Before going into stop mode you should manually disable video and sound (or
/// they will continue to consume power), and you should also disable any other
/// optional externals such as rumble and infra-red.
#[inline(always)]
#[cfg_attr(all(target_arch = "arm", target_env = ""), instruction_set(arm::a32))]
pub fn stop() {
  #[cfg(not(target_arch = "arm"))]
  {
    unimplemented!("This function is not supported on this target.")
  }
  #[cfg(target_arch = "arm")]
  {
    unsafe {
      asm!("swi 0x030000");
    }
  }
}

/// (`swi 0x04`) "IntrWait", similar to halt but with more options.
///
/// * The first argument controls if you want to ignore all current flags and
///   wait until a new flag is set.
/// * The second argument is what flags you're waiting on (same format as the
///   [`IE`](io::irq::IE)/[`IF`](io::irq::IF) registers).
///
/// If you're trying to handle more than one interrupt at once this has less
/// overhead than calling `halt` over and over.
///
/// When using this routing your interrupt handler MUST update the BIOS
/// Interrupt Flags at [`BIOS_IF`](io::irq::BIOS_IF) in addition to
/// the usual interrupt acknowledgement.
#[inline(always)]
#[cfg_attr(all(target_arch = "arm", target_env = ""), instruction_set(arm::a32))]
pub fn interrupt_wait(ignore_current_flags: bool, target_flags: IrqFlags) {
  #[cfg(not(target_arch = "arm"))]
  {
    unimplemented!("This function is not supported on this target.")
  }
  #[cfg(target_arch = "arm")]
  {
    unsafe {
      asm!(
          "swi 0x040000",
          in("r0") ignore_current_flags as u8,
          in("r1") target_flags.0,
      );
    }
  }
}

/// (`swi 0x05`) "VBlankIntrWait", VBlank Interrupt Wait.
///
/// This is as per `interrupt_wait(true, IrqFlags::new().with_vblank(true))`
/// (aka "wait for a new vblank"). You must follow the same guidelines that
/// [`interrupt_wait`](interrupt_wait) outlines.
#[inline(always)]
#[cfg_attr(all(target_arch = "arm", target_env = ""), instruction_set(arm::a32))]
pub fn vblank_interrupt_wait() {
  #[cfg(not(target_arch = "arm"))]
  {
    unimplemented!("This function is not supported on this target.")
  }
  #[cfg(target_arch = "arm")]
  {
    unsafe {
      asm!(
          "swi 0x050000",
          out("r0") _,
          out("r1") _,
      );
    }
  }
}

/// (`swi 0x06`) Software Division and Remainder.
///
/// ## Panics
///
/// If the denominator is 0.
#[inline(always)]
#[cfg_attr(all(target_arch = "arm", target_env = ""), instruction_set(arm::a32))]
pub fn div_rem(numerator: i32, denominator: i32) -> (i32, i32) {
  assert!(denominator != 0);
  #[cfg(not(target_arch = "arm"))]
  {
    (numerator / denominator, numerator % denominator)
  }
  #[cfg(target_arch = "arm")]
  {
    let div_out: i32;
    let rem_out: i32;
    unsafe {
      asm!(
          "swi 0x060000",
          inout("r0") numerator => div_out,
          inout("r1") denominator => rem_out,
          out("r3") _,
          options(nostack, nomem),
      );
    }
    (div_out, rem_out)
  }
}

/// As `div_rem`, keeping only the `div` output.
#[inline(always)]
pub fn div(numerator: i32, denominator: i32) -> i32 {
  div_rem(numerator, denominator).0
}

/// As `div_rem`, keeping only the `rem` output.
#[inline(always)]
pub fn rem(numerator: i32, denominator: i32) -> i32 {
  div_rem(numerator, denominator).1
}

// (`swi 0x07`): We deliberately don't implement this one. It's the same as DIV
// but with reversed arguments, so it just runs 3 cycles slower as it does the
// swap.

/// (`swi 0x08`) Integer square root.
///
/// If you want more fractional precision, you can shift your input to the left
/// by `2n` bits to get `n` more bits of fractional precision in your output.
#[inline(always)]
#[cfg_attr(all(target_arch = "arm", target_env = ""), instruction_set(arm::a32))]
pub fn sqrt(val: u32) -> u16 {
  #[cfg(not(target_arch = "arm"))]
  {
    0 // TODO: simulate this properly when not on GBA
  }
  #[cfg(target_arch = "arm")]
  {
    let out: u32;
    unsafe {
      asm!(
          "swi 0x080000",
          inout("r0") val => out,
          out("r1") _,
          out("r3") _,
          options(pure, nomem),
      );
    }
    out as u16
  }
}

/// (`swi 0x09`) Gives the arctangent of `theta`.
///
/// The input format is 1 bit for sign, 1 bit for integral part, 14 bits for
/// fractional part.
///
/// Accuracy suffers if `theta` is less than `-pi/4` or greater than `pi/4`.
#[inline(always)]
#[cfg_attr(all(target_arch = "arm", target_env = ""), instruction_set(arm::a32))]
pub fn atan(theta: i16) -> i16 {
  #[cfg(not(target_arch = "arm"))]
  {
    0 // TODO: simulate this properly when not on GBA
  }
  #[cfg(target_arch = "arm")]
  {
    let out: i16;
    unsafe {
      asm!(
          "swi 0x090000",
          inout("r0") theta => out,
          out("r1") _,
          out("r3") _,
          options(pure, nomem),
      );
    }
    out
  }
}

/// (`swi 0x0A`) Gives the atan2 of `y` over `x`.
///
/// The output `theta` value maps into the range `[0, 2pi)`, or `0 .. 2pi` if
/// you prefer Rust's range notation.
///
/// `y` and `x` use the same format as with `atan`: 1 bit for sign, 1 bit for
/// integral, 14 bits for fractional.
#[inline(always)]
#[cfg_attr(all(target_arch = "arm", target_env = ""), instruction_set(arm::a32))]
pub fn atan2(y: i16, x: i16) -> u16 {
  #[cfg(not(target_arch = "arm"))]
  {
    0 // TODO: simulate this properly when not on GBA
  }
  #[cfg(target_arch = "arm")]
  {
    let out: u16;
    unsafe {
      asm!(
          "swi 0x0A0000",
          inout("r0") x => out,
          in("r1") y,
          out("r3") _,
          options(pure, nomem),
      );
    }
    out
  }
}

/// (`swi 0x0B`) "CpuSet", `u16` memory copy.
///
/// * `count` is the number of `u16` values to copy (20 bits or less)
/// * `fixed_source` argument, if true, turns this copying routine into a
///   filling routine.
///
/// ## Safety
///
/// * Both pointers must be aligned
#[inline(always)]
#[cfg_attr(all(target_arch = "arm", target_env = ""), instruction_set(arm::a32))]
pub unsafe fn cpu_set16(src: *const u16, dest: *mut u16, count: u32, fixed_source: bool) {
  #[cfg(not(target_arch = "arm"))]
  {
    unimplemented!("This function is not supported on this target.")
  }
  #[cfg(target_arch = "arm")]
  {
    let control = count + ((fixed_source as u32) << 24);
    asm!(
        "swi 0x0B0000",
        in("r0") src,
        in("r1") dest,
        in("r2") control,
        lateout("r3") _,
    );
  }
}

/// (`swi 0x0B`) "CpuSet", `u32`  memory copy/fill.
///
/// * `count` is the number of `u32` values to copy (20 bits or less)
/// * `fixed_source` argument, if true, turns this copying routine into a
///   filling routine.
///
/// ## Safety
///
/// * Both pointers must be aligned
#[inline(always)]
#[cfg_attr(all(target_arch = "arm", target_env = ""), instruction_set(arm::a32))]
pub unsafe fn cpu_set32(src: *const u32, dest: *mut u32, count: u32, fixed_source: bool) {
  #[cfg(not(target_arch = "arm"))]
  {
    unimplemented!("This function is not supported on this target.")
  }
  #[cfg(target_arch = "arm")]
  {
    let control = count + ((fixed_source as u32) << 24) + (1 << 26);
    asm!(
        "swi 0x0B0000",
        in("r0") src,
        in("r1") dest,
        in("r2") control,
        lateout("r3") _,
    );
  }
}

/// (`swi 0x0C`) "CpuFastSet", copies memory in 32 byte chunks.
///
/// * The `count` value is the number of `u32` values to transfer (20 bits or
///   less), and it's rounded up to the nearest multiple of 8 words.
/// * The `fixed_source` argument, if true, turns this copying routine into a
///   filling routine.
///
/// ## Safety
///
/// * Both pointers must be aligned
#[inline(always)]
#[cfg_attr(all(target_arch = "arm", target_env = ""), instruction_set(arm::a32))]
pub unsafe fn cpu_fast_set(src: *const u32, dest: *mut u32, count: u32, fixed_source: bool) {
  #[cfg(not(target_arch = "arm"))]
  {
    unimplemented!("This function is not supported on this target.")
  }
  #[cfg(target_arch = "arm")]
  {
    let control = count + ((fixed_source as u32) << 24);
    asm!(
        "swi 0x0C0000",
        in("r0") src,
        in("r1") dest,
        in("r2") control,
        lateout("r3") _,
    );
  }
}

/// (`swi 0x0C`) "GetBiosChecksum" (Undocumented)
///
/// Though we usually don't cover undocumented functionality, this one can make
/// it into the crate.
///
/// The function computes the checksum of the BIOS data. You should get either
/// `0xBAAE_187F` (GBA / GBA SP) or `0xBAAE_1880` (DS in GBA mode). If you get
/// some other value I guess you're probably running on an emulator that just
/// broke the fourth wall.
#[cfg_attr(all(target_arch = "arm", target_env = ""), instruction_set(arm::a32))]
pub fn get_bios_checksum() -> u32 {
  #[cfg(not(target_arch = "arm"))]
  {
    unimplemented!("This function is not supported on this target.")
  }
  #[cfg(target_arch = "arm")]
  {
    let out: u32;
    unsafe {
      asm!(
          "swi 0x0D0000",
          out("r0") out,
          options(pure, readonly),
      );
    }
    out
  }
}

// TODO: consider using "fixed" crate? (or optionally implementing setters for it behind a feature)
#[repr(C, packed(2))]
pub struct BgAffineSetParams {
  pub data_center_x: i32, /// .8f
  pub data_center_y: i32, /// .8f
  pub display_center_x: i16,
  pub display_center_y: i16,
  pub scale_x: i16, /// .8f
  pub scale_y: i16, /// .8f
  pub angle: u16,
}

#[cfg_attr(all(target_arch = "arm", target_env = ""), instruction_set(arm::a32))]
pub fn bg_affine_set(src: *const BgAffineSetParams, dest: usize, num_calc: u32) {
  #[cfg(not(target_arch = "arm"))]
  {
    unimplemented!()
  }
  #[cfg(target_arch = "arm")]
  {
    unsafe {
      asm!(
          "swi 0x0E0000",
          in("r0") src,
          in("r1") dest,
          in("r2") num_calc,
          lateout("r3") _,
      );
    }
  }
}

#[repr(C, packed(2))]
pub struct ObjAffineSetParams {
  pub scale_x: i16, /// .8f
  pub scale_y: i16, /// .8f
  pub angle: u16,
}

newtype_enum! {
  ObjAffineSetOffset = u32,
  Continuous = 2,
  OAM = 8,
}

#[cfg_attr(all(target_arch = "arm", target_env = ""), instruction_set(arm::a32))]
pub fn obj_affine_set(src: *const ObjAffineSetParams, dest: usize, num_calc: u32, offset: ObjAffineSetOffset) {
  #[cfg(not(target_arch = "arm"))]
  {
    unimplemented!()
  }
  #[cfg(target_arch = "arm")]
  {
    unsafe {
      asm!(
          "swi 0x0F0000",
          in("r0") src,
          in("r1") dest,
          in("r2") num_calc,
          in("r3") offset as u32,
      );
    }
  }
}

#[repr(u8)]
pub enum BitUnpackSourceBitWidth {
  One = 1,
  Two = 2,
  Four = 4,
  Eight = 8,
}

#[repr(u8)]
pub enum BitUnpackDestinationBitWidth {
  One = 1,
  Two = 2,
  Four = 4,
  Eight = 8,
  Sixteen = 16,
  ThirtyTwo = 32,
}

newtype!(BitUnPackDataParams, pub u32);

#[allow(missing_docs)]
impl BitUnPackDataParams {
  phantom_fields! {
    self.0: u32,
    data_offset: 0-30,
    zero_data: 31,
  }
}

#[repr(C, packed)]
pub struct BitUnpackParams {
  pub source_data_length: u16,
  pub source_bit_width: BitUnpackSourceBitWidth,
  pub destination_bit_width: BitUnpackDestinationBitWidth,
  pub data_offset_and_zero_flag: BitUnPackDataParams,
}

#[cfg_attr(all(target_arch = "arm", target_env = ""), instruction_set(arm::a32))]
pub fn bit_unpack(src: *const u8, dest: *mut u32, params: *const BitUnpackParams) {
  #[cfg(not(target_arch = "arm"))]
  {
    unimplemented!()
  }
  #[cfg(target_arch = "arm")]
  {
    unsafe {
      asm!(
          "swi 0x100000",
          in("r0") src,
          in("r1") dest,
          in("r2") params,
          lateout("r3") _,
      );
    }
  }
}

#[cfg_attr(all(target_arch = "arm", target_env = ""), instruction_set(arm::a32))]
pub fn lz77_uncomp_8bit(src: *const u32, dest: *mut u8) {
  #[cfg(not(target_arch = "arm"))]
  {
    unimplemented!()
  }
  #[cfg(target_arch = "arm")]
  {
    unsafe {
      asm!(
          "swi 0x110000",
          in("r0") src,
          in("r1") dest,
          lateout("r3") _,
      );
    }
  }
}

#[cfg_attr(all(target_arch = "arm", target_env = ""), instruction_set(arm::a32))]
pub fn lz77_uncomp_16bit(src: *const u32, dest: *mut u16) {
  #[cfg(not(target_arch = "arm"))]
  {
    unimplemented!()
  }
  #[cfg(target_arch = "arm")]
  {
    unsafe {
      asm!(
          "swi 0x120000",
          in("r0") src,
          in("r1") dest,
          lateout("r3") _,
      );
    }
  }
}

#[cfg_attr(all(target_arch = "arm", target_env = ""), instruction_set(arm::a32))]
pub fn huff_uncomp(src: *const u32, dest: *mut u32) {
  #[cfg(not(target_arch = "arm"))]
  {
    unimplemented!()
  }
  #[cfg(target_arch = "arm")]
  {
    unsafe {
      asm!(
          "swi 0x130000",
          in("r0") src,
          in("r1") dest,
          lateout("r3") _,
      );
    }
  }
}

#[cfg_attr(all(target_arch = "arm", target_env = ""), instruction_set(arm::a32))]
pub fn rl_uncomp_8bit(src: *const u32, dest: *mut u8) {
  #[cfg(not(target_arch = "arm"))]
  {
    unimplemented!()
  }
  #[cfg(target_arch = "arm")]
  {
    unsafe {
      asm!(
          "swi 0x140000",
          in("r0") src,
          in("r1") dest,
          lateout("r3") _,
      );
    }
  }
}

#[cfg_attr(all(target_arch = "arm", target_env = ""), instruction_set(arm::a32))]
pub fn rl_uncomp_16bit(src: *const u32, dest: *mut u16) {
  #[cfg(not(target_arch = "arm"))]
  {
    unimplemented!()
  }
  #[cfg(target_arch = "arm")]
  {
    unsafe {
      asm!(
          "swi 0x150000",
          in("r0") src,
          in("r1") dest,
          lateout("r3") _,
      );
    }
  }
}

#[cfg_attr(all(target_arch = "arm", target_env = ""), instruction_set(arm::a32))]
pub fn diff_8bit_unfilter_write_8bit(src: *const u8, dest: *mut u8) {
  #[cfg(not(target_arch = "arm"))]
  {
    unimplemented!()
  }
  #[cfg(target_arch = "arm")]
  {
    unsafe {
      asm!(
          "swi 0x160000",
          in("r0") src,
          in("r1") dest,
          lateout("r3") _,
      );
    }
  }
}

#[cfg_attr(all(target_arch = "arm", target_env = ""), instruction_set(arm::a32))]
pub fn diff_8bit_unfilter_write_16bit(src: *const u8, dest: *mut u16) {
  #[cfg(not(target_arch = "arm"))]
  {
    unimplemented!()
  }
  #[cfg(target_arch = "arm")]
  {
    unsafe {
      asm!(
          "swi 0x170000",
          in("r0") src,
          in("r1") dest,
          lateout("r3") _,
      );
    }
  }
}

#[cfg_attr(all(target_arch = "arm", target_env = ""), instruction_set(arm::a32))]
pub fn diff_16bit_unfilter(src: *const u16, dest: *mut u16) {
  #[cfg(not(target_arch = "arm"))]
  {
    unimplemented!()
  }
  #[cfg(target_arch = "arm")]
  {
    unsafe {
      asm!(
          "swi 0x180000",
          in("r0") src,
          in("r1") dest,
          lateout("r3") _,
      );
    }
  }
}

/// (`swi 0x19`) "SoundBias", adjusts the volume level to a new level.
///
/// This increases or decreases the current level of the `SOUNDBIAS` register
/// (with short delays) until at the new target level. The upper bits of the
/// register are unaffected.
///
/// The final sound level setting will be `level` * `0x200`.
#[cfg_attr(all(target_arch = "arm", target_env = ""), instruction_set(arm::a32))]
pub fn sound_bias(level: u32) {
  #[cfg(not(target_arch = "arm"))]
  {
    unimplemented!("This function is not supported on this target.")
  }
  #[cfg(target_arch = "arm")]
  {
    unsafe {
      asm!("swi 0x190000", in("r0") level);
    }
  }
}

#[repr(packed, C)]
pub struct SoundArea<const MAX_CH: usize, const PCM_BF_X2: usize> {
    /// Flag the system checks to see whether the work area has been initialized
    /// and whether it is currently being accessed.
    pub ident: u32,
    _dma_count: u8,
    /// Variable for applying reverb effects to direct sound
    pub reverb: u8,
    // strange names from gbatek for these private fields
    _d1: u16,
    _func: *mut fn() -> (),
    _intp: i32,
    _no_use: *mut core::ffi::c_void,
    /// The structure array for controlling the direct sound channels
    /// (currently 8 channels are available).
    /// The term "channel" here does not refer to hardware channels,
    /// but rather to virtual constructs inside the sound driver.
    pub vchn: [SoundChannel; MAX_CH],
    pub pcmbuf: [i8; PCM_BF_X2],
}

#[repr(packed, C)]
pub struct SoundChannel {
    /// The flag indicating the status of this channel.
    /// When 0 sound is stopped.
    /// To start sound, set other parameters and then write 80h to here.
    /// To stop sound, logical OR 40h for a release-attached off (key-off),
    /// or write zero for a pause.
    /// The use of other bits is prohibited.
    // TODO: repr(u8) enum? are other values set by bios? hide behind a setter fn?
    pub status_flag: u8,
    _r1: u8,
    /// Sound volume output to right side
    pub right_volume: u8,
    /// Sound volume output to left side
    pub left_volume: u8,
    /// The attack value of the envelope.
    /// When the sound starts, the volume begins at zero and increases every 1/60 second.
    /// When it reaches 255, the process moves on to the next decay value.
    pub attack: u8,
    /// The decay value of the envelope.
    /// It is multiplied by "this value/256" every 1/60 sec. and when sustain value is reached,
    /// the process moves to the sustain condition.
    pub decay: u8,
    /// The sustain value of the envelope. The sound is sustained by this amount.
    /// (Actually, multiplied by right_volume/256, left_volume/256 and output left and right.)
    pub sustain: u8,
    /// The release value of the envelope.
    /// Key-off (logical OR 40h in sf) to enter this state.
    /// The value is multiplied by "this value/256" every 1/60 sec.
    /// and when it reaches zero, this channel is completely stopped.
    pub release: u8,
    _r2: [u8; 4],
    /// The frequency of the produced sound.
    /// Write the value obtained with the MidiKey2Freq function here.
    pub frequency: u32,
    // workaround for size issues
    pub wave_data: *const WaveDataProxy,
    _r3: [u32; 6],
    _r4: [u8; 4],
}

#[repr(packed, C)]
// TODO: better constraint, it needs to be a plain array (not a pointer!)
pub struct WaveData<const SIZE: usize> {
    _type: u16,
    /// At the present time, non-looped (1 shot) waveform is 0000h and forward loop is 4000h.
    pub stat: u16,
    /// This value is used to calculate the frequency. It is obtained using the following formula:
    /// `sampling rate x 2^((180-original MIDI key)/12)`
    pub freq: u32,
    /// Loop pointer (start of loop)
    pub loop_position: u32,
    /// Number of samples (end position)
    pub size: u32,
    /// The actual waveform data. Takes (number of samples+1) bytes of 8bit signed linear
    /// uncompressed data. The last byte is zero for a non-looped waveform, and the same value as
    /// the loop pointer data for a looped waveform.
    pub data: [i8; SIZE],
}

pub struct WaveDataProxy;

impl<const SIZE: usize> From<&WaveData<SIZE>> for *const WaveDataProxy {
    fn from(wave: &WaveData<SIZE>) -> Self {
        let wave_proxy: *const WaveData<SIZE> = wave;
        wave_proxy as *const WaveDataProxy
    }
}

// TODO: SoundDriverInit

/// (`swi 0x1B`) "SoundDriverMode", sets the sound driver operation mode.
///
/// The `mode` input uses the following flags and bits:
///
/// * Bits 0-6: Reverb value
/// * Bit 7: Reverb Enable
/// * Bits 8-11: Simultaneously-produced channel count (default=8)
/// * Bits 12-15: Master Volume (1-15, default=15)
/// * Bits 16-19: Playback Frequency Index (see below, default=4)
/// * Bits 20-23: "Final number of D/A converter bits (8-11 = 9-6bits, def. 9=8bits)" TODO: what the hek?
/// * Bits 24 and up: Not used
///
/// The frequency index selects a frequency from the following array:
/// * 0: 5734
/// * 1: 7884
/// * 2: 10512
/// * 3: 13379
/// * 4: 15768
/// * 5: 18157
/// * 6: 21024
/// * 7: 26758
/// * 8: 31536
/// * 9: 36314
/// * 10: 40137
/// * 11: 42048
#[cfg_attr(all(target_arch = "arm", target_env = ""), instruction_set(arm::a32))]
pub fn sound_driver_mode(mode: u32) {
  #[cfg(not(target_arch = "arm"))]
  {
    unimplemented!("This function is not supported on this target.")
  }
  #[cfg(target_arch = "arm")]
  {
    unsafe {
      asm!("swi 0x1B0000", in("r0") mode);
    }
  }
}
//TODO(lokathor): newtype this mode business.

/// (`swi 0x1C`) "SoundDriverMain", main of the sound driver
///
/// You should call `SoundDriverVSync` immediately after the vblank interrupt
/// fires.
///
/// "After that, this routine is called after BG and OBJ processing is
/// executed." --what?
#[inline(always)]
#[cfg_attr(all(target_arch = "arm", target_env = ""), instruction_set(arm::a32))]
pub fn sound_driver_main() {
  #[cfg(not(target_arch = "arm"))]
  {
    unimplemented!("This function is not supported on this target.")
  }
  #[cfg(target_arch = "arm")]
  {
    unsafe {
      asm!("swi 0x1C0000");
    }
  }
}

/// (`swi 0x1D`) "SoundDriverVSync", resets the sound DMA.
///
/// The timing is critical, so you should call this _immediately_ after the
/// vblank interrupt (every 1/60th of a second).
#[inline(always)]
#[cfg_attr(all(target_arch = "arm", target_env = ""), instruction_set(arm::a32))]
pub fn sound_driver_vsync() {
  #[cfg(not(target_arch = "arm"))]
  {
    unimplemented!("This function is not supported on this target.")
  }
  #[cfg(target_arch = "arm")]
  {
    unsafe {
      asm!("swi 0x1D0000");
    }
  }
}

/// (`swi 0x1E`) "SoundChannelClear", clears the direct sound channels and stops
/// the sound.
///
/// "This function may not operate properly when the library which expands the
/// sound driver feature is combined afterwards. In this case, do not use it."
/// --what?
#[inline(always)]
#[cfg_attr(all(target_arch = "arm", target_env = ""), instruction_set(arm::a32))]
pub fn sound_channel_clear() {
  #[cfg(not(target_arch = "arm"))]
  {
    unimplemented!("This function is not supported on this target.")
  }
  #[cfg(target_arch = "arm")]
  {
    unsafe {
      asm!("swi 0x1E0000");
    }
  }
}

//MidiKey2Freq
//MultiBoot

/// (`swi 0x28`) "SoundDriverVSyncOff", disables sound
///
/// If you can't use vblank interrupts to ensure that `sound_driver_vsync` is
/// called every 1/60th of a second for any reason you must use this function to
/// stop sound DMA. Otherwise the DMA will overrun its buffer and cause random
/// noise.
#[inline(always)]
#[cfg_attr(all(target_arch = "arm", target_env = ""), instruction_set(arm::a32))]
pub fn sound_driver_vsync_off() {
  #[cfg(not(target_arch = "arm"))]
  {
    unimplemented!("This function is not supported on this target.")
  }
  #[cfg(target_arch = "arm")]
  {
    unsafe {
      asm!("swi 0x280000");
    }
  }
}

/// (`swi 0x29`) "SoundDriverVSyncOn", enables sound that was stopped by
/// `sound_driver_vsync_off`.
///
/// Restarts sound DMA system. After restarting the sound you must have a vblank
/// interrupt followed by a `sound_driver_vsync` within 2/60th of a second.
#[inline(always)]
#[cfg_attr(all(target_arch = "arm", target_env = ""), instruction_set(arm::a32))]
pub fn sound_driver_vsync_on() {
  #[cfg(not(target_arch = "arm"))]
  {
    unimplemented!("This function is not supported on this target.")
  }
  #[cfg(target_arch = "arm")]
  {
    unsafe {
      asm!("swi 0x290000");
    }
  }
}
