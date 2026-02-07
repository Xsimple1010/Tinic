use crate::model::GameInfoInDb;

pub(crate) fn _get_data_test() -> Vec<GameInfoInDb> {
    vec![
        GameInfoInDb {
            name: Some("Super Mario World".into()),
            genre: Some("Platform".into()),
            developer: Some("Nintendo".into()),
            publisher: Some("Nintendo".into()),
            franchise: Some("Super Mario".into()),
            rom_name: Some("Super Mario World (USA).smc".into()),
            serial: Some("SNS-MW-USA".into()),
            core_path: Some("/cores/snes9x_libretro.so".into()),
            rom_path: Some("/roms/snes/super_mario_world.smc".into()),
            console_name: Some("SNES".into()),
            release_year: Some(1990),
            size: Some(524_288),
            crc32: Some(0xA1B2C3D4),
            rumble: false,
            ..Default::default()
        },
        GameInfoInDb {
            name: Some("The Legend of Zelda: A Link to the Past".into()),
            genre: Some("Action RPG".into()),
            developer: Some("Nintendo".into()),
            publisher: Some("Nintendo".into()),
            franchise: Some("Zelda".into()),
            rom_name: Some("Zelda - A Link to the Past.smc".into()),
            serial: Some("SNS-ZLTP".into()),
            core_path: Some("/cores/snes9x_libretro.so".into()),
            rom_path: Some("/roms/snes/zelda_alttp.smc".into()),
            console_name: Some("SNES".into()),
            release_year: Some(1991),
            size: Some(1_048_576),
            crc32: Some(0xDEADBEEF),
            rumble: false,
            ..Default::default()
        },
        GameInfoInDb {
            name: Some("Final Fantasy VII".into()),
            genre: Some("JRPG".into()),
            developer: Some("Square".into()),
            publisher: Some("Sony".into()),
            franchise: Some("Final Fantasy".into()),
            rom_name: Some("Final Fantasy VII (Disc 1).bin".into()),
            serial: Some("SCUS-94163".into()),
            core_path: Some("/cores/pcsx_rearmed_libretro.so".into()),
            rom_path: Some("/roms/ps1/ff7_disc1.bin".into()),
            console_name: Some("PS1".into()),
            release_year: Some(1997),
            size: Some(734_003_200),
            crc32: Some(0x12345678),
            rumble: true,
            ..Default::default()
        },
    ]
}
