use std::{fs,path};
use parser;

pub fn list(path: &str) -> Vec<parser::File> {
    let files: Vec<parser::File> = Vec::new();
    list_files(path, files)
}

pub fn read(filename: &str) -> String {
    fs::read_to_string(filename).expect("Error reading file")
}

fn list_files(path: &str, mut files: Vec<parser::File>) -> Vec<parser::File> {
    for entry in fs::read_dir(path).expect("Error reading dir") {
        let entry = entry.expect("Invalid dir entry");
        let path = entry.path();
        let ps = path.to_str().expect("Invalid path");
        if path.is_dir() {
            files = list_files(ps, files);
            continue;
        }

        if let Some(kind) = determine_file_kind(&path) {
            files.push(parser::File{
                kind,
                path: ps.to_string(),
            });
        }
    }
    files
}

fn determine_file_kind(path: &path::PathBuf) -> Option<parser::FileType> {
    match path.extension().unwrap().to_str().expect("Invalid extension") {
        "js" => Some(parser::FileType::JavaScript),
        "rs" => Some(parser::FileType::Rust),
        "py" => Some(parser::FileType::Python),
        "sh" => Some(parser::FileType::Shell),
        "go" => Some(parser::FileType::Go),
        "php" => Some(parser::FileType::Php),
        "sql" => Some(parser::FileType::Sql),
        _ => None
    }
}
