use crate::rdb_manager::game::GameInfo;
use generics::constants::RDB_HEADER_SIZE;
use generics::error_handle::ErrorHandle;
use rmp_serde::Deserializer;
use serde::Deserialize;
use std::io::Cursor;

pub fn read_to_end_of_rdb<C>(rdb_path: &str, mut callback: C) -> Result<(), ErrorHandle>
where
    C: FnMut(Vec<GameInfo>),
{
    let file = std::fs::read(rdb_path)?;
    let data = file.as_slice();

    if data.len() < RDB_HEADER_SIZE {
        return Ok(());
    }

    let cursor = Cursor::new(&data[RDB_HEADER_SIZE..]);
    let mut de = Deserializer::new(cursor);

    let mut game_out: Vec<GameInfo> = Vec::new();

    loop {
        match GameInfo::deserialize(&mut de) {
            Ok(game) => {
                game_out.push(game);

                if game_out.len() >= 50 {
                    callback(std::mem::take(&mut game_out));
                }
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

    Ok(())
}

pub fn debug_rdb(data: &[u8]) {
    let payload = &data[0x10..];

    let mut cursor = &payload[..];
    let v = rmpv::decode::read_value(&mut cursor).unwrap();
    println!("{:#?}", v);
}
