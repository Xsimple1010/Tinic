use std::{fs::File, io::copy, path::PathBuf};

pub fn extract_zip_file(file_path: PathBuf, out_dir: String) {
    let file = File::open(file_path).unwrap();

    let mut archive = zip::ZipArchive::new(file).unwrap();

    for i in 0..archive.len() {
        let mut file = archive.by_index(i).unwrap();

        if !file.is_dir() {
            let mut out_path = PathBuf::from(&out_dir);
            out_path.push(file.name());

            let mut outfile = File::create(&out_path).unwrap();
            copy(&mut file, &mut outfile).unwrap();
        }
    }
}

pub fn extract_7zip_file(file: PathBuf, out_dir: String) {
    sevenz_rust::decompress_file(file, out_dir).expect("complete");
}
