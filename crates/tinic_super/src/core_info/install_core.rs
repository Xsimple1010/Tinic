use crate::{
    FileProgress,
    event::TinicSuperEventListener,
    tools::extract_files::{SevenZipBeforeExtractionAction, extract_7zip_file},
};
use generics::retro_paths::RetroPaths;
use std::{collections::HashSet, sync::Arc};

fn remove_so_extension(name: String) -> String {
    name.replace(".so", "").replace(".dll", "")
}

pub async fn install_core(
    retro_paths: RetroPaths,
    core_file_name: Vec<String>,
    event_listener: Arc<dyn TinicSuperEventListener>,
) {
    let src_path = format!("{}/cores.7z", &retro_paths.temps);

    let mut wanted: HashSet<String> = core_file_name
        .into_iter()
        .map(remove_so_extension)
        .collect();

    tokio::task::spawn_blocking(move || {
        extract_7zip_file(
            src_path.into(),
            retro_paths.cores.to_string(),
            event_listener,
            |file_progress: FileProgress| match file_progress {
                FileProgress::Extract(name) => {
                    let name = remove_so_extension(name);

                    if wanted.remove(&name) {
                        return SevenZipBeforeExtractionAction::Extract;
                    }

                    if wanted.is_empty() {
                        SevenZipBeforeExtractionAction::Stop
                    } else {
                        SevenZipBeforeExtractionAction::Jump
                    }
                }
                FileProgress::Download(_, _) => SevenZipBeforeExtractionAction::Jump,
            },
        );
    });
}
