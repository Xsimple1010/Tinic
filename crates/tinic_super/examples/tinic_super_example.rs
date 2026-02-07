use generics::retro_paths::RetroPaths;
use std::ops::Not;
use std::sync::Arc;
use tinic_super::event::TinicSuperEventListener;
use tinic_super::tinic_super::TinicSuper;

struct TinicSuperListener;

impl TinicSuperEventListener for TinicSuperListener {
    fn downloading(&self, file_name: String, percent: f32) {
        println!("{file_name}: {percent}%")
    }

    fn extract_file(&self, file_name: String) {
        println!("extracting: {file_name}")
    }

    fn download_completed(&self, file_name: String) {
        println!("{file_name} downloaded")
    }
}

#[tokio::main]
async fn main() {
    let retro_paths =
        RetroPaths::from_base("/home/aderval/Downloads/RetroArch_cores".to_owned()).unwrap();

    let tinic_super = TinicSuper {
        retro_paths,
        event_listener: Arc::new(TinicSuperListener),
    };

    if tinic_super.has_core_installed().not() {
        tinic_super.try_update_core(true).await.unwrap();
    }

    let rom = "/home/aderval/Downloads/RetroArch_cores/Super Mario World (USA).sfc";
    let core_infos = { tinic_super.get_compatibility_core_infos(rom) };

    // tinic_super
    //     .install_cores_and_rdb(&core_infos, false)
    //     .await
    //     .unwrap();

    for core in core_infos {
        tinic_super
            .read_rdb_to_end(&core.database, |cores| {
                println!("{cores:?}");
                println!("=====")
            })
            .unwrap();
    }

    tinic_super
        .download_all_thumbnail_from_game(
            "Nintendo - Super Nintendo Entertainment System",
            "Super Mario World (USA)",
        )
        .await;
}
