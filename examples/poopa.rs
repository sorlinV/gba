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


  fn write_text(text:&str, start_pos:Vector2, color:Color) {
      let support_letter:&str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
      let pixel_pos:[[i32; 25]; 36] = [
      [
      1, 1, 1, 0, 0,
      1, 0, 1, 0, 0,
      1, 1, 1, 0, 0,
      1, 0, 1, 0, 0,
      1, 0, 1, 0, 0,
      ],
      [
      1, 1, 0, 0, 0,
      1, 0, 1, 0, 0,
      1, 1, 0, 0, 0,
      1, 0, 1, 0, 0,
      1, 1, 0, 0, 0,
      ],
      [
      1, 1, 1, 0, 0,
      1, 0, 0, 0, 0,
      1, 0, 0, 0, 0,
      1, 0, 0, 0, 0,
      1, 1, 1, 0, 0,
      ],
      [
      1, 1, 0, 0, 0,
      1, 0, 1, 0, 0,
      1, 0, 1, 0, 0,
      1, 0, 1, 0, 0,
      1, 1, 0, 0, 0,
      ],
      [
      1, 1, 1, 0, 0,
      1, 0, 0, 0, 0,
      1, 1, 0, 0, 0,
      1, 0, 0, 0, 0,
      1, 1, 1, 0, 0,
      ],
      [
      1, 1, 1, 0, 0,
      1, 0, 0, 0, 0,
      1, 1, 0, 0, 0,
      1, 0, 0, 0, 0,
      1, 0, 0, 0, 0,
      ],
      [
      1, 1, 1, 0, 0,
      1, 0, 0, 0, 0,
      1, 0, 1, 1, 0,
      1, 0, 0, 1, 0,
      1, 1, 1, 1, 0,
      ],
      [
      1, 0, 1, 0, 0,
      1, 0, 1, 0, 0,
      1, 1, 1, 0, 0,
      1, 0, 1, 0, 0,
      1, 0, 1, 0, 0,
      ],
      [
      1, 1, 1, 0, 0,
      0, 1, 0, 0, 0,
      0, 1, 0, 0, 0,
      0, 1, 0, 0, 0,
      1, 1, 1, 0, 0,
      ],
      [
      1, 1, 1, 1, 0,
      0, 0, 1, 0, 0,
      0, 0, 1, 0, 0,
      1, 0, 1, 0, 0,
      1, 1, 1, 0, 0,
      ],
      [
      1, 0, 1, 0, 0,
      1, 0, 1, 0, 0,
      1, 1, 0, 0, 0,
      1, 0, 1, 0, 0,
      1, 0, 1, 0, 0,
      ],
      [
      1, 0, 0, 0, 0,
      1, 0, 0, 0, 0,
      1, 0, 0, 0, 0,
      1, 0, 0, 0, 0,
      1, 1, 1, 0, 0,
      ],
      [
      1, 0, 0, 0, 1,
      1, 1, 0, 1, 1,
      1, 0, 1, 0, 1,
      1, 0, 0, 0, 1,
      1, 0, 0, 0, 1,
      ],
      [
      1, 0, 0, 0, 1,
      1, 1, 0, 0, 1,
      1, 0, 1, 0, 1,
      1, 0, 0, 1, 1,
      1, 0, 0, 0, 1,
      ],
      [
      1, 1, 1, 1, 0,
      1, 0, 0, 1, 0,
      1, 0, 0, 1, 0,
      1, 0, 0, 1, 0,
      1, 1, 1, 1, 0,
      ],
      [
      1, 1, 1, 0, 0,
      1, 0, 1, 0, 0,
      1, 1, 1, 0, 0,
      1, 0, 0, 0, 0,
      1, 0, 0, 0, 0,
      ],
      [
      0, 1, 1, 0, 0,
      1, 0, 0, 1, 0,
      1, 0, 0, 1, 0,
      1, 0, 0, 1, 0,
      0, 1, 1, 1, 1,
      ],
      [
      1, 1, 1, 0, 0,
      1, 0, 1, 0, 0,
      1, 1, 0, 0, 0,
      1, 0, 1, 0, 0,
      1, 0, 1, 0, 0,
      ],
      [
      1, 1, 1, 0, 0,
      1, 0, 0, 0, 0,
      1, 1, 1, 0, 0,
      0, 0, 1, 0, 0,
      1, 1, 1, 0, 0,
      ],
      [
      1, 1, 1, 0, 0,
      0, 1, 0, 0, 0,
      0, 1, 0, 0, 0,
      0, 1, 0, 0, 0,
      0, 1, 0, 0, 0,
      ],
      [
      1, 0, 1, 0, 0,
      1, 0, 1, 0, 0,
      1, 0, 1, 0, 0,
      1, 0, 1, 0, 0,
      1, 1, 1, 0, 0,
      ],
      [
      1, 0, 0, 0, 1,
      1, 0, 0, 0, 1,
      1, 0, 0, 0, 1,
      0, 1, 0, 1, 0,
      0, 0, 1, 0, 0,
      ],
      [
      1, 0, 0, 0, 1,
      1, 0, 0, 0, 1,
      1, 0, 1, 0, 1,
      1, 1, 0, 1, 1,
      1, 0, 0, 0, 1,
      ],
      [
      1, 0, 0, 0, 1,
      0, 1, 0, 1, 0,
      0, 0, 1, 0, 0,
      0, 1, 0, 1, 0,
      1, 0, 0, 0, 1,
      ],
      [
      1, 0, 1, 0, 0,
      1, 0, 1, 0, 0,
      1, 0, 1, 0, 0,
      0, 1, 0, 0, 0,
      0, 1, 0, 0, 0,
      ],
      [
      1, 1, 1, 1, 1,
      0, 0, 0, 1, 0,
      0, 0, 1, 0, 0,
      0, 1, 0, 0, 0,
      1, 1, 1, 1, 1,
      ],
      [
      0, 1, 0, 0, 0,
      1, 0, 1, 0, 0,
      1, 0, 1, 0, 0,
      1, 0, 1, 0, 0,
      0, 1, 0, 0, 0,
      ],
      [
      0, 1, 0, 0, 0,
      1, 1, 0, 0, 0,
      0, 1, 0, 0, 0,
      0, 1, 0, 0, 0,
      1, 1, 1, 0, 0,
      ],
      [
      1, 1, 1, 0, 0,
      0, 0, 1, 0, 0,
      1, 1, 1, 0, 0,
      1, 0, 0, 0, 0,
      1, 1, 1, 0, 0,
      ],
      [
      1, 1, 1, 0, 0,
      0, 0, 1, 0, 0,
      1, 1, 1, 0, 0,
      0, 0, 1, 0, 0,
      1, 1, 1, 0, 0,
      ],
      [
      1, 0, 1, 0, 0,
      1, 0, 1, 0, 0,
      1, 1, 1, 0, 0,
      0, 0, 1, 0, 0,
      0, 0, 1, 0, 0,
      ],
      [
      1, 1, 1, 0, 0,
      1, 0, 0, 0, 0,
      1, 1, 1, 0, 0,
      0, 0, 1, 0, 0,
      1, 1, 1, 0, 0,
      ],
      [
      1, 1, 1, 0, 0,
      1, 0, 0, 0, 0,
      1, 1, 1, 0, 0,
      1, 0, 1, 0, 0,
      1, 1, 1, 0, 0,
      ],
      [
      1, 1, 1, 0, 0,
      0, 0, 1, 0, 0,
      0, 1, 1, 0, 0,
      0, 0, 1, 0, 0,
      0, 0, 1, 0, 0,
      ],
      [
      1, 1, 1, 0, 0,
      1, 0, 1, 0, 0,
      1, 1, 1, 0, 0,
      1, 0, 1, 0, 0,
      1, 1, 1, 0, 0,
      ],
      [
      1, 1, 1, 0, 0,
      1, 0, 1, 0, 0,
      1, 1, 1, 0, 0,
      0, 0, 1, 0, 0,
      1, 1, 1, 0, 0,
      ],
      ];
      let unasigned_char = [
      1, 1, 1, 1, 1,
      1, 1, 1, 1, 1,
      1, 1, 1, 1, 1,
      1, 1, 1, 1, 1,
      1, 1, 1, 1, 1,
      ];

      let mut i = 0;
      let mut j;
      let mut char_exist;
      while i < text.len() {
          j = 0;
          char_exist = false;
          while j < support_letter.len() {
              println!("{:?}", support_letter.chars().take(0).last().unwrap());
              if support_letter.chars().take(0).last().unwrap() == 'A' {
                  mode3::bitmap_xy(0, 0).write(Color::from_rgb(0, 255, 0));
              }
              if text.chars().take(i).last().unwrap() == support_letter.chars().take(j).last().unwrap() {
                  mode3::bitmap_xy(0, 0).write(Color::from_rgb(0, 255, 0));
                  print_char(Vector2::new(start_pos.x + ((i * 6) as i32), start_pos.y), pixel_pos[j], color);
                  char_exist = true;
              }
              mode3::bitmap_xy(0, 5).write(Color::from_rgb(0, 255, 0));
              j += 1;
          }
          if char_exist {
              mode3::bitmap_xy(5, 0).write(Color::from_rgb(0, 255, 0));
              print_char(Vector2::new(start_pos.x + ((i * 6) as i32), start_pos.y), unasigned_char, color);
          }
          i += 1;
      }
  }

  fn print_char(start_pos:Vector2, pixel_pos:[i32; 25], color:Color) {
      let mut i = 0;
      let mut j;
      while i < 5 {
          j = 0;
          while j < 5 {
              if pixel_pos[i * j + i] == 1 {
                  mode3::bitmap_xy((start_pos.x + i as i32) as usize, (start_pos.y + j as i32)  as usize).write(color);
              } else {
                  mode3::bitmap_xy((start_pos.x + i as i32) as usize, (start_pos.y + j as i32)  as usize).write(Color::from_rgb(0, 0, 255));
              }
              j+= 1;
          }
          i+= 1;
      }
  }

  loop {
    spin_until_vblank();
    write_text("TOTOFAITDUVELO", Vector2::new(0, 0), Color::from_rgb(255, 0, 0));
    // let mut i = 0;
    // while i < (160*160*3) - 3 {
    //     mode3::bitmap_xy((i/3) / 160 as usize, (i/3) % 160 as usize).write(Color::from_rgb(poopa[i], poopa[i+1], poopa[i+2]));
    //     i+= 3;
    // }

    spin_until_vdraw();
  }
}
