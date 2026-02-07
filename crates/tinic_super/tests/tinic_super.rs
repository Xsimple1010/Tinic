#[cfg(test)]
mod test {
    use generics::{
        retro_paths::RetroPaths,
        test_workdir::{create_test_work_dir_path, get_rom_test_path},
    };
    use std::{path::PathBuf, sync::Arc};
    use tinic_super::{
        GameIdentifier,
        art::helper::ThumbnailEventType,
        cores::CoreEventType,
        event::TinicSuperEventListener,
        infos::{helper::InfoEventType, model::CoreInfo},
        rdb_manager::helper::RdbEventType,
        tinic_super::TinicSuper,
    };

    struct TinicSuperListener;

    impl TinicSuperEventListener for TinicSuperListener {
        fn on_thumbnail_evnt(&self, event: ThumbnailEventType) {
            println!("on_thumbnail_evnt: {event:?}");
        }

        fn on_info_event(&self, event: InfoEventType) {
            println!("on_info_event: {event:?}");
        }

        fn on_core_event(&self, event: CoreEventType) {
            println!("on_core_event: {event:?}");
        }

        fn on_rdb_event(&self, event: RdbEventType) {
            println!("on_core_event: {event:?}");
        }
    }

    async fn setup(base_path: &str) -> (TinicSuper, PathBuf) {
        let work_dir = create_test_work_dir_path(base_path);
        let _ = tokio::fs::remove_dir_all(&work_dir).await;
        let retro_paths = RetroPaths::from_base(&work_dir.display()).unwrap();
        (
            TinicSuper::new(retro_paths, Arc::new(TinicSuperListener)),
            work_dir,
        )
    }

    async fn clean_up(work_dir: &PathBuf) {
        tokio::fs::remove_dir_all(work_dir).await.unwrap();
    }

    #[tokio::test]
    async fn info_helper() {
        let (tinic_super, work_dir) = setup("tinic_super..install_core").await;

        tinic_super
            .info_helper
            .download_blocking(false)
            .await
            .unwrap();

        // vai ser usado para o read file
        let info: Option<CoreInfo>;

        // get_infos
        {
            let infos = tinic_super.info_helper.get_infos().await;
            assert_eq!(infos.len(), 295);
            info = infos
                .into_iter()
                .find(|info| info.file_name == "snes9x_libretro");
        }

        // read_file
        {
            let info = info.as_ref().expect("core info not found");
            let path = info.info_path.clone();

            let new_info = tinic_super
                .info_helper
                .read_file(&path)
                .await
                .expect("info não foi encontrada verifique o caminho do arquivo");

            assert_eq!(new_info.file_name, info.file_name);
            assert_eq!(new_info.description, info.description);
            assert_eq!(new_info.database, info.database);
            assert_eq!(new_info.core_path, info.core_path);
            assert_eq!(new_info.core_path, info.core_path);
        }

        // get_compatibility_core_infos
        {
            let rom = PathBuf::from("./mario.smc");

            let infos = tinic_super
                .info_helper
                .get_compatibility_core_infos(&rom)
                .await;

            assert_eq!(infos.len(), 22);
        }

        clean_up(&work_dir).await;
    }

    #[tokio::test]
    async fn rdb_helper() {
        let (tinic_super, work_dir) = setup("tinic_super..rdb_helper").await;

        tinic_super.rdb_helper.download(false).await.unwrap();

        tinic_super.rdb_helper.read_rdbs().unwrap();

        clean_up(&work_dir).await;
    }

    #[tokio::test]
    async fn core_helper() {
        let (tinic_super, work_dir) = setup("tinic_super..core_helper").await;
        tinic_super
            .core_helper
            .download_blocking(false)
            .await
            .unwrap();

        let cores_file_created = PathBuf::from(&work_dir)
            .join("temps")
            .join("cores.7z")
            .exists();
        assert!(cores_file_created, "o arquivo 'cores.7z' não foi salvo!");

        tinic_super
            .core_helper
            .install_blocking(vec!["snes9x_libretro".to_string()])
            .await;

        let core_created = PathBuf::from(&work_dir)
            .join("cores")
            .join("snes9x_libretro.so")
            .exists();
        assert!(
            core_created,
            "o arquivo 'snes9x_libretro.so' não foi salvo!"
        );

        clean_up(&work_dir).await;
    }

    #[tokio::test]
    async fn art_helper() {
        let (tinic_super, work_dir) = setup("tinic_super..art_helper").await;

        let thumbnails = tinic_super
            .art_helper
            .get_urls(
                &"Nintendo - Super Nintendo Entertainment System",
                &"Batman Returns (USA)",
            )
            .await
            .unwrap();

        let (box_img, _box_img_url) = thumbnails.box_img;
        let (snap_img, _snap_img_url) = thumbnails.snap_img;
        let (title_img, _title_img_url) = thumbnails.title_img;

        assert!(box_img.is_some(), "box img não foi definida!");
        assert!(snap_img.is_some(), "snap img não foi definida!");
        assert!(title_img.is_some(), "title img não foi definida!");

        let box_path = box_img.unwrap();
        let snap_path = snap_img.unwrap();
        let title_path = title_img.unwrap();

        assert!(box_path.exists(), "box img não foi salvo!");
        assert!(snap_path.exists(), "snap img não foi salvo!");
        assert!(title_path.exists(), "title img não foi salvo!");

        clean_up(&work_dir).await;
    }

    #[tokio::test]
    async fn rom_identifier() {
        let (_tinic_super, work_dir) = setup("tinic_super..rom_identifier").await;

        let path = get_rom_test_path();

        let ident = GameIdentifier::new(path.clone()).await.unwrap();

        assert_eq!(ident.path, path);
        assert_eq!(ident.file_name, "240pTestSuite");
        assert_eq!(ident.crc, 3274800960);
        assert_eq!(ident.size, 65552);

        println!("{ident:?}");

        clean_up(&work_dir).await;
    }
}
