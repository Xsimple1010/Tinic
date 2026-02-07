use pc_keyboard::{KeyboardLayout, ScancodeSet1};
use uuid::Uuid;

struct RetroKeyboard {
    pub id: Uuid,
    pub name: String,
    #[doc = "indicar ao Core em qual porta o teclado esta conectado, se o valor for -1 significa que todas as porta suportas pelo Core ja estão sendo usadas"]
    pub retro_port: i16,
    #[doc = "padrão RETRO_DEVICE_JOYPAD"]
    pub retro_type: u32,
}

impl RetroKeyboard {
    pub fn new() {}

    pub fn update() {}

    pub fn key_pressed() {}
}
