use generics::retro_paths::RetroPaths;
use std::sync::Arc;
use tinic_super::tinic_super::TinicSuper;
use tinic_super::FileProgress;

#[tokio::main]
async fn main() {
    let retro_paths =
        RetroPaths::from_base("/home/aderval/Downloads/RetroArch_cores".to_owned()).unwrap();

    let tinic_super = TinicSuper { retro_paths };

    let rom = "/home/aderval/Downloads/RetroArch_cores/FFVii.iso";
    let core_infos = { tinic_super.get_compatibility_core_infos(rom) };

    let on_progress = |progress| match progress {
        FileProgress::Download(file_name, percent) => {
            println!("{file_name}: {percent}%")
        }
        FileProgress::Extract(file_name) => println!("extracting: {file_name}"),
    };

    tinic_super
        .install_cores(&core_infos, false, Arc::new(on_progress))
        .await
        .unwrap();

    // let core_infos = core_infos
    //     .into_iter()
    //     .filter(|c| c.file_name.eq("ppsspp_libretro"))
    //     .collect();

    let game_info = tinic_super.identifier_rom_file(rom, &core_infos).unwrap();
    println!("{game_info:?}");
}
