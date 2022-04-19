#![no_std]
#![no_main]

use gba::prelude::*;

#[derive(Copy, Clone)]
struct Vector2 {
    x: i32,
    y: i32
}

impl Vector2 {
    fn new(x:i32, y:i32) -> Self {
        return Vector2{x: x, y: y};
    }
}

#[panic_handler]
#[allow(unused)]
fn panic(info: &core::panic::PanicInfo) -> ! {
  // This kills the emulation with a message if we're running inside an
  // emulator we support (mGBA or NO$GBA), or just crashes the game if we
  // aren't.
  //fatal!("{}", info);

  loop {
    DISPCNT.read();
  }
}

/// Performs a busy loop until VBlank starts.
///
/// This is very inefficient, and please keep following the lessons until we
/// cover how interrupts work!
pub fn spin_until_vblank() {
  while VCOUNT.read() < 160 {}
}

/// Performs a busy loop until VDraw starts.
///
/// This is very inefficient, and please keep following the lessons until we
/// cover how interrupts work!
pub fn spin_until_vdraw() {
  while VCOUNT.read() >= 160 {}
}

#[no_mangle]
pub fn main() -> ! {
  const SETTING: DisplayControl = DisplayControl::new().with_display_mode(3).with_display_bg2(true);
  DISPCNT.write(SETTING);



  loop {
    spin_until_vblank();
    mode3::bitmap_xy(0, 0).write(Color::from_rgb(0, 255, 0));
    spin_until_vdraw();
  }
}
