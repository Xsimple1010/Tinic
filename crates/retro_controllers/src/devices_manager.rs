use crate::gamepad::retro_gamepad::RetroGamePad;
use crate::keyboard::Keyboard;
use generics::{
    constants::DEFAULT_MAX_PORT,
    error_handle::ErrorHandle,
    types::{ArcTMutex, TMutex},
};
use gilrs::Gilrs;
use libretro_sys::binding_libretro;
use libretro_sys::binding_libretro::{RETRO_DEVICE_ID_JOYPAD_MASK, retro_rumble_effect};
use std::{
    fmt::Debug,
    sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    },
};
use winit::keyboard::PhysicalKey;

#[derive(Debug, Clone, Copy)]
pub struct DeviceRubble {
    pub port: usize,
    pub effect: retro_rumble_effect,
    pub strength: u16,
}

pub type DeviceStateListener = ArcTMutex<Box<dyn DeviceListener>>;

#[derive(Debug, Clone)]
pub struct DevicesManager {
    gilrs: ArcTMutex<Gilrs>,
    connected_gamepads: ArcTMutex<Vec<RetroGamePad>>,
    keyboard: ArcTMutex<Option<Keyboard>>,
    max_ports: Arc<AtomicUsize>,
    listener: DeviceStateListener,
}

pub trait DeviceListener: Debug + Send {
    fn connected(&self, device: RetroGamePad);
    fn disconnected(&self, device: RetroGamePad);
    fn button_pressed(&self, button: String, device: RetroGamePad);
}

pub trait DeviceKeyMap<K, B> {
    fn get_key_name_from_retro_button<'a>(retro: u32) -> &'a str {
        match retro {
            //DPads
            binding_libretro::RETRO_DEVICE_ID_JOYPAD_DOWN => "Retro DPad-down",
            binding_libretro::RETRO_DEVICE_ID_JOYPAD_UP => "Retro DPad-up",
            binding_libretro::RETRO_DEVICE_ID_JOYPAD_LEFT => "Retro DPad-left",
            binding_libretro::RETRO_DEVICE_ID_JOYPAD_RIGHT => "Retro DPad-right",

            //buttons
            binding_libretro::RETRO_DEVICE_ID_JOYPAD_B => "Retro B",
            binding_libretro::RETRO_DEVICE_ID_JOYPAD_A => "Retro A",
            binding_libretro::RETRO_DEVICE_ID_JOYPAD_X => "Retro X",
            binding_libretro::RETRO_DEVICE_ID_JOYPAD_Y => "Retro Y",

            //Trigger
            binding_libretro::RETRO_DEVICE_ID_JOYPAD_L => "Retro L",
            binding_libretro::RETRO_DEVICE_ID_JOYPAD_R => "Retro R",
            binding_libretro::RETRO_DEVICE_ID_JOYPAD_L2 => "Retro L2",
            binding_libretro::RETRO_DEVICE_ID_JOYPAD_R2 => "Retro R2",

            //Thumb
            binding_libretro::RETRO_DEVICE_ID_JOYPAD_L3 => "Retro L3",
            binding_libretro::RETRO_DEVICE_ID_JOYPAD_R3 => "Retro R3",

            //Menu
            binding_libretro::RETRO_DEVICE_ID_JOYPAD_START => "Retro Start",
            binding_libretro::RETRO_DEVICE_ID_JOYPAD_SELECT => "Retro Select",
            _ => "Chave desconhecida",
        }
    }

    fn get_key_name_from_native_button<'a>(native: &B) -> &'a str;

    fn get_default_key_maps() -> Vec<K>;
}

impl DevicesManager {
    pub fn new(listener: Box<dyn DeviceListener>) -> Result<Self, ErrorHandle> {
        let gilrs = match Gilrs::new() {
            Ok(gilrs) => gilrs,
            Err(e) => {
                return Err(ErrorHandle {
                    message: e.to_string(),
                });
            }
        };
        Ok(Self {
            gilrs: TMutex::new(gilrs),
            connected_gamepads: TMutex::new(Vec::new()),
            max_ports: Arc::new(AtomicUsize::new(DEFAULT_MAX_PORT)),
            listener: TMutex::new(listener),
            keyboard: TMutex::new(None),
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

    pub fn update_keyboard(&self, native: PhysicalKey, pressed: bool) {
        if let Some(keyboard) = &mut *self.keyboard.load_or(None) {
            keyboard.set_key_pressed(native, pressed);
        }
    }

    pub fn active_keyboard(&self) -> Keyboard {
        let keyboard = Keyboard::new();
        self.keyboard.store(Some(keyboard.clone()));
        keyboard
    }

    pub fn disable_keyboard(&self) {
        self.keyboard.store(None);
    }

    pub fn is_using_keyboard(&self) -> bool {
        self.keyboard.load_or(None).is_some()
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
        if let Some(keyboard) = &*self.keyboard.load_or(None) && keyboard.retro_port.eq(&port) {
            return if key_id as u32 != RETRO_DEVICE_ID_JOYPAD_MASK {
                keyboard.get_key_pressed(key_id)
            } else {
                keyboard.get_key_bitmasks()
            }
        }

        for gamepad in &*self.connected_gamepads.load_or(Vec::new()) {
            if gamepad.retro_port.eq(&port) {
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
