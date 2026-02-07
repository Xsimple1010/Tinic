use crate::event::TinicSuperEventListener;
use crate::rdb_manager::game_model::GameInfo;
use crate::rdb_manager::helper::RdbEventType;
use generics::constants::RDB_HEADER_SIZE;
use generics::error_handle::ErrorHandle;
use rayon::iter::{ParallelBridge, ParallelIterator};
use rmp_serde::Deserializer;
use serde::Deserialize;
use std::io::Cursor;
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

pub fn read_rdbs_from_dir(
    rdb_dir: &PathBuf,
    event: Arc<dyn TinicSuperEventListener>,
) -> Result<(), ErrorHandle> {
    let read_dir = std::fs::read_dir(&rdb_dir)?;
    let remaining = Arc::new(AtomicUsize::new(read_dir.count()));

    let read_dir = std::fs::read_dir(&rdb_dir)?;
    read_dir.par_bridge().for_each(|entry| {
        if let Ok(dir_entry) = entry {
            let _ = read_rdb_blocking(&dir_entry.path(), event.clone(), remaining.clone());
        }
    });

    Ok(())
}

fn get_file_name_from_path(rdb_path: &PathBuf) -> Result<String, ErrorHandle> {
    match rdb_path.file_prefix() {
        Some(name) => match name.to_str() {
            Some(name) => Ok(name.to_string()),
            None => return Err(ErrorHandle::new("message")),
        },
        None => return Err(ErrorHandle::new("message")),
    }
}

pub fn read_rdb_blocking(
    rdb_path: &PathBuf,
    event: Arc<dyn TinicSuperEventListener>,
    remaining: Arc<AtomicUsize>,
) -> Result<(), ErrorHandle> {
    let rdb_name = get_file_name_from_path(rdb_path)?;

    event.on_rdb_event(RdbEventType::StartRead {
        name: rdb_name.clone(),
    });

    let buffer = std::fs::read(&rdb_path)?;
    let data = buffer.as_slice();

    if data.len() < RDB_HEADER_SIZE {
        return Ok(());
    }

    let mut de = {
        let cursor = Cursor::new(&data[RDB_HEADER_SIZE..]);
        Deserializer::new(cursor)
    };

    let mut game_out: Vec<GameInfo> = Vec::new();

    loop {
        match GameInfo::deserialize(&mut de) {
            Ok(mut game) => {
                let rdb_name = match get_file_name_from_path(rdb_path) {
                    Ok(name) => name,
                    Err(_) => continue,
                };

                game.rdb_name = rdb_name.clone();
                game_out.push(game);

                if game_out.len() >= 50 {
                    event.on_rdb_event(RdbEventType::Reading {
                        game_infos: std::mem::take(&mut game_out),
                    });
                }
            }
            Err(rmp_serde::decode::Error::InvalidMarkerRead(_e))
            | Err(rmp_serde::decode::Error::InvalidDataRead(_e)) => {
                // println!("Invalid marker read: {}", e);
                break;
            }
            Err(rmp_serde::decode::Error::Syntax(_e)) => {
                // println!("Syntax error: {}", e);
                break;
            }

            Err(e) => return Err(ErrorHandle::new(&e.to_string())),
        }
    }

    remaining.fetch_sub(1, Ordering::SeqCst);

    event.on_rdb_event(RdbEventType::ReadCompleted {
        remaining: remaining.load(Ordering::SeqCst),
        name: rdb_name,
    });
    Ok(())
}
