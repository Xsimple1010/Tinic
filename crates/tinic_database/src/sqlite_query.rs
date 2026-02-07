pub fn get_create_game_table_query() -> &'static str {
    "
    CREATE TABLE IF NOT EXISTS game_info (
        id INTEGER PRIMARY KEY AUTOINCREMENT,

        name TEXT,
        description TEXT,
        genre TEXT,
        developer TEXT,
        publisher TEXT,
        franchise TEXT,
        origin TEXT,
        rom_name TEXT,
        serial TEXT,

        core_path TEXT,
        rom_path TEXT,
        console_name TEXT,

        release_year INTEGER,
        release_month INTEGER,

        size INTEGER,
        crc32 INTEGER,

        rumble INTEGER NOT NULL DEFAULT 0
    );

    CREATE INDEX IF NOT EXISTS idx_game_crc32 ON game_info (crc32);
    CREATE INDEX IF NOT EXISTS idx_game_size ON game_info (size);
    CREATE INDEX IF NOT EXISTS idx_game_name ON game_info (name);
    CREATE INDEX IF NOT EXISTS idx_game_serial ON game_info (serial);
    CREATE INDEX IF NOT EXISTS idx_game_console_name ON game_info (console_name);

    CREATE UNIQUE INDEX IF NOT EXISTS ux_game_crc_console ON game_info (crc32, console_name);
    "
}

pub(crate) fn get_insert_game_query() -> &'static str {
    "
        INSERT INTO game_info (
            name,
            description,
            genre,
            developer,
            publisher,
            franchise,
            origin,
            rom_name,
            serial,
            core_path,
            rom_path,
            console_name,
            release_year,
            release_month,
            size,
            crc32,
            rumble
        ) VALUES (
            ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?
        );
        "
}

pub(crate) fn delete_all_games_query() -> &'static str {
    "DELETE FROM game_info;"
}

pub(crate) fn get_select_console_names_query() -> &'static str {
    "
        SELECT DISTINCT console_name
        FROM game_info
        WHERE console_name IS NOT NULL
        ORDER BY console_name;
    "
}

pub(crate) fn get_game_pagination_query() -> &'static str {
    "
        SELECT
        id,
        name,
        rom_path,
        console_name
        FROM game_info WHERE rom_path IS NOT NULL ORDER BY id LIMIT ? OFFSET ?
    "
}
