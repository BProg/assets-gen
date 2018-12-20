use std::env;
use std::path::{Path, PathBuf};
use std::io;
use std::fs;

struct TypeScriptImageUriEnum {
    enum_key: String,
    enum_value: String,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.get(1) {
        None => panic!("Missing path"),
        Some(folder) => {
            let enum_result = make_typescript_enum(folder);
            match enum_result {
                Ok(ts_enum) => println!("{}", ts_enum),
                Err(e) => println!("ERROR: {}", e),
            }
        }
    };
}

fn make_typescript_enum(folder: &String) -> Result<String, io::Error> {
    let steam_and_names = map_file_stem_to_name(folder)?;
    let typescript_files_uri = create_typescript_image_uri_enum(steam_and_names);
    print!("{}", typescript_files_uri);
    return Ok(typescript_files_uri);
}

fn map_file_stem_to_name(folder: &String) -> Result<Vec<TypeScriptImageUriEnum>, io::Error> {
    let paths = read_file_paths_in_folder(folder)?;
    let mut filenames: Vec<TypeScriptImageUriEnum> = vec!();
    for path in paths {
        if let (Some(file_name_os), Some(file_stem_os)) = (path.file_name(), path.file_stem()) {
            let filename = file_name_os.to_str();
            let filestem = file_stem_os.to_str();
            if let (Some(name), Some(stem)) = (filename, filestem) {
                filenames.push(TypeScriptImageUriEnum {
                    enum_key: String::from(stem),
                    enum_value: String::from(name),
                });
            }
        }
    }
    Result::Ok(filenames)
}

fn read_file_paths_in_folder(folder: &String) -> Result<Vec<PathBuf>, io::Error> {
    let folder_path = Path::new(folder);
    let dir_entries_iter = folder_path.read_dir()?;
    let mut paths: Vec<PathBuf> = vec!();
    for entry_result in dir_entries_iter {
        let entry = entry_result?;
        let file_path_buf = entry.path();
        paths.push(file_path_buf);
    }
    Result::Ok(paths)
}

fn create_typescript_image_uri_enum(file_steam_name: Vec<TypeScriptImageUriEnum>) -> String {
    let mut typescript_files_uri = String::from("export const enum IconUri {\n");
    println!("There are {} files", file_steam_name.len());
    let pngs = file_steam_name.into_iter()
        .filter(|file_s_and_n| file_s_and_n.enum_value.ends_with("png"))
        .collect::<Vec<TypeScriptImageUriEnum>>();
    println!("There are {} images", pngs.len());
    for steam_and_name in pngs {
        let uri_map = format!("    {}: res://drawable/default/{},\n", steam_and_name.enum_key, steam_and_name.enum_value);
        typescript_files_uri.push_str(uri_map.as_ref());
    }
    typescript_files_uri.push_str("}\n");
    typescript_files_uri
}
