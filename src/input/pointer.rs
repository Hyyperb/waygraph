use std::process::Command;

use crate::AppData;
use wayland_client::{WEnum, protocol::wl_pointer::ButtonState};

pub fn handle_motion(state: &mut AppData, x: f64, y: f64, click_time: u32) {
    state.app.pointer.last_x = x;
    state.app.pointer.last_y = y;
}
pub fn handle_button(
    state: &mut AppData,
    button: u32,
    button_state: WEnum<ButtonState>,
    click_time: u32,
) {
    if button_state == WEnum::Value(ButtonState::Pressed) {
        if click_time - state.app.pointer.last_time <= state.app.config.double_click_timeout {
            handle_double_click(state, button, click_time);
        }
        state.app.pointer.last_time = click_time;
    }
}

#[allow(clippy::zombie_processes)]
pub fn handle_double_click(state: &mut AppData, button: u32, click_time: u32) {
    let last_x = state.app.pointer.last_x;
    let last_y = state.app.pointer.last_y;

    if let Some(item_index) = state.app.desktop.layout.get_index_of_item_under_position(
        &state.app.desktop.items,
        last_x as f32,
        last_y as f32,
    ) {
        let desktop_item = state.app.desktop.items.get(item_index).unwrap();
        let exec = &desktop_item.data.exec.as_ref().unwrap().replace("%f", "");
        println!("Launching {}...", exec);

        let mut command_builder = Command::new("sh");

        command_builder.args(["-c", exec]);

        println!("command: {:?}", command_builder);
        let status = command_builder.spawn().unwrap();
    };

    println!()
}
