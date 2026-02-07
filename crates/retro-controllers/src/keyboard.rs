use crate::devices_manager::{DeviceKeyMap, DevicesRequiredFunctions};
use libretro_sys::binding_libretro;
use libretro_sys::binding_libretro::RETRO_DEVICE_JOYPAD;
use winit::keyboard::{KeyCode, PhysicalKey};

#[derive(Debug, Clone)]
pub struct Keyboard {
    pub retro_port: i16,
    #[doc = "padrão RETRO_DEVICE_JOYPAD"]
    pub retro_type: u32,
    key_map: Vec<KeyboardKeyMap>,
}

impl Keyboard {
    pub fn new() -> Self {
        Self {
            retro_port: 0,
            retro_type: RETRO_DEVICE_JOYPAD,
            key_map: KeyboardKeyMap::get_default_key_maps(),
        }
    }

    pub fn set_key_pressed(&mut self, native: PhysicalKey, pressed: bool) {
        for key_map in &mut self.key_map {
            if key_map.native == native {
                key_map.pressed = pressed;
            }
        }
    }
}

impl DevicesRequiredFunctions for Keyboard {
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
}

#[derive(Debug, Clone)]
struct KeyboardKeyMap {
    pub native: PhysicalKey,
    pub retro: u32,
    pub pressed: bool,
}

impl KeyboardKeyMap {
    fn new(native: PhysicalKey, retro: u32) -> Self {
        Self {
            native,
            retro,
            pressed: false,
        }
    }
}

impl DeviceKeyMap<KeyboardKeyMap, PhysicalKey> for KeyboardKeyMap {
    fn get_key_name_from_native_button<'a>(_native: &PhysicalKey) -> &'a str {
        todo!()
    }

    fn get_default_key_maps() -> Vec<KeyboardKeyMap> {
        vec![
            //DPads
            KeyboardKeyMap::new(
                PhysicalKey::Code(KeyCode::KeyS),
                binding_libretro::RETRO_DEVICE_ID_JOYPAD_DOWN,
            ),
            KeyboardKeyMap::new(
                PhysicalKey::Code(KeyCode::KeyA),
                binding_libretro::RETRO_DEVICE_ID_JOYPAD_LEFT,
            ),
            KeyboardKeyMap::new(
                PhysicalKey::Code(KeyCode::KeyW),
                binding_libretro::RETRO_DEVICE_ID_JOYPAD_UP,
            ),
            KeyboardKeyMap::new(
                PhysicalKey::Code(KeyCode::KeyD),
                binding_libretro::RETRO_DEVICE_ID_JOYPAD_RIGHT,
            ),
            //buttons
            KeyboardKeyMap::new(
                PhysicalKey::Code(KeyCode::Space),
                binding_libretro::RETRO_DEVICE_ID_JOYPAD_B,
            ),
            KeyboardKeyMap::new(
                PhysicalKey::Code(KeyCode::KeyL),
                binding_libretro::RETRO_DEVICE_ID_JOYPAD_A,
            ),
            KeyboardKeyMap::new(
                PhysicalKey::Code(KeyCode::KeyJ),
                binding_libretro::RETRO_DEVICE_ID_JOYPAD_X,
            ),
            KeyboardKeyMap::new(
                PhysicalKey::Code(KeyCode::KeyI),
                binding_libretro::RETRO_DEVICE_ID_JOYPAD_Y,
            ),
            //Trigger
            KeyboardKeyMap::new(
                PhysicalKey::Code(KeyCode::KeyQ),
                binding_libretro::RETRO_DEVICE_ID_JOYPAD_L,
            ),
            KeyboardKeyMap::new(
                PhysicalKey::Code(KeyCode::KeyE),
                binding_libretro::RETRO_DEVICE_ID_JOYPAD_R,
            ),
            KeyboardKeyMap::new(
                PhysicalKey::Code(KeyCode::KeyU),
                binding_libretro::RETRO_DEVICE_ID_JOYPAD_L2,
            ),
            KeyboardKeyMap::new(
                PhysicalKey::Code(KeyCode::KeyO),
                binding_libretro::RETRO_DEVICE_ID_JOYPAD_R2,
            ),
            //Thumb
            KeyboardKeyMap::new(
                PhysicalKey::Code(KeyCode::KeyC),
                binding_libretro::RETRO_DEVICE_ID_JOYPAD_L3,
            ),
            KeyboardKeyMap::new(
                PhysicalKey::Code(KeyCode::KeyN),
                binding_libretro::RETRO_DEVICE_ID_JOYPAD_R3,
            ),
            //Menu
            KeyboardKeyMap::new(
                PhysicalKey::Code(KeyCode::Enter),
                binding_libretro::RETRO_DEVICE_ID_JOYPAD_START,
            ),
            KeyboardKeyMap::new(
                PhysicalKey::Code(KeyCode::Backspace),
                binding_libretro::RETRO_DEVICE_ID_JOYPAD_SELECT,
            ),
        ]
    }
}
