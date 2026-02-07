use std::{fmt::Display, path::PathBuf};

pub fn create_test_work_dir_path(test_dir: impl Display) -> PathBuf {
    workspace_root().join(format!("test_workspace/{test_dir}"))
}

pub fn remove_test_work_dir_path(test_dir: impl Display) -> PathBuf {
    workspace_root().join(format!("test_workspace/{test_dir}"))
}

pub fn workspace_root() -> PathBuf {
    let mut dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));

    loop {
        if dir.join("Cargo.toml").exists() {
            // Verifica se é o Cargo.toml do WORKSPACE
            if let Ok(contents) = std::fs::read_to_string(dir.join("Cargo.toml")) {
                if contents.contains("[workspace]") {
                    return dir;
                }
            }
        }

        dir = dir
            .parent()
            .expect("Não foi possível encontrar a raiz do workspace")
            .to_path_buf();
    }
}

pub fn get_core_test_path() -> PathBuf {
    workspace_root().join("tests/assets/mesen_libretro.so")
}

pub fn get_rom_test_path() -> PathBuf {
    workspace_root().join("tests/assets/240pTestSuite.nes")
}
