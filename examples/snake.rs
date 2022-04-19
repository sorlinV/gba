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

struct Snake {
    pos: [Vector2; 500],
    size: usize,
    color: Color,
    rng: RNG,
}

impl Snake {
    fn new() -> Self {
        Snake{
            pos: [Vector2::new((mode3::WIDTH / 2) as i32, (mode3::HEIGHT / 2) as i32); 500],
            size: 5,
            color: Color::from_rgb(255, 0, 0),
            rng: RNG::default()
        }
    }

    fn generate_fruit(&mut self) -> Vector2 {
        let mut rand_x = (&self.rng.next_u32() % mode3::WIDTH as u32) as i32;
        let mut rand_y = (&self.rng.next_u32() % mode3::HEIGHT as u32) as i32;
        let mut is_snake = true;
        let mut i = 0;
        while is_snake {
            is_snake = false;
            i = 0;
            while i < self.size {
                if self.pos[i].x == rand_x && self.pos[i].y == rand_y {
                    is_snake = true;
                    rand_x = (&self.rng.next_u32() % mode3::WIDTH as u32) as i32;
                    rand_y = (&self.rng.next_u32() % mode3::HEIGHT as u32) as i32;
                }
                i += 1;
            }
        }
        return Vector2::new(rand_x, rand_y);
    }

    fn new_pos(&mut self, mut pos: Vector2) {
        if pos.x >= mode3::WIDTH as i32 {
            pos.x = 0;
        } else if pos.x < 0 {
            pos.x = mode3::WIDTH as i32 - 1;
        }
        if pos.y >= mode3::HEIGHT as i32 {
            pos.y = 0;
        } else if pos.y < 0 {
            pos.y = mode3::HEIGHT as i32 - 1;
        }
        let mut temp_pos = self.pos;
        let mut i = 0;
        while i < self.size {
          temp_pos[i + 1] = self.pos[i];
          i += 1;
        }
        temp_pos[0] = pos;
        i = 0;
        while i < self.size {
          self.pos[i] = temp_pos[i];
          i += 1;
        }
    }

    fn render(&self) {
        let mut i = 0;
        while i < self.size {
          mode3::bitmap_xy(self.pos[i].x as usize, self.pos[i].y as usize).write(self.color);
          i += 1;
        }
    }

    fn is_died(&self) -> bool {
        let mut i = 1;
        while i < self.size {
          if self.pos[i].x == self.pos[0].x && self.pos[i].y == self.pos[0].y {
              return true;
          }
          i += 1;
        }
        return false;
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

  let mut snake = Snake::new();
  let mut dir = Vector2::new(0, 0);
  let mut lock_x = false;
  let mut lock_y = false;
  let mut fruit = snake.generate_fruit();
  let mut frame = 0;

  loop {
    frame += 1;
    // read our keys for this frame
    let keys: Keys = KEYINPUT.read().into();
    if keys.x_signum() != 0 && !lock_x{
        dir.x = keys.x_signum();
        dir.y = 0;
        lock_x = true;
        lock_y = false;
    }
    if keys.y_signum() != 0 && !lock_y{
        dir.x = 0;
        dir.y = keys.y_signum();
        lock_x = false;
        lock_y = true;
    }
    if frame % 3 == 0 {
        snake.new_pos(Vector2::new(snake.pos[0].x + dir.x, snake.pos[0].y + dir.y));
        if snake.pos[0].x == fruit.x && snake.pos[0].y == fruit.y {
            snake.size += 1;
            if snake.size >= 500 {
                snake.size = 500;
            }
            fruit = snake.generate_fruit();
        }
    }

    // color = Color(color.0.rotate_left(5));
    if keys.a() {
        snake.size += 1;
        snake.color = Color(snake.color.0.rotate_right(5));
    }
    if keys.start() || snake.is_died() {
      snake = Snake::new();
      dir = Vector2::new(0, 0);
      lock_x = false;
      lock_y = false;
      fruit = snake.generate_fruit();
    }
    spin_until_vblank();

    mode3::dma3_clear_to(Color::from_rgb(255, 255, 255));
    mode3::bitmap_xy(fruit.x as usize, fruit.y as usize).write(Color::from_rgb(0, 255, 0));
    snake.render();

    spin_until_vdraw();
  }
}
