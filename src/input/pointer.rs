use std::process::{Command, Stdio};

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
        let terminal_needed = desktop_item.data.terminal.unwrap_or(false);
        let exec = &desktop_item
            .data
            .exec
            .as_ref()
            .unwrap()
            .split_whitespace()
            .next()
            .unwrap();

        println!("Launching {}...", exec);

        _ = Command::new(if terminal_needed { "kitty" } else { "sh" })
            .args([if terminal_needed { "-e" } else { "-c" }, exec])
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .unwrap();

        // println!("command: {:?}", command_builder);
    };

    println!()
}
