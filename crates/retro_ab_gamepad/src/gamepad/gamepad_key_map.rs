use gilrs::Button;
use retro_ab::retro_sys;

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
            GamepadKeyMap::new(Button::DPadDown, retro_sys::RETRO_DEVICE_ID_JOYPAD_DOWN),
            GamepadKeyMap::new(Button::DPadLeft, retro_sys::RETRO_DEVICE_ID_JOYPAD_LEFT),
            GamepadKeyMap::new(Button::DPadUp, retro_sys::RETRO_DEVICE_ID_JOYPAD_UP),
            GamepadKeyMap::new(Button::DPadRight, retro_sys::RETRO_DEVICE_ID_JOYPAD_RIGHT),

            //buttons
            GamepadKeyMap::new(Button::South, retro_sys::RETRO_DEVICE_ID_JOYPAD_B),
            GamepadKeyMap::new(Button::East, retro_sys::RETRO_DEVICE_ID_JOYPAD_A),
            GamepadKeyMap::new(Button::North, retro_sys::RETRO_DEVICE_ID_JOYPAD_X),
            GamepadKeyMap::new(Button::West, retro_sys::RETRO_DEVICE_ID_JOYPAD_Y),

            //Trigger
            GamepadKeyMap::new(Button::LeftTrigger, retro_sys::RETRO_DEVICE_ID_JOYPAD_L),
            GamepadKeyMap::new(Button::RightTrigger, retro_sys::RETRO_DEVICE_ID_JOYPAD_R),
            GamepadKeyMap::new(Button::LeftTrigger2, retro_sys::RETRO_DEVICE_ID_JOYPAD_L2),
            GamepadKeyMap::new(Button::RightTrigger2, retro_sys::RETRO_DEVICE_ID_JOYPAD_R2),

            //Thumb
            GamepadKeyMap::new(Button::LeftThumb, retro_sys::RETRO_DEVICE_ID_JOYPAD_L3),
            GamepadKeyMap::new(Button::RightThumb, retro_sys::RETRO_DEVICE_ID_JOYPAD_R3),

            //Menu
            GamepadKeyMap::new(Button::Start, retro_sys::RETRO_DEVICE_ID_JOYPAD_START),
            GamepadKeyMap::new(Button::Select, retro_sys::RETRO_DEVICE_ID_JOYPAD_SELECT),
        ]
    }

    pub fn get_key_name_from_retro_button<'a>(retro: u32) -> &'a str {
        match retro {
            //DPads
            retro_sys::RETRO_DEVICE_ID_JOYPAD_DOWN => "Retro DPad-down",
            retro_sys::RETRO_DEVICE_ID_JOYPAD_UP => "Retro DPad-up",
            retro_sys::RETRO_DEVICE_ID_JOYPAD_LEFT => "Retro DPad-left",
            retro_sys::RETRO_DEVICE_ID_JOYPAD_RIGHT => "Retro DPad-right",

            //buttons
            retro_sys::RETRO_DEVICE_ID_JOYPAD_B => "Retro B",
            retro_sys::RETRO_DEVICE_ID_JOYPAD_A => "Retro A",
            retro_sys::RETRO_DEVICE_ID_JOYPAD_X => "Retro X",
            retro_sys::RETRO_DEVICE_ID_JOYPAD_Y => "Retro Y",

            //Trigger
            retro_sys::RETRO_DEVICE_ID_JOYPAD_L => "Retro L",
            retro_sys::RETRO_DEVICE_ID_JOYPAD_R => "Retro R",
            retro_sys::RETRO_DEVICE_ID_JOYPAD_L2 => "Retro L2",
            retro_sys::RETRO_DEVICE_ID_JOYPAD_R2 => "Retro R2",

            //Thumb
            retro_sys::RETRO_DEVICE_ID_JOYPAD_L3 => "Retro L3",
            retro_sys::RETRO_DEVICE_ID_JOYPAD_R3 => "Retro R3",


            //Menu
            retro_sys::RETRO_DEVICE_ID_JOYPAD_START => "Retro Start",
            retro_sys::RETRO_DEVICE_ID_JOYPAD_SELECT => "Retro Select",
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
