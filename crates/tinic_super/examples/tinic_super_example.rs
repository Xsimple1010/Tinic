use generics::retro_paths::RetroPaths;
use tinic_super::{
    FileProgress, core_info::helper::CoreInfoHelper, database::helper::DatabaseHelper,
};

#[tokio::main]
async fn main() {
    let paths =
        RetroPaths::from_base("/home/aderval/Downloads/RetroArch_cores".to_owned()).unwrap();

    CoreInfoHelper::try_update_core_infos(&paths, false, |progress| match progress {
        FileProgress::Download(file, progress) => println!("Download {}: {:.2}%", file, progress),
        FileProgress::Extract(file) => println!("Extract {}", file),
    })
    .await
    .unwrap();

    DatabaseHelper::download_db(
        &paths,
        "Nintendo - GameCube|Nintendo - Wii|Nintendo - Wii (Digital)",
        false,
        |progress| match progress {
            FileProgress::Download(file, progress) => {
                println!("Download {}: {:.2}%", file, progress)
            }
            FileProgress::Extract(file) => println!("Extract {}", file),
        },
    )
    .await
    .unwrap();

    let rdbs = DatabaseHelper::get_instaled_rdb(&paths).unwrap();

    for rdb in rdbs {
        println!("{rdb:?}")
    }
}
