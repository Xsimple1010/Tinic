use crate::devices_manager::{DeviceKeyMap, DevicesRequiredFunctions};
use libretro_sys::binding_libretro;
use libretro_sys::binding_libretro::RETRO_DEVICE_JOYPAD;
use winit::keyboard::{KeyCode, PhysicalKey};

#[derive(Debug, Clone)]
pub struct Keyboard {
    pub retro_port: i16,
    #[doc = "padrão RETRO_DEVICE_JOYPAD"]
    pub retro_type: u32,
    pub key_map: Vec<KeyboardKeymap>,
}

impl Keyboard {
    pub fn new() -> Self {
        Self {
            retro_port: 0,
            retro_type: RETRO_DEVICE_JOYPAD,
            key_map: KeyboardKeymap::get_default_key_maps(),
        }
    }

    pub fn set_key_pressed(&mut self, native: PhysicalKey, pressed: bool) {
        for keymap in &mut self.key_map {
            if keymap.native == native {
                keymap.pressed = pressed;
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
struct KeyboardKeymap {
    pub native: PhysicalKey,
    pub retro: u32,
    pub pressed: bool,
}

impl KeyboardKeymap {
    fn new(native: PhysicalKey, retro: u32) -> Self {
        Self {
            native,
            retro,
            pressed: false,
        }
    }
}

impl DeviceKeyMap<KeyboardKeymap, PhysicalKey> for KeyboardKeymap {
    fn get_key_name_from_native_button<'a>(native: &PhysicalKey) -> &'a str {
        todo!()
    }

    fn get_default_key_maps() -> Vec<KeyboardKeymap> {
        vec![
            //DPads
            KeyboardKeymap::new(
                PhysicalKey::Code(KeyCode::KeyS),
                binding_libretro::RETRO_DEVICE_ID_JOYPAD_DOWN,
            ),
            KeyboardKeymap::new(
                PhysicalKey::Code(KeyCode::KeyA),
                binding_libretro::RETRO_DEVICE_ID_JOYPAD_LEFT,
            ),
            KeyboardKeymap::new(
                PhysicalKey::Code(KeyCode::KeyW),
                binding_libretro::RETRO_DEVICE_ID_JOYPAD_UP,
            ),
            KeyboardKeymap::new(
                PhysicalKey::Code(KeyCode::KeyD),
                binding_libretro::RETRO_DEVICE_ID_JOYPAD_RIGHT,
            ),
            //buttons
            KeyboardKeymap::new(
                PhysicalKey::Code(KeyCode::KeyL),
                binding_libretro::RETRO_DEVICE_ID_JOYPAD_B,
            ),
            KeyboardKeymap::new(
                PhysicalKey::Code(KeyCode::KeyI),
                binding_libretro::RETRO_DEVICE_ID_JOYPAD_A,
            ),
            KeyboardKeymap::new(
                PhysicalKey::Code(KeyCode::KeyJ),
                binding_libretro::RETRO_DEVICE_ID_JOYPAD_X,
            ),
            KeyboardKeymap::new(
                PhysicalKey::Code(KeyCode::KeyK),
                binding_libretro::RETRO_DEVICE_ID_JOYPAD_Y,
            ),
            //Trigger
            KeyboardKeymap::new(
                PhysicalKey::Code(KeyCode::KeyQ),
                binding_libretro::RETRO_DEVICE_ID_JOYPAD_L,
            ),
            KeyboardKeymap::new(
                PhysicalKey::Code(KeyCode::KeyE),
                binding_libretro::RETRO_DEVICE_ID_JOYPAD_R,
            ),
            KeyboardKeymap::new(
                PhysicalKey::Code(KeyCode::KeyU),
                binding_libretro::RETRO_DEVICE_ID_JOYPAD_L2,
            ),
            KeyboardKeymap::new(
                PhysicalKey::Code(KeyCode::KeyO),
                binding_libretro::RETRO_DEVICE_ID_JOYPAD_R2,
            ),
            //Thumb
            KeyboardKeymap::new(
                PhysicalKey::Code(KeyCode::KeyC),
                binding_libretro::RETRO_DEVICE_ID_JOYPAD_L3,
            ),
            KeyboardKeymap::new(
                PhysicalKey::Code(KeyCode::KeyN),
                binding_libretro::RETRO_DEVICE_ID_JOYPAD_R3,
            ),
            //Menu
            KeyboardKeymap::new(
                PhysicalKey::Code(KeyCode::Enter),
                binding_libretro::RETRO_DEVICE_ID_JOYPAD_START,
            ),
            KeyboardKeymap::new(
                PhysicalKey::Code(KeyCode::Backspace),
                binding_libretro::RETRO_DEVICE_ID_JOYPAD_SELECT,
            ),
        ]
    }
}
