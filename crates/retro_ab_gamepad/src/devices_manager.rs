use crate::{constants::DEFAULT_MAX_PORT, gamepad::retro_gamepad::RetroGamePad};
use gilrs::Gilrs;
use retro_ab::retro_sys::{retro_rumble_effect, RETRO_DEVICE_ID_JOYPAD_MASK};
use std::sync::{Arc, Mutex};
use uuid::Uuid;

#[derive(Debug)]
pub enum DeviceState {
    Connected,
    Disconnected,
    ButtonPressed(String),
}

#[derive(Debug, Clone, Copy)]
pub struct DeviceRubble {
    pub port: usize,
    pub effect: retro_rumble_effect,
    pub strength: u16,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum DeviceType {
    Gamepad,
    Keyboard,
}

#[derive(Debug, Clone, Eq)]
pub struct Device {
    pub id: Uuid,
    pub name: String,
    pub retro_port: i16,
    pub retro_type: u32,
    pub device_type: DeviceType,
}

impl PartialEq for Device {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}


impl Device {
    pub fn from_gamepad(gamepad: &RetroGamePad) -> Self {
        Self {
            id: gamepad.id.clone(),
            device_type: DeviceType::Gamepad,
            name: gamepad.name.clone(),
            retro_port: gamepad.retro_port,
            retro_type: gamepad.retro_type,
        }
    }
}

pub type DeviceStateListener = fn(DeviceState, Device);

#[derive(Debug, Clone)]
pub struct DevicesManager {
    gilrs_instance: Arc<Mutex<Gilrs>>,
    pub connected_gamepads: Arc<Mutex<Vec<RetroGamePad>>>,
    max_ports: Arc<Mutex<usize>>,
    listener: Option<Arc<Mutex<DeviceStateListener>>>,
}

impl DevicesManager {
    pub fn new(listener: Option<Arc<Mutex<DeviceStateListener>>>) -> Self {
        Self {
            gilrs_instance: Arc::new(Mutex::new(Gilrs::new().unwrap())),
            connected_gamepads: Arc::new(Mutex::new(Vec::new())),
            max_ports: Arc::new(Mutex::new(DEFAULT_MAX_PORT)),
            listener,
        }
    }

    pub fn set_listener(&mut self, listener: Arc<Mutex<DeviceStateListener>>) {
        self.listener = Some(listener);
    }

    pub fn update_state(&mut self) {
        RetroGamePad::update(
            &mut self.gilrs_instance,
            &self.connected_gamepads,
            &self.max_ports,
            &self.listener,
        );
    }

    pub fn set_max_port(&self, max_port: usize) {
        *self.max_ports.lock().unwrap() = max_port;
    }

    pub fn get_gamepads(&self) -> Vec<RetroGamePad> {
        let gamepads = self.connected_gamepads.lock().unwrap_or_else(|op| {
            let mut gamepads = op.into_inner();
            //TODO: o correto seria colocar uma lista verdadeira de gamepads aqui!
            *gamepads = Vec::new();
            gamepads
        });

        gamepads.clone()
    }

    pub fn get_input_state(&self, port: i16, key_id: i16) -> i16 {
        for gamepad in &*self.connected_gamepads.lock().unwrap() {
            if gamepad.retro_port == port {
                return if key_id as u32 != RETRO_DEVICE_ID_JOYPAD_MASK {
                    gamepad.get_key_pressed(key_id)
                } else {
                    gamepad.get_key_bitmask()
                };
            }
        }

        0
    }

    pub fn apply_rumble(&self, rubble: DeviceRubble) -> bool {
        println!("{:?}", rubble);
        true
    }
}

pub trait DevicesRequireFunctions {
    #[doc = "deve retornar 1 se estive pressionado e 0 se nao estive"]
    fn get_key_pressed(&self, key_id: i16) -> i16;

    fn get_key_bitmask(&self) -> i16;
}
