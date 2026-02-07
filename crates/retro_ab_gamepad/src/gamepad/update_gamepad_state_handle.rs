use super::{gamepad_key_map::GamepadKeyMap, retro_gamepad::RetroGamePad};
use crate::devices_manager::{Device, DeviceState, DeviceStateListener};
use gilrs::{Button, GamepadId, Gilrs};
use retro_ab::retro_sys::RETRO_DEVICE_JOYPAD;
use std::sync::{Arc, Mutex};

//se o valor retornado for -1 significa que todas as portas suportas pelo Core ja estão sendo usadas
fn get_available_port(
    max_ports: &Arc<Mutex<usize>>,
    connected_gamepads: &Arc<Mutex<Vec<RetroGamePad>>>,
) -> i16 {
    let invalid_port = -1;

    let mut connected_gamepads = connected_gamepads.lock().unwrap();

    connected_gamepads.sort_by(|gmp, f_gmp| gmp.retro_port.cmp(&f_gmp.retro_port));

    if let Some(gamepad) = connected_gamepads.last() {
        let current_port = gamepad.retro_port + 1;

        if current_port as usize > *max_ports.lock().unwrap() {
            return invalid_port;
        }

        return current_port;
    }

    0
}

pub fn remove(
    id: GamepadId,
    connected_gamepads: &Arc<Mutex<Vec<RetroGamePad>>>,
) -> Result<RetroGamePad, ()> {
    if let Ok(list) = &mut connected_gamepads.lock() {
        let mut gm_list = list.clone();
        gm_list.retain(|gm| gm.inner_id == id);

        list.retain(|g| g.inner_id != id);

        return if gm_list.is_empty() {
            Err(())
        } else {
            Ok(gm_list.first().unwrap().to_owned())
        };
    }

    Err(())
}

pub fn connect_handle(
    gamepad_id: GamepadId,
    gilrs: &mut Gilrs,
    connected_gamepads: &Arc<Mutex<Vec<RetroGamePad>>>,
    max_ports: &Arc<Mutex<usize>>,
    listener: &Option<Arc<Mutex<DeviceStateListener>>>,
) {
    if let Some(gamepad) = gilrs.connected_gamepad(gamepad_id) {
        let port = get_available_port(&max_ports, &connected_gamepads);

        let gamepad = RetroGamePad::new(
            gamepad_id,
            gamepad.name().to_string(),
            port,
            RETRO_DEVICE_JOYPAD,
        );

        let mut gamepads = connected_gamepads.lock().unwrap();
        gamepads.push(gamepad.clone());

        if let Some(listener) = listener {
            let listener = listener.lock().unwrap();
            listener(DeviceState::Connected, Device::from_gamepad(&gamepad));
        }
    }
}

pub fn disconnect_handle(
    id: GamepadId,
    connected_gamepads: &Arc<Mutex<Vec<RetroGamePad>>>,
    listener: &Option<Arc<Mutex<DeviceStateListener>>>,
) {
    if let Ok(gamepad) = remove(id, &connected_gamepads) {
        if let Some(listener) = listener {
            let listener = listener.lock().unwrap();
            listener(DeviceState::Disconnected, Device::from_gamepad(&gamepad));
        }
    }
}

pub fn pressed_button_handle(
    button: &Button,
    gamepad_id: GamepadId,
    connected_gamepads: &Arc<Mutex<Vec<RetroGamePad>>>,
    listener: &Option<Arc<Mutex<DeviceStateListener>>>,
) {
    for gamepad in &mut *connected_gamepads.lock().unwrap() {
        if gamepad.inner_id != gamepad_id {
            return;
        }

        if let Some(listener) = listener {
            let listener = listener.lock().unwrap();

            listener(
                DeviceState::ButtonPressed(
                    GamepadKeyMap::get_key_name_from_native_button(&button).to_owned(),
                ),
                Device::from_gamepad(&gamepad),
            );
        }
    }
}
