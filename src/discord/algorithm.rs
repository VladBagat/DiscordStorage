use std::fs::{create_dir_all, read_dir, File};
use std::io::{Read, Write, Result, SeekFrom, Seek};
use std::path::{PathBuf, Path};
use std::vec;
use regex::Regex;

const DIRECTORY: &str = r"D:/DiscStore";
const DECONSTRUCTION_DIR: &str = "DeconstructorResult";
const CONSTRUCTION_DIR: &str = "ConstructorResult";
const FILE_SIZE: usize = 8 * 1024 * 1024;

pub fn deconstruct(target: &str) -> Result<Vec<PathBuf>> {
    let root = PathBuf::from(target);
    let res: Vec<PathBuf> = trace_path_deconstruct(&root);
    Ok(res)
}

pub fn reconstruct() -> Result<()> {
    let construct_root = PathBuf::from(DECONSTRUCTION_DIR);
    trace_path_construct(&construct_root);
    Ok(())
}

fn deconstruct_file(file_path: &str) -> Result<Vec<String>> {
    let mut file_index: u32 = 0;
    let read_path = file_path;
    let mut local_vec: Vec<String> = vec![];
    
    loop {
        let write_path_base = clean_absolute_path(file_path);
        let write_path = format!(r"{}/{}_{}.bin", DECONSTRUCTION_DIR, write_path_base, &file_index);
        let (buffer, read_bytes) = read_chunk(&file_index, &read_path)?;
        
        if read_bytes == 0 {
            break;
        }

        if let Some(parent) = Path::new(&write_path).parent() {
            match create_dir_all(parent){
                Ok(_) => {},
                Err(e) => {println!("Creating directory {} failed: {:?}", parent.to_str().unwrap(), e);},
            }
        }

        match write_file(&write_path, buffer){
            Ok(_) => {local_vec.push(write_path.clone())},
            Err(e) => {println!("Writing deconstruction file {} failed: {:?}", &write_path, e);},
        }
        
        file_index += 1;
        

        if read_bytes < FILE_SIZE {
            break;
        }
    }

    Ok(local_vec)
}

fn reconstruct_file(file_path: &str) {

    let mut write_result: Vec<u8> = vec![];
    let file_path = remove_deconstruction_artifact(file_path);
    let write_path = file_path.replace(r"DeconstructorResult", "ConstructorResult"); 
    let mut index: i32 = 0;
    

    loop{
        let read_path = format!("{}_{}.bin", file_path, index);
        match read_chunk(&0, &read_path) {
            Ok((mut buffer, read_bytes)) => {
                write_result.append(&mut buffer);
                index += 1;
                if read_bytes < FILE_SIZE {
                    break;
                }
            },
            Err(e) => {println!("Reading reconstrucion file {} chunk failed: {:?}", &read_path, e); break;},
        }
    }

    if let Some(parent) = Path::new(&write_path).parent() {
        match create_dir_all(parent){
            Ok(_) => {},
            Err(e) => {println!("Creating directory {} failed: {:?}", parent.to_str().unwrap(), e);},
        }
    }

    match write_file(&write_path, write_result){
        Ok(_) => {},
        Err(e) => {println!("Writing reconstruction file {} failed: {:?}", &write_path, e);},
    }
}

fn read_chunk(file_index:&u32, path: &str) -> Result<(Vec<u8>, usize)> {
    let mut f: File = File::open(path)?;
    let index: u32 = FILE_SIZE.try_into().expect("Value out of range for u32");
    f.seek(SeekFrom::Start((file_index * index).into()))?;

    // Allocate 10 MB on the heap
    let mut buffer = vec![0u8; FILE_SIZE];

    let n = f.read(&mut buffer[..])?;
    
    Ok((buffer[..n].to_vec(), n))
}

fn write_file(file_path:&str, buffer: Vec<u8>) -> Result<()> {
    //TODO: change file_path &str to PathBuf
    let mut file = File::create(format!("{}", file_path))?; 
    file.write(&buffer)?; 

    Ok(())
}

fn process_deconstruct_files(top_dir: &Path) -> Option<Vec<PathBuf>>{
    let mut file_paths: Vec<PathBuf> = vec![];
    if let Ok(entries) = read_dir(top_dir) {
        for entry in entries.flatten() {
            if entry.path().is_file() && is_definitely_file(&entry.path()) {
                let path = entry.path();
                match deconstruct_file(path.to_str().unwrap()){
                    Ok(local_vec) => {
                        let mut temp_vec = Vec::new();
                            for component in &local_vec {
                                temp_vec.push(PathBuf::from(component));
                            }
                        file_paths.append(&mut temp_vec);
                    },
                    Err(e) => {println!("Error deconstructing file {:?}: {:?}", entry.path(), e);},
                }
            } else if entry.path().is_dir() {
                if let Some(mut sub_paths) = process_deconstruct_files(&entry.path()) {
                    file_paths.append(&mut sub_paths);
                }
            }
        }
    } else {
        println!("Failed to read directory {:?}", top_dir);
    }
    Some(file_paths)
}

fn trace_path_deconstruct(top_dir: &PathBuf) -> Vec<PathBuf> {
    match process_deconstruct_files(&top_dir){
        Some(file_paths) => {file_paths},
        None => panic!("Error processing deconstruction files"),
    }
}

fn trace_path_construct(top_dir: &PathBuf) {
    if let Ok(entries) = read_dir(top_dir) {
        for entry in entries.flatten() {
            if entry.path().is_file() && is_definitely_file(&entry.path()) {
                //TODO: This allows some directories to pass as files.?
                if is_file_start(entry.path().to_str().unwrap()) {
                    reconstruct_file(entry.path().to_str().unwrap());
                }
            } else if entry.path().is_dir() {
                trace_path_construct(&entry.path());
            }
        }
    } else {
        println!("Failed to read directory {:?}", top_dir);
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
    return false;
}

fn remove_deconstruction_artifact(file_name: &str) -> String {
    let drive_regex = Regex::new(r"_\d+\.bin$").unwrap();
    let res: String = drive_regex.replace_all(file_name, "").to_string();
    res
}

fn is_definitely_file(path: &Path) -> bool {
    // First check: Must have a file extension
    if path.extension().is_none() {
        return false;
    }

    // Second check: Try to open as file and read a byte
    match File::open(path) {
        Ok(mut file) => {
            let mut buffer = [0u8; 1];
            match file.read_exact(&mut buffer) {
                Ok(_) => true,
                Err(_) => false
            }
        }
        Err(_) => false
    }
}