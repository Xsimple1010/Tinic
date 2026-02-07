use crate::database::game::Game;
use generics::error_handle::ErrorHandle;
use rmp_serde::Deserializer;
use serde::Deserialize;
use std::io::Cursor;

pub fn parse_rdb(rdb_file: String) -> Result<Vec<Game>, ErrorHandle> {
    let file = std::fs::read(rdb_file).unwrap();
    let data = file.as_slice();

    let cursor = Cursor::new(&data[0x10..]);
    let mut de = Deserializer::new(cursor);
    let mut games = Vec::new();

    loop {
        match Game::deserialize(&mut de) {
            Ok(game) => {
                games.push(game);
            }
            Err(rmp_serde::decode::Error::InvalidMarkerRead(e))
            | Err(rmp_serde::decode::Error::InvalidDataRead(e)) => {
                println!("Invalid marker read: {}", e);
                break;
            }
            Err(rmp_serde::decode::Error::Syntax(e)) => {
                println!("Syntax error: {}", e);
                break;
            }

            Err(e) => return Err(ErrorHandle::new(&e.to_string())),
        }
    }

    Ok(games)
}

pub fn debug_rdb(data: &[u8]) {
    let payload = &data[0x10..];

    let mut cursor = &payload[..];
    let v = rmpv::decode::read_value(&mut cursor).unwrap();

    println!("{:#?}", v);
}
