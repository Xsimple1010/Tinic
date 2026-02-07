use gilrs::Button;
use libretro_sys::binding_libretro;

#[derive(Debug, Clone, PartialEq)]
pub struct GamepadKeyMap {
    pub native: Button,
    pub retro: u32,
    pub pressed: bool,
}

impl GamepadKeyMap {
    pub fn new(native: Button, retro: u32) -> Self {
        Self {
            native,
            retro,
            pressed: false,
        }
    }
    pub fn get_default_key_maps() -> Vec<GamepadKeyMap> {
        vec![
            //DPads
            GamepadKeyMap::new(
                Button::DPadDown,
                binding_libretro::RETRO_DEVICE_ID_JOYPAD_DOWN,
            ),
            GamepadKeyMap::new(
                Button::DPadLeft,
                binding_libretro::RETRO_DEVICE_ID_JOYPAD_LEFT,
            ),
            GamepadKeyMap::new(Button::DPadUp, binding_libretro::RETRO_DEVICE_ID_JOYPAD_UP),
            GamepadKeyMap::new(
                Button::DPadRight,
                binding_libretro::RETRO_DEVICE_ID_JOYPAD_RIGHT,
            ),
            //buttons
            GamepadKeyMap::new(Button::South, binding_libretro::RETRO_DEVICE_ID_JOYPAD_B),
            GamepadKeyMap::new(Button::East, binding_libretro::RETRO_DEVICE_ID_JOYPAD_A),
            GamepadKeyMap::new(Button::North, binding_libretro::RETRO_DEVICE_ID_JOYPAD_X),
            GamepadKeyMap::new(Button::West, binding_libretro::RETRO_DEVICE_ID_JOYPAD_Y),
            //Trigger
            GamepadKeyMap::new(
                Button::LeftTrigger,
                binding_libretro::RETRO_DEVICE_ID_JOYPAD_L,
            ),
            GamepadKeyMap::new(
                Button::RightTrigger,
                binding_libretro::RETRO_DEVICE_ID_JOYPAD_R,
            ),
            GamepadKeyMap::new(
                Button::LeftTrigger2,
                binding_libretro::RETRO_DEVICE_ID_JOYPAD_L2,
            ),
            GamepadKeyMap::new(
                Button::RightTrigger2,
                binding_libretro::RETRO_DEVICE_ID_JOYPAD_R2,
            ),
            //Thumb
            GamepadKeyMap::new(
                Button::LeftThumb,
                binding_libretro::RETRO_DEVICE_ID_JOYPAD_L3,
            ),
            GamepadKeyMap::new(
                Button::RightThumb,
                binding_libretro::RETRO_DEVICE_ID_JOYPAD_R3,
            ),
            //Menu
            GamepadKeyMap::new(
                Button::Start,
                binding_libretro::RETRO_DEVICE_ID_JOYPAD_START,
            ),
            GamepadKeyMap::new(
                Button::Select,
                binding_libretro::RETRO_DEVICE_ID_JOYPAD_SELECT,
            ),
        ]
    }

    pub fn get_key_name_from_retro_button<'a>(retro: u32) -> &'a str {
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

    pub fn get_key_name_from_native_button<'a>(native: &Button) -> &'a str {
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
}
