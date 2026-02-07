use super::{gamepad_key_map::GamepadKeyMap, retro_gamepad::RetroGamePad};
use crate::devices_manager::{Device, DeviceState, DeviceStateListener};
use generics::{constants::INVALID_CONTROLLER_PORT, erro_handle::ErroHandle, types::ArcTMuxte};
use gilrs::{Button, GamepadId, Gilrs};
use libretro_sys::binding_libretro::RETRO_DEVICE_JOYPAD;
use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
};

//Se o valor retornado for -1(INVALID_CONTROLLER_PORT) significa que todas as
//portas suportas pelo Core ja estão sendo usadas.
fn get_available_port(
    max_ports: &Arc<AtomicUsize>,
    connected_gamepads: &ArcTMuxte<Vec<RetroGamePad>>,
) -> i16 {
    let mut connected_gamepads = connected_gamepads.load_or(Vec::new());

    connected_gamepads.sort_by(|gmp, f_gmp| gmp.retro_port.cmp(&f_gmp.retro_port));

    if let Some(gamepad) = connected_gamepads.last() {
        let current_port = gamepad.retro_port + 1;

        if current_port as usize > max_ports.load(Ordering::SeqCst) {
            return INVALID_CONTROLLER_PORT;
        }

        return current_port;
    }

    0
}

pub fn remove(
    id: GamepadId,
    connected_gamepads: &ArcTMuxte<Vec<RetroGamePad>>,
) -> Result<Option<RetroGamePad>, ErroHandle> {
    let list = &mut connected_gamepads.try_load()?;

    let mut gm_list = list.clone();
    gm_list.retain(|gm| gm.inner_id == id);

    list.retain(|g| g.inner_id != id);

    match gm_list.first() {
        Some(gm) => Ok(Some(gm.clone())),
        None => Ok(None),
    }
}

pub fn connect_handle(
    gamepad_id: GamepadId,
    gilrs: &mut Gilrs,
    connected_gamepads: &ArcTMuxte<Vec<RetroGamePad>>,
    max_ports: &Arc<AtomicUsize>,
    listener: &ArcTMuxte<DeviceStateListener>,
) {
    if let Some(gamepad) = gilrs.connected_gamepad(gamepad_id) {
        let port = get_available_port(max_ports, connected_gamepads);

        let gamepad = RetroGamePad::new(
            gamepad_id,
            gamepad.name().to_string(),
            port,
            RETRO_DEVICE_JOYPAD,
        );

        let mut gamepads = connected_gamepads.load_or(Vec::new());
        gamepads.push(gamepad.clone());

        let listener = listener.load_or(|_, _| {});
        listener(DeviceState::Connected, Device::from_gamepad(&gamepad));
    }
}

pub fn disconnect_handle(
    id: GamepadId,
    connected_gamepads: &ArcTMuxte<Vec<RetroGamePad>>,
    listener: &ArcTMuxte<DeviceStateListener>,
) -> Result<(), ErroHandle> {
    if let Some(gamepad) = remove(id, connected_gamepads)? {
        let listener = listener.load_or(|_, _| {});
        listener(DeviceState::Disconnected, Device::from_gamepad(&gamepad));
    }

    Ok(())
}

pub fn pressed_button_handle(
    button: &Button,
    gamepad_id: GamepadId,
    connected_gamepads: &ArcTMuxte<Vec<RetroGamePad>>,
    listener: &ArcTMuxte<DeviceStateListener>,
) {
    for gamepad in &mut *connected_gamepads.load_or(Vec::new()) {
        if gamepad.inner_id != gamepad_id {
            return;
        }

        let listener = listener.load_or(|_, _| {});

        listener(
            DeviceState::ButtonPressed(
                GamepadKeyMap::get_key_name_from_native_button(button).to_owned(),
            ),
            Device::from_gamepad(gamepad),
        );
    }
}
