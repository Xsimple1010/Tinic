use crate::gamepad::retro_gamepad::RetroGamePad;
use generics::{
    constants::DEFAULT_MAX_PORT,
    error_handle::ErrorHandle,
    types::{ArcTMutex, TMutex},
};
use gilrs::Gilrs;
use libretro_sys::binding_libretro::{retro_rumble_effect, RETRO_DEVICE_ID_JOYPAD_MASK};
use std::{
    fmt::Debug,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
};
use uuid::Uuid;

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
            id: gamepad.id,
            device_type: DeviceType::Gamepad,
            name: gamepad.name.clone(),
            retro_port: gamepad.retro_port,
            retro_type: gamepad.retro_type,
        }
    }
}

pub type DeviceStateListener = ArcTMutex<Box<dyn DeviceListener>>;

#[derive(Debug, Clone)]
pub struct DevicesManager {
    gilrs: ArcTMutex<Gilrs>,
    connected_gamepads: ArcTMutex<Vec<RetroGamePad>>,
    max_ports: Arc<AtomicUsize>,
    listener: DeviceStateListener,
}

pub trait DeviceListener: Debug + Send {
    fn connected(&self, device: Device);
    fn disconnected(&self, device: Device);
    fn button_pressed(&self, button: String, device: Device);
}

impl DevicesManager {
    pub fn new(listener: Box<dyn DeviceListener>) -> Result<Self, ErrorHandle> {
        let gilrs = match Gilrs::new() {
            Ok(gilrs) => gilrs,
            Err(e) => {
                return Err(ErrorHandle {
                    message: e.to_string(),
                })
            }
        };
        Ok(Self {
            gilrs: TMutex::new(gilrs),
            connected_gamepads: TMutex::new(Vec::new()),
            max_ports: Arc::new(AtomicUsize::new(DEFAULT_MAX_PORT)),
            listener: TMutex::new(listener),
        })
    }

    pub fn update_state(&self) -> Result<(), ErrorHandle> {
        RetroGamePad::update(
            &mut *self.gilrs.try_load()?,
            &self.connected_gamepads,
            &self.max_ports,
            &self.listener,
        )
    }

    pub fn set_max_port(&self, max_port: usize) {
        self.max_ports.store(max_port, Ordering::SeqCst);
    }

    pub fn get_gamepads(&self) -> Vec<RetroGamePad> {
        //TODO: o correto seria colocar uma lista verdadeira de gamepads aqui!
        let gamepads = self.connected_gamepads.load_or(Vec::new());

        gamepads.clone()
    }

    pub fn get_input_state(&self, port: i16, key_id: i16) -> i16 {
        for gamepad in &*self.connected_gamepads.load_or(Vec::new()) {
            if gamepad.retro_port == port {
                return if key_id as u32 != RETRO_DEVICE_ID_JOYPAD_MASK {
                    gamepad.get_key_pressed(key_id)
                } else {
                    gamepad.get_key_bitmasks()
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

pub trait DevicesRequiredFunctions {
    #[doc = "deve retornar 1 se estive pressionado e 0 se nao estive"]
    fn get_key_pressed(&self, key_id: i16) -> i16;

    fn get_key_bitmasks(&self) -> i16;
}
