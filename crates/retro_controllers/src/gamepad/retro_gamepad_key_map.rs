use crate::devices_manager::DeviceKeyMap;
use gilrs::Button;
use libretro_sys::binding_libretro;

#[derive(Debug, Clone, PartialEq)]
pub struct GamePadKeyMap {
    pub native: Button,
    pub retro: u32,
    pub pressed: bool,
}

impl GamePadKeyMap {
    pub fn new(native: Button, retro: u32) -> Self {
        Self {
            native,
            retro,
            pressed: true,
        }
    }
}

impl DeviceKeyMap<GamePadKeyMap, Button> for GamePadKeyMap {
    fn get_key_name_from_native_button<'a>(native: &Button) -> &'a str {
        match native {
            //DPads
            Button::DPadUp => "DPad-up",
            Button::DPadDown => "DPad-down",
            Button::DPadLeft => "DPad-left",
            Button::DPadRight => "DPad-right",

            //Buttons
            Button::South => "B",
            Button::East => "A",
            Button::North => "X",
            Button::West => "Y",

            //Trigger
            Button::LeftTrigger => "L",
            Button::RightTrigger => "R",
            Button::LeftTrigger2 => "L2",
            Button::RightTrigger2 => "R2",

            //Thumb
            Button::LeftThumb => "LeftThumb",
            Button::RightThumb => "RightThumb",

            Button::Start => "Start",
            Button::Select => "Select",
            Button::Mode => "mode",

            _ => "Chave desconhecida",
        }
    }

    fn get_default_key_maps() -> Vec<GamePadKeyMap> {
        vec![
            //DPads
            GamePadKeyMap::new(
                Button::DPadDown,
                binding_libretro::RETRO_DEVICE_ID_JOYPAD_DOWN,
            ),
            GamePadKeyMap::new(
                Button::DPadLeft,
                binding_libretro::RETRO_DEVICE_ID_JOYPAD_LEFT,
            ),
            GamePadKeyMap::new(Button::DPadUp, binding_libretro::RETRO_DEVICE_ID_JOYPAD_UP),
            GamePadKeyMap::new(
                Button::DPadRight,
                binding_libretro::RETRO_DEVICE_ID_JOYPAD_RIGHT,
            ),
            //buttons
            GamePadKeyMap::new(Button::South, binding_libretro::RETRO_DEVICE_ID_JOYPAD_B),
            GamePadKeyMap::new(Button::East, binding_libretro::RETRO_DEVICE_ID_JOYPAD_A),
            GamePadKeyMap::new(Button::North, binding_libretro::RETRO_DEVICE_ID_JOYPAD_X),
            GamePadKeyMap::new(Button::West, binding_libretro::RETRO_DEVICE_ID_JOYPAD_Y),
            //Trigger
            GamePadKeyMap::new(
                Button::LeftTrigger,
                binding_libretro::RETRO_DEVICE_ID_JOYPAD_L,
            ),
            GamePadKeyMap::new(
                Button::RightTrigger,
                binding_libretro::RETRO_DEVICE_ID_JOYPAD_R,
            ),
            GamePadKeyMap::new(
                Button::LeftTrigger2,
                binding_libretro::RETRO_DEVICE_ID_JOYPAD_L2,
            ),
            GamePadKeyMap::new(
                Button::RightTrigger2,
                binding_libretro::RETRO_DEVICE_ID_JOYPAD_R2,
            ),
            //Thumb
            GamePadKeyMap::new(
                Button::LeftThumb,
                binding_libretro::RETRO_DEVICE_ID_JOYPAD_L3,
            ),
            GamePadKeyMap::new(
                Button::RightThumb,
                binding_libretro::RETRO_DEVICE_ID_JOYPAD_R3,
            ),
            //Menu
            GamePadKeyMap::new(
                Button::Start,
                binding_libretro::RETRO_DEVICE_ID_JOYPAD_START,
            ),
            GamePadKeyMap::new(
                Button::Select,
                binding_libretro::RETRO_DEVICE_ID_JOYPAD_SELECT,
            ),
        ]
    }
}
