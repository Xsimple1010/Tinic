use serde::de::{MapAccess, Visitor};
use serde::{Deserialize, Deserializer, de};
use std::fmt;

#[derive(Debug, Default)]
pub struct GameInfo {
    pub name: Option<String>,
    pub description: Option<String>,
    pub genre: Option<String>,
    pub developer: Option<String>,
    pub publisher: Option<String>,
    pub franchise: Option<String>,
    pub origin: Option<String>,
    pub rom_name: Option<String>,
    pub release_year: Option<u32>,
    pub release_month: Option<u32>,
    pub size: Option<u64>,
    pub crc32: Option<u32>,
    pub serial: Option<String>,
    pub rumble: bool,
    pub rdb_name: String,
}

#[derive(Deserialize)]
#[serde(untagged)]
enum Crc32Repr {
    Int(u32),
    Bin(serde_bytes::ByteBuf),
}
//

#[derive(Deserialize)]
#[serde(untagged)]
enum SerialRepr {
    Str(String),
    Bin(serde_bytes::ByteBuf),
}

impl<'de> Deserialize<'de> for GameInfo {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct GameVisitor;

        impl<'de> Visitor<'de> for GameVisitor {
            type Value = GameInfo;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a RetroArch RDB game map")
            }

            fn visit_map<M>(self, mut map: M) -> Result<GameInfo, M::Error>
            where
                M: MapAccess<'de>,
            {
                let mut game = GameInfo::default();

                while let Some(key) = map.next_key::<String>()? {
                    match key.as_str() {
                        "name" => game.name = map.next_value()?,
                        "description" => game.description = map.next_value()?,
                        "genre" => game.genre = map.next_value()?,
                        "developer" => game.developer = map.next_value()?,
                        "publisher" => game.publisher = map.next_value()?,
                        "franchise" => game.franchise = map.next_value()?,
                        "origin" => game.origin = map.next_value()?,
                        "rom_name" => game.rom_name = map.next_value()?,
                        "releaseyear" => game.release_year = map.next_value()?,
                        "releasemonth" => game.release_month = map.next_value()?,
                        "size" => game.size = map.next_value()?,
                        "crc" => {
                            let crc: Crc32Repr = map.next_value()?;

                            match crc {
                                Crc32Repr::Int(v) => {
                                    game.crc32 = Some(v);
                                }
                                Crc32Repr::Bin(raw) => {
                                    if raw.len() == 4 {
                                        // game.crc32 = Some(u32::from_le_bytes([
                                        //     raw[0], raw[1], raw[2], raw[3],
                                        // ]));

                                        let crc =
                                            u32::from_le_bytes([raw[0], raw[1], raw[2], raw[3]]);
                                        game.crc32 = Some(crc.to_be());
                                    }
                                }
                            }
                        }
                        "serial" => {
                            let crc: SerialRepr = map.next_value()?;

                            match crc {
                                SerialRepr::Str(s) => {
                                    game.serial = Some(s);
                                }
                                SerialRepr::Bin(raw) => {
                                    game.serial = match String::from_utf8(raw.to_vec()) {
                                        Ok(s) => Some(s),
                                        Err(_) => None,
                                    };
                                }
                            }
                        }
                        // "rumble" => game.rumble = ,
                        _ => {
                            // ignora campos desconhecidos
                            let _: de::IgnoredAny = map.next_value()?;
                        }
                    }
                }

                Ok(game)
            }
        }

        deserializer.deserialize_map(GameVisitor)
    }
}
