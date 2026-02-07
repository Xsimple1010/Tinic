use crate::model::GameInfoInDb;

pub(crate) fn read_opt_string(stmt: &sqlite::Statement, idx: usize) -> Option<String> {
    stmt.read::<Option<String>, _>(idx).ok().flatten()
}

pub(crate) fn read_opt_u32(stmt: &sqlite::Statement, idx: usize) -> Option<u32> {
    stmt.read::<Option<i64>, _>(idx)
        .ok()
        .flatten()
        .map(|v| v as u32)
}

pub(crate) fn read_opt_u64(stmt: &sqlite::Statement, idx: usize) -> Option<u64> {
    stmt.read::<Option<i64>, _>(idx)
        .ok()
        .flatten()
        .map(|v| v as u64)
}

pub(crate) fn read_game_info(stmt: &sqlite::Statement) -> sqlite::Result<GameInfoInDb> {
    Ok(GameInfoInDb {
        name: read_opt_string(stmt, 1),
        description: read_opt_string(stmt, 2),
        genre: read_opt_string(stmt, 3),
        developer: read_opt_string(stmt, 4),
        publisher: read_opt_string(stmt, 5),
        franchise: read_opt_string(stmt, 6),
        origin: read_opt_string(stmt, 7),
        rom_name: read_opt_string(stmt, 8),
        serial: read_opt_string(stmt, 9),

        core_path: read_opt_string(stmt, 10),
        rom_path: read_opt_string(stmt, 11),
        console_name: read_opt_string(stmt, 12),

        release_year: read_opt_u32(stmt, 13),
        release_month: read_opt_u32(stmt, 14),

        size: read_opt_u64(stmt, 15),
        crc32: read_opt_u32(stmt, 16),

        rumble: stmt.read::<i64, _>(17)? != 0,
    })
}
