use generics::retro_paths::RetroPaths;
use tinic_super::tinic_super::TinicSuper;
use tinic_super::FileProgress;

#[tokio::main]
async fn main() {
    let retro_paths =
        RetroPaths::from_base("/home/aderval/Downloads/RetroArch_cores".to_owned()).unwrap();

    let tinic_super = TinicSuper { retro_paths };

    let rom = "/home/aderval/Downloads/RetroArch_cores/Super Mario World (USA).sfc";
    let core_infos = { tinic_super.get_compatibility_core_infos(rom) };

    for core in &core_infos {
        tinic_super
            .install_core(&core, false, |progress| match progress {
                FileProgress::Download(file_name, percent) => {
                    println!("{file_name}: {percent}%")
                }
                FileProgress::Extract(file_name) => println!("extracting: {file_name}"),
            })
            .await
            .unwrap();
    }

    let game_info = tinic_super.identifier_rom_file(rom, &core_infos).unwrap();
    println!("{game_info:?}");
}
