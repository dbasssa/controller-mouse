mod tray;
use std::time::Duration;
use std::thread::sleep;

use enigo::{Enigo, Mouse, Settings, Coordinate, Direction};
use gilrs:: {Gilrs, Button, Event, EventType, Axis};

fn main() {
    let mut gilrs = Gilrs::new().unwrap();
    let mut active: Option<gilrs::GamepadId> = None;
    let mut enigo = Enigo::new(&Settings::default()).unwrap();
    let mut speed = 20.0;
    let mut scroll_speed = 1.0;

    let _handle = tray::spawn();
    const DEADZONE: f32 = 0.11;
    loop {
        while let Some(Event {id, event, .. }) = gilrs.next_event() {
            active = Some(id);
            match event {
                EventType::ButtonPressed(Button::South, _) => {enigo.button(enigo::Button::Left, Direction::Press).expect("msg")} , 
                EventType::ButtonReleased(Button::South, _) => {enigo.button(enigo::Button::Left, Direction::Release).expect("msg")}
                _ => ()
            }
        }
        if let Some(id) = active {
            let gamepad = gilrs.gamepad(id);
            let x = gamepad.value(Axis::LeftStickX);
            let x = if x.abs() < DEADZONE {0.0} else {x};
            let y = gamepad.value(Axis::LeftStickY);
            let y = if y.abs() < DEADZONE {0.0} else {y};
            let right_y = gamepad.value(Axis::RightStickY);
            let right_y = if right_y.abs() < DEADZONE {0.0} else {right_y};
            enigo.move_mouse((x * speed) as i32,(-y * speed) as i32, Coordinate::Rel);
            let scroll = (right_y * scroll_speed).round() as i32;
            if scroll != 0 {enigo.scroll(-scroll,enigo::Axis::Vertical);}
        }
        sleep(Duration::from_millis(16))
    }
}
