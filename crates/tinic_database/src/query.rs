use crate::model::{opt_bool, opt_str, opt_u32, opt_u64, GameInfoInDb, GameInfoPagination};
use crate::sqlite_query::{
    delete_all_games_query, get_create_game_table_query, get_game_pagination_query,
    get_insert_game_query, get_select_console_names_query,
};
use crate::sqlite_query_tools::read_game_info;
use crate::tinic_database_connection::TinicDbConnection;
use generics::error_handle::ErrorHandle;
use sqlite::Value;

pub fn create_game_table(connection: &TinicDbConnection) -> Result<(), ErrorHandle> {
    connection.try_execute(get_create_game_table_query())
}

pub fn insert_game_infos(
    conn: &TinicDbConnection,
    games: &[GameInfoInDb],
) -> Result<(), ErrorHandle> {
    if games.is_empty() {
        return Ok(());
    }

    conn.with_statement(get_insert_game_query(), |stmt, conn| {
        conn.execute("BEGIN TRANSACTION;")?;

        for game in games {
            stmt.bind((1, opt_str(&game.name)))?;
            stmt.bind((2, opt_str(&game.description)))?;
            stmt.bind((3, opt_str(&game.genre)))?;
            stmt.bind((4, opt_str(&game.developer)))?;
            stmt.bind((5, opt_str(&game.publisher)))?;
            stmt.bind((6, opt_str(&game.franchise)))?;
            stmt.bind((7, opt_str(&game.origin)))?;
            stmt.bind((8, opt_str(&game.rom_name)))?;
            stmt.bind((9, opt_str(&game.serial)))?;

            stmt.bind((10, opt_str(&game.core_path)))?;
            stmt.bind((11, opt_str(&game.rom_path)))?;
            stmt.bind((12, opt_str(&game.rdb_name)))?;

            stmt.bind((13, opt_u32(game.release_year)))?;
            stmt.bind((14, opt_u32(game.release_month)))?;

            stmt.bind((15, opt_u64(game.size)))?;
            stmt.bind((16, opt_u32(game.crc32)))?;

            stmt.bind((17, opt_bool(game.rumble)))?;

            stmt.next()?; // executa o INSERT
            stmt.reset()?; // LIMPA os binds para o próximo loop
        }

        Ok(())
    })?;

    conn.execute("COMMIT;")?;
    Ok(())
}

pub fn delete_all_games(conn: &TinicDbConnection) -> Result<(), ErrorHandle> {
    conn.execute(delete_all_games_query())
}

pub fn select_by_crc32_list(
    conn: &TinicDbConnection,
    crc_list: &[u32],
) -> Result<Vec<GameInfoInDb>, ErrorHandle> {
    if crc_list.is_empty() {
        return Ok(Vec::new());
    }

    // gera "?, ?, ?, ?"
    let placeholders = std::iter::repeat("?")
        .take(crc_list.len())
        .collect::<Vec<_>>()
        .join(",");

    let sql = format!("SELECT * FROM game_info WHERE crc32 IN ({})", placeholders);

    let stmt = conn.with_statement(&sql, |stmt, _conn| {
        // bind dos crc32
        for (i, crc) in crc_list.iter().enumerate() {
            stmt.bind((i + 1, Value::Integer(*crc as i64)))?;
        }

        let mut results = Vec::new();

        while let sqlite::State::Row = stmt.next()? {
            results.push(read_game_info(&stmt)?);
        }

        Ok(results)
    })?;

    Ok(stmt)
}

pub fn list_consoles(conn: &TinicDbConnection) -> Result<Vec<String>, ErrorHandle> {
    conn.with_statement(get_select_console_names_query(), |stmt, _conn| {
        let mut consoles = Vec::new();

        while let sqlite::State::Row = stmt.next()? {
            let console: String = stmt.read(0)?;
            consoles.push(console);
        }

        Ok(consoles)
    })
}

pub fn list_games_with_rom_path_paginated(
    db: &TinicDbConnection,
    page: u32,
    page_size: u32,
) -> Result<Vec<GameInfoPagination>, ErrorHandle> {
    let offset = (page.saturating_sub(1) * page_size) as i64;
    let limit = page_size as i64;

    db.with_statement(get_game_pagination_query(), |stmt, _conn| {
        stmt.bind((1, limit))?;
        stmt.bind((2, offset))?;

        let mut games = Vec::new();

        while let sqlite::State::Row = stmt.next()? {
            games.push(GameInfoPagination {
                id: stmt.read(0)?,
                name: stmt.read(1)?,
                rom_path: stmt.read(2)?,
                console_name: stmt.read(3)?,
            });
        }

        Ok(games)
    })
}

pub fn update_game_paths(
    db: &TinicDbConnection,
    crc32: Option<u32>,
    rom_name: &str,
    rom_path: Option<&str>,
    core_path: Option<&str>,
) -> Result<usize, ErrorHandle> {
    // 1️⃣ Tenta atualizar pelo CRC32 (se existir)
    if let Some(crc) = crc32 {
        let updated = db.with_statement(
            "
            UPDATE game_info
            SET
                rom_path  = ?,
                core_path = ?
            WHERE crc32 = ?
            ",
            |stmt, conn| {
                stmt.bind((1, rom_path))?;
                stmt.bind((2, core_path))?;
                stmt.bind((3, opt_u32(Some(crc))))?;

                stmt.next()?;
                Ok(conn.change_count())
            },
        )?;

        if updated > 0 {
            return Ok(updated);
        }
    }

    // 2️⃣ Fallback: atualiza pelo nome da ROM
    db.with_statement(
        "
        UPDATE game_info
        SET
            rom_path  = ?,
            core_path = ?
        WHERE rom_name = ?
        ",
        |stmt, conn| {
            stmt.bind((1, rom_path))?;
            stmt.bind((2, core_path))?;
            stmt.bind((3, rom_name))?;

            stmt.next()?;
            Ok(conn.change_count())
        },
    )
}
