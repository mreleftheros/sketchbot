use serde::{Serialize, Deserialize};
use winit::event::ElementState;
use std::fs::File;
use std::io::{self, Write};
use winit::{
    event::{Event, WindowEvent, MouseButton, KeyEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
    keyboard::Key
};

#[derive(Serialize, Deserialize)]
struct MouseEvent {
    x: i32,
    y: i32,
    is_start: bool
}

fn main() -> io::Result<()> {
    let event_loop = EventLoop::new().unwrap();
    let window = WindowBuilder::new().with_transparent(true).build(&event_loop).unwrap();

    let mut events: Vec<MouseEvent> = Vec::new();
    let mut is_pressed = false;
    let mut is_start = true;

    event_loop.set_control_flow(ControlFlow::Wait);

    event_loop.run( |event, elwt| {
      if let Event::WindowEvent{event, ..} = event {
        match event {
          WindowEvent::MouseInput { state, button: MouseButton::Left, .. } => {
            match state {
              ElementState::Pressed => is_pressed = true,
              ElementState::Released => is_pressed = false
            }
          }
          WindowEvent::CursorMoved { position, .. } => {
            if is_pressed {
              events.push(MouseEvent {
                  x: position.x as i32,
                  y: position.y as i32,
                  is_start
              });
              is_start = false;
            } else {
              is_start = true;
            }
        }
        WindowEvent::CloseRequested => {
            elwt.exit();
        }
        _ => (),
        }
      }
    }).unwrap();

    // Serialize mouse events and save to a file
    let serialized = serde_json::to_string(&events)?;
    let mut file = File::create("sketches/recorded.json")?;
    file.write_all(serialized.as_bytes())?;

    Ok(())
}