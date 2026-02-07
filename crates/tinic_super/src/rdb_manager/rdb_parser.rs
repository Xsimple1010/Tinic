use crate::core_info::model::CoreInfo;
use crate::event::TinicSuperEventListener;
use crate::rdb_manager::game_model::GameInfo;
use futures_util::stream::FuturesUnordered;
use futures_util::StreamExt;
use generics::constants::RDB_HEADER_SIZE;
use generics::error_handle::ErrorHandle;
use rmp_serde::Deserializer;
use serde::Deserialize;
use std::collections::HashSet;
use std::io::Cursor;
use std::sync::Arc;

pub fn read_rdb_blocking(
    rdb_path: &str,
    event: Arc<dyn TinicSuperEventListener>,
) -> Result<(), ErrorHandle> {
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
                    event.rdb_read(std::mem::take(&mut game_out));
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

pub async fn read_rdb(rdb_path: String, event: Arc<dyn TinicSuperEventListener>) {
    tokio::task::spawn_blocking(move || read_rdb_blocking(&rdb_path, event));
}

pub async fn read_rdb_from_cores(
    core_infos: Vec<CoreInfo>,
    rdb_dir: String,
    event: Arc<dyn TinicSuperEventListener>,
) {
    let mut tasks = FuturesUnordered::new();

    let rdb_names: HashSet<String> = core_infos
        .into_iter()
        .map(|c| c.database)
        .flat_map(|d| d)
        .map(|rdb_name| format!("{}/{rdb_name}.rdb", rdb_dir))
        .collect();

    for rdb in rdb_names {
        let event = event.clone();

        tasks.push(async move {
            read_rdb(rdb, event).await;
        });
    }

    while let Some(_) = tasks.next().await {}
}

pub fn debug_rdb(data: &[u8]) {
    let payload = &data[0x10..];

    let mut cursor = &payload[..];
    let v = rmpv::decode::read_value(&mut cursor).unwrap();
    println!("{:#?}", v);
}
