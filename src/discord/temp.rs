fn create_directory_structure(top_dir: &Path) {
    if let Ok(entries) = read_dir(top_dir) {
        for entry in entries.flatten() {
            if entry.path().is_dir() {
                let entry_path = clean_absolute_path(entry.path().to_str().unwrap());
                let mut temp_write_path = PathBuf::from(DIRECTORY);
                temp_write_path.push(&entry_path);
                let _ = create_dir_all(&temp_write_path);
                create_directory_structure(&entry.path());
            }
        }
    } else {
        println!("Failed to read directory {:?}", top_dir);
    }
}

fn create_construction_directory_structure(top_dir: &Path) {
    if let Ok(entries) = read_dir(top_dir) {
        for entry in entries.flatten() {
            if entry.path().is_dir() {
                let entry_path = clean_absolute_path(entry.path().to_str().unwrap());
                let mut temp_write_path = PathBuf::from(CONSTRUCTION_DIR);
                temp_write_path.push(&entry_path);
                let _ = create_dir_all(&temp_write_path);
                create_construction_directory_structure(&entry.path());
            }
        }
    } else {
        println!("Failed to read directory {:?}", top_dir);
    }
}