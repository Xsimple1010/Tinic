mod data_test;
pub mod model;
pub mod query;
mod sqlite_query;
mod sqlite_query_tools;
pub mod tinic_database_connection;

#[cfg(test)]
mod tests {
    use crate::data_test;
    use crate::query::{
        create_game_table, delete_all_games, insert_game_infos, list_consoles,
        list_games_with_rom_path_paginated, select_by_crc32_list, update_game_paths,
    };
    use crate::tinic_database_connection::TinicDbConnection;
    use generics::error_handle::ErrorHandle;

    #[test]
    fn start_connection() -> Result<(), ErrorHandle> {
        let conn = TinicDbConnection::in_memory()?;
        create_game_table(&conn)?;
        insert_game_infos(&conn, &data_test::_get_data_test())?;

        let mut crcs = data_test::_get_data_test()
            .into_iter()
            .map(|g| g.crc32.unwrap())
            .collect::<Vec<_>>();
        crcs.remove(2);

        let games = select_by_crc32_list(&conn, &crcs)?;
        assert_eq!(games.len(), 2);

        let consoles = list_consoles(&conn)?;
        assert_eq!(consoles.len(), 2);

        let games = list_games_with_rom_path_paginated(&conn, 1, 1)?;
        assert_eq!(games.len(), 1);

        let core_path = "test/test";
        let rom_path = "test/test.so";
        let rom_name = "Final Fantasy VII (Disc 1).bin";

        let change_lines =
            update_game_paths(&conn, None, rom_name, Some(rom_path), Some(core_path))?;
        assert_eq!(change_lines, 1);

        let crc = 0x12345678;
        let change_lines =
            update_game_paths(&conn, Some(crc), rom_name, Some(rom_path), Some(core_path))?;
        assert_eq!(change_lines, 1);

        delete_all_games(&conn)?;
        Ok(())
    }
}
