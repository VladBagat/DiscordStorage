use std::fs::{create_dir_all, read_dir, File};
use std::io::{Read, Write, Result, SeekFrom, Seek};
use std::path::{PathBuf, Path};
use regex::Regex;


const DIRECTORY: &str = "output";
const CONSTRUCTION_DIR: &str = "D:/ConstructorResult";


fn main() -> Result<()> {
    create_dir_all(DIRECTORY)?; 
    create_dir_all(CONSTRUCTION_DIR)?;
    /*let path: &str = "test.mp4";
    deconstruct_file(path)?;
    reconstruct_file(path)?;*/
    let root = PathBuf::from(r"D:\SteamLibrary\steamapps\common\No Man's Sky");
    trace_path_deconstruct(root);
    println!("deconstruction successful");

    let construct_root = PathBuf::from(DIRECTORY);
    trace_path_construct(construct_root);

    Ok(())
}

fn deconstruct_file(file_path: &str) -> Result<()> {
    let mut file_index: u32 = 0;
    let read_path = file_path;
    
    loop {
        let write_path_base = clean_absolute_path(file_path);
        let write_path = format!("{}/{}_{}.bin", DIRECTORY, write_path_base, &file_index);
        let (buffer, read_bytes) = read_chunk(&file_index, &read_path)?;
        
        if read_bytes == 0 {
            break;
        }

        write_file(&write_path, buffer)?;
        file_index += 1;

        if read_bytes < 10 * 1024 * 1024 {
            break;
        }
    }

    Ok(())
}

fn reconstruct_file(file_path: &str) -> Result<()> {

    let mut write_result: Vec<u8> = vec![];
    let mut file_index: u32 = 0;
    let file_path = remove_deconstruction_artifact(file_path);
    let write_path: String = format!("{}/{}", CONSTRUCTION_DIR, file_path); 
    //println!("Write path is: {}", write_path);

    loop{
        
        let read_path = format!(r"{}_{}.bin", file_path, &file_index); 
        //println!("Read path is: {}", &read_path);

        match read_chunk(&0, &read_path) {
            Ok((mut buffer, read_bytes)) => {
                write_result.append(&mut buffer);
                file_index += 1;
                if read_bytes < 10 * 1024 * 1024 {
                    break;
                }
            },
            Err(e) => {println!("Construction loop exited with error: {}", e); break;},
        }
    }
    write_file(&write_path, write_result)?;
    
    Ok(())
}

fn read_chunk(file_index:&u32, path: &str) -> Result<(Vec<u8>, usize)> {
    let mut f: File = File::open(path)?;

    f.seek(SeekFrom::Start((file_index * (10 * 1024 * 1024)).into()))?;

    // Allocate 10 MB on the heap
    let mut buffer = vec![0u8; 10 * 1024 * 1024];

    let n = f.read(&mut buffer[..])?;
    //println!("Read {} KB", &n/1000);
    
    Ok((buffer[..n].to_vec(), n))
}

fn write_file(file_path:&str, buffer: Vec<u8>) -> Result<()> {
    //TODO: change file_path &str to PathBuf
    //println!("Write path is: {}", file_path);
    let mut file = File::create(format!("{}", file_path))?; 
    file.write(&buffer)?; 

    Ok(())
}

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

fn process_files(top_dir: &Path) {
    if let Ok(entries) = read_dir(top_dir) {
        for entry in entries.flatten() {
            if entry.path().is_file() {
                if let Err(e) = deconstruct_file(entry.path().to_str().unwrap()) {
                    println!("Error deconstructing file {:?}: {:?}", entry.path(), e);
                }
            } else if entry.path().is_dir() {
                process_files(&entry.path());
            }
        }
    } else {
        println!("Failed to read directory {:?}", top_dir);
    }
}

fn trace_path_deconstruct(top_dir: PathBuf) {
    create_directory_structure(&top_dir);
    process_files(&top_dir);
}


fn trace_path_construct(top_dir:PathBuf) {
    let mut dir_path = PathBuf::new();
    dir_path.push(top_dir.as_path());

    if let Ok(entries) = read_dir(&dir_path) {
        for entry in entries.flatten() {
            if entry.path().is_file() {
                //println!("File {:?} found", entry.path());
                if is_file_start(entry.path().as_path().to_str().unwrap()){
                    match reconstruct_file(&entry.path().to_str().unwrap()) {
                        Ok(_v) => {},
                        Err(e) => println!("Constructor method exited with error: {:?}", e),
                    }
                }
                
            }
            else if entry.path().is_dir() {
                let entry_path = clean_absolute_path(&entry.path().to_str().unwrap());
                let mut temp_write_path = PathBuf::from(CONSTRUCTION_DIR);
                temp_write_path.push(Path::new(&entry_path));

                let _ = create_dir_all(&temp_write_path);

                trace_path_construct(entry.path());
            }
        }
    }
    else {
        println!("Failed to read directory {:?}", &dir_path)
    }
}



fn clean_absolute_path(entry: &str) -> String {
    let drive_regex = Regex::new(r"^[A-Za-z]:[\\/]").unwrap();

    let path = drive_regex.replace(&entry, "");
    let out = PathBuf::from(path.as_ref()).to_string_lossy().to_string();

    out
}

fn is_file_start(file_name: &str) -> bool {
    let drive_regex = Regex::new(r"_([0-9]+)\.").unwrap();
    let res = drive_regex.captures(file_name).unwrap().get(1).unwrap().as_str();

    if res == "0" {
        return true;
    } 
    else{
        return false;
    }
}

fn remove_deconstruction_artifact(file_name: &str) -> String {
    let drive_regex = Regex::new(r".?[a-z0-9]*\.bin").unwrap();
    let res: String = drive_regex.replace_all(file_name, "").to_string();
    res

    
}