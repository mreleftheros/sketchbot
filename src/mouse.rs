use enigo::*;
use crate::sleep;

pub fn move_to(enigo: &mut Enigo, x: i32, y: i32) {
  enigo.mouse_up(MouseButton::Left);
  sleep::sleep_ms(15);
  enigo.mouse_move_to(x, y);
  sleep::sleep_ms(15);
  enigo.mouse_down(MouseButton::Left);
  sleep::sleep_ms(15);
}

pub fn move_and_draw(enigo: &mut Enigo, x: i32, y: i32) {
  enigo.mouse_move_to(x, y);
  sleep::sleep_ms(1); // Adjust sleep duration for smoother movement
}

pub fn smooth_mouse_move(enigo: &mut Enigo, x: i32, y: i32) {
  // Current mouse position
  let (mut current_x, mut current_y) = enigo.mouse_location();

  // Calculate the steps needed to move from current position to target position
  let steps = 15; // Number of steps
  let dx = (x - current_x) as f64 / steps as f64;
  let dy = (y - current_y) as f64 / steps as f64;

  for _ in 0..steps {
      current_x += dx as i32;
      current_y += dy as i32;
      move_and_draw(enigo, current_x, current_y);
  }

  // Move to the final position to ensure accuracy
  move_and_draw(enigo, x, y);
}