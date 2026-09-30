use super::update_gamepad_state_handle::{
    axis_change_handle, connect_handle, disconnect_handle, pressed_button_handle,
};
use crate::devices_manager::{DeviceKeyMap, DeviceStateListener, DevicesRequiredFunctions};
use crate::gamepad::retro_gamepad_key_map::GamePadKeyMap;
use gilrs::{Event, GamepadId, Gilrs};
use libretro_sys::binding_libretro::{
    RETRO_DEVICE_ID_ANALOG_X, RETRO_DEVICE_ID_ANALOG_Y, RETRO_DEVICE_INDEX_ANALOG_LEFT,
    RETRO_DEVICE_INDEX_ANALOG_RIGHT,
};
use std::sync::{Arc, atomic::AtomicUsize};
use tinic_generics::error_handle::TinicResult;
use tinic_generics::types::ArcTMutex;
use tinic_ipc_protocol::out::GamePadAxis;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct RetroGamePad {
    pub id: Uuid,
    #[doc = "identificação do gamepad fornecida pelo crate gilrs"]
    pub inner_id: GamepadId,
    #[doc = "nome do gamepad"]
    pub name: String,
    #[doc = "
        indicar ao Core em qual porta o controle esta conectado, se o valor for
        -1(INVALID_CONTROLLER_PORT) significa que todas as porta suportas pelo Core
        ja estão sendo usadas
    "]
    pub retro_port: i16,
    #[doc = "padrão RETRO_DEVICE_JOYPAD"]
    pub retro_type: u32,
    pub key_map: Vec<GamePadKeyMap>,
    pub axis: GamePadAxis,
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
            key_map: GamePadKeyMap::get_default_key_maps(),
            axis: GamePadAxis::default(),
        }
    }

    fn update_key_pressed_bitmarsk(&mut self, gilrs: &Gilrs) {
        let gamepad = gilrs.gamepad(self.inner_id);

        for key_info in &mut self.key_map {
            key_info.pressed = gamepad.is_pressed(key_info.native);
        }
    }

    pub fn update(
        gilrs: &mut Gilrs,
        connected_gamepads: &ArcTMutex<Vec<RetroGamePad>>,
        max_ports: &Arc<AtomicUsize>,
        listener: &DeviceStateListener,
    ) -> TinicResult<()> {
        while let Some(Event { id, event, .. }) = gilrs.next_event() {
            match event {
                gilrs::EventType::Connected => {
                    connect_handle(id, gilrs, connected_gamepads, max_ports, listener)?;
                }
                gilrs::EventType::Disconnected => {
                    disconnect_handle(id, connected_gamepads, listener)?;
                }
                gilrs::EventType::ButtonPressed(button, _) => {
                    pressed_button_handle(&button, id, connected_gamepads, listener)?;
                }
                gilrs::EventType::AxisChanged(axis, axis_value, _) => {
                    axis_change_handle(axis, axis_value, id, connected_gamepads, listener)?;
                }
                _ => {}
            }

            for gamepad_info in &mut *connected_gamepads.load_or(Vec::new()) {
                if gamepad_info.inner_id == id {
                    gamepad_info.update_key_pressed_bitmarsk(gilrs);
                }
            }
        }

        Ok(())
    }
}

impl DevicesRequiredFunctions for RetroGamePad {
    fn get_key_pressed(&self, key_id: i16) -> i16 {
        for key_map in &self.key_map {
            if key_map.retro as i16 == key_id {
                return if key_map.pressed { 1 } else { 0 };
            }
        }

        0
    }

    fn get_key_bitmasks(&self) -> i16 {
        let mut bitmasks = 0;

        for key in &self.key_map {
            let pressed = if key.pressed { 1 } else { 0 };
            bitmasks += pressed << key.retro;
        }

        bitmasks
    }

    fn get_analog_value(&self, index: u32, id: u32) -> i16 {
        let raw: f32 = match (index, id) {
            (RETRO_DEVICE_INDEX_ANALOG_LEFT, RETRO_DEVICE_ID_ANALOG_X) => self.axis.left_stick_x,
            (RETRO_DEVICE_INDEX_ANALOG_LEFT, RETRO_DEVICE_ID_ANALOG_Y) => self.axis.left_stick_y,
            (RETRO_DEVICE_INDEX_ANALOG_RIGHT, RETRO_DEVICE_ID_ANALOG_X) => self.axis.right_stick_x,
            (RETRO_DEVICE_INDEX_ANALOG_RIGHT, RETRO_DEVICE_ID_ANALOG_Y) => self.axis.right_stick_y,
            // RETRO_DEVICE_INDEX_ANALOG_BUTTON: id aqui é um RETRO_DEVICE_ID_JOYPAD_* (L2/R2 etc)
            // (RETRO_DEVICE_INDEX_ANALOG_BUTTON, _) => gamepad.get_analog_button_pressure(id),
            _ => 0.0,
        };

        (raw.clamp(-1.0, 1.0) * 0x7fff as f32) as i16
    }
}
