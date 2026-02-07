use crate::{FileProgress, tools::extract_files::{SevenZipBeforeExtractionAction, extract_7zip_file}};
use generics::retro_paths::RetroPaths;

fn remove_so_extension(name: String) -> String {
    name.replace(".so", "").replace(".dll", "")
}

pub async fn install_core(retro_paths: RetroPaths, core_file_name: Vec<String>) {
    let src_path = format!("{}/cores.7z", &retro_paths.temps);

    tokio::task::spawn_blocking(move || {
        extract_7zip_file(
            src_path.into(),
            retro_paths.cores.to_string(),
            |file_progress: FileProgress| match file_progress {
                FileProgress::Extract(name) => {
                    let name = remove_so_extension(name);
                    if core_file_name.contains(&name) {
                        return SevenZipBeforeExtractionAction::Extract;
                    }
                    SevenZipBeforeExtractionAction::Jump
                }
                FileProgress::Download(_, _) => SevenZipBeforeExtractionAction::Jump,
            },
        );
    });
}
