use crate::database::game::Game;
use generics::error_handle::ErrorHandle;
use rmp_serde::Deserializer;
use serde::Deserialize;
use std::io::Cursor;

pub fn parse_rdb<C>(rdb_file: &String, mut callback: C) -> Result<(), ErrorHandle>
where
    C: FnMut(Game) -> bool,
{
    let file = std::fs::read(rdb_file)?;
    let data = file.as_slice();

    let cursor = Cursor::new(&data[0x10..]);
    let mut de = Deserializer::new(cursor);

    loop {
        match Game::deserialize(&mut de) {
            Ok(game) => {
                if callback(game) {
                    break;
                }
            },
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

    Ok(())
}

pub fn parse_all_rdb_to_vec(rdb_dir: &String) -> Result<Vec<Game>, ErrorHandle> {
    let mut games = Vec::new();
    parse_rdb(rdb_dir, |game| {
        games.push(game);
        false
    })?;
    Ok(games)
}

pub fn debug_rdb(data: &[u8]) {
    let payload = &data[0x10..];

    let mut cursor = &payload[..];
    let v = rmpv::decode::read_value(&mut cursor).unwrap();

    println!("{:#?}", v);
}
