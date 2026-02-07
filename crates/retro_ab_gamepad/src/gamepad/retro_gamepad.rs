use super::{
    gamepad_key_map::GamepadKeyMap,
    update_gamepad_state_handle::{connect_handle, disconnect_handle, pressed_button_handle},
};
use crate::devices_manager::{DeviceStateListener, DevicesRequireFunctions};
use gilrs::{Event, GamepadId, Gilrs};
use std::sync::{Arc, Mutex};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct RetroGamePad {
    pub id: Uuid,
    #[doc = "identificação do gamepad fornecida pelo crate gilrs"]
    pub inner_id: GamepadId,
    #[doc = "nome do gamepad"]
    pub name: String,
    #[doc = "indicar ao Core em qual porta o controle esta conectado, se o valor for -1 significa que todas as porta suportas pelo Core ja estão sendo usadas"]
    pub retro_port: i16,
    #[doc = "padrão RETRO_DEVICE_JOYPAD"]
    pub retro_type: u32,
    pub key_map: Vec<GamepadKeyMap>,
}

impl RetroGamePad {
    pub fn new(
        inner_id: GamepadId,
        name: String,
        retro_port: i16,
        retro_type: u32,
    ) -> RetroGamePad {
        Self {
            id: Uuid::new_v4(),
            inner_id,
            name,
            retro_port,
            retro_type,
            key_map: GamepadKeyMap::get_default_key_maps(),
        }
    }

    fn update_key_pressed(&mut self, gilrs: &Gilrs) {
        let gamepad = gilrs.gamepad(self.inner_id);

        for key_info in &mut self.key_map {
            key_info.pressed = gamepad.is_pressed(key_info.native);
        }
    }

    pub fn update(
        gilrs_instance: &Arc<Mutex<Gilrs>>,
        connected_gamepads: &Arc<Mutex<Vec<RetroGamePad>>>,
        max_ports: &Arc<Mutex<usize>>,
        listener: &Option<Arc<Mutex<DeviceStateListener>>>,
    ) {
        let gilrs = &mut *gilrs_instance.lock().unwrap();

        while let Some(Event { id, event, .. }) = gilrs.next_event() {
            match event {
                gilrs::EventType::Connected => {
                    connect_handle(id, gilrs, &connected_gamepads, &max_ports, &listener);
                }
                gilrs::EventType::Disconnected => {
                    disconnect_handle(id, &connected_gamepads, &listener)
                }
                gilrs::EventType::ButtonPressed(button, _) => {
                    pressed_button_handle(&button, id, &connected_gamepads, &listener)
                }
                _ => {}
            }

            for gamepad_info in &mut *connected_gamepads.lock().unwrap() {
                if gamepad_info.inner_id == id {
                    gamepad_info.update_key_pressed(&gilrs);
                }
            }
        }
    }
}

impl DevicesRequireFunctions for RetroGamePad {
    fn get_key_pressed(&self, key_id: i16) -> i16 {
        for key_map in &self.key_map {
            if key_map.retro as i16 == key_id {
                return if key_map.pressed { 1 } else { 0 };
            }
        }

        0
    }

    fn get_key_bitmask(&self) -> i16 {
        let mut bitmask = 0;

        for key in &self.key_map {
            let pressed = if key.pressed { 1 } else { 0 };
            bitmask += pressed << key.retro;
        }

        bitmask
    }
}
