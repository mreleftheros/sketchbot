pub mod sleep;
mod mouse;

use enigo::*;
use sleep::{sleep_ms, sleep_s};
use std::{fs, io, thread, time::Duration};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Point {
  x: i32,
  y: i32,
  is_start: bool
}

#[derive(Debug)]
pub struct SketchBot {
  x: i32,
  y: i32
}

impl SketchBot {
  pub fn new() -> Self {
    Self {
      x: 1150,
      y: 400,
    }
  }

  pub fn sketch_squares(&self) {
    let mut enigo = Enigo::new();
    
    for i in 0..=200 {
      if i % 2 == 0 {
        mouse::move_to(&mut enigo, self.x + i, self.y + i);

        mouse::smooth_mouse_move(&mut enigo, self.x + i, self.y - i);
        mouse::smooth_mouse_move(&mut enigo, self.x - i, self.y - i);
        mouse::smooth_mouse_move(&mut enigo, self.x - i, self.y + i);
        mouse::smooth_mouse_move(&mut enigo, self.x + i, self.y + i);
      }
    }

    enigo.mouse_up(MouseButton::Left);
  }

  pub fn sketch_from_file(&self, filename: &str) -> io::Result<()> {
    let mut enigo = Enigo::new();

    let file = fs::read_to_string(filename)?;
  
    let mut points: Vec<Point> = serde_json::from_str(&file)?;
    let sketch_x = points[0].x;
    let sketch_y = points[0].y;

    let diff_x = self.x - sketch_x;
    let diff_y = self.y - sketch_y;
   
    mouse::move_to(&mut enigo, self.x, self.y);
    
    for point in points.iter().skip(1) {
      if point.is_start {
        mouse::move_to(&mut enigo, point.x + diff_x, point.y + diff_y);
      } else {
        mouse::move_and_draw(&mut enigo, point.x + diff_x, point.y + diff_y);
      }
    }
    enigo.mouse_up(MouseButton::Left);

    Ok(())
  }
}