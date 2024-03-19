use std::error::Error;
use std::fs;
use std::env::current_dir;
use std::path::PathBuf;

enum Subdirectories {
    Archives,
    Code,
    Documents,
    Music,
    Pictures,
    Videos,
}

struct DirFileMap<'a>{
    subdirectory: &'a Subdirectories,
    filetypes: Vec<&'a str>,
}


struct FileMatch<'a> {
    directory: &'a Subdirectories,
    file_path: fs::DirEntry
}

impl <'a> FileMatch<'a> {
    fn new(directory: &'a Subdirectories, file_path: fs::DirEntry) -> FileMatch<'a> {
        FileMatch {
            directory,
            file_path,
        }
    }
}

fn get_files(directory: PathBuf, master_dir_file_map: &[DirFileMap<'static>]) -> Result<Vec<FileMatch<'static>>, Box<dyn Error>> {
    let directory_files = fs::read_dir(&directory)?;
    let mut file_match_maps: Vec<FileMatch> = Vec::new();

    for directory_file in directory_files {
        let directory_file = directory_file?;
        if directory_file.path().is_file() {
            let file_extension = match directory_file.path().extension().and_then(|e| e.to_str()) {
                None => {continue}
                Some(extension) => {extension.to_owned()}
            };

            for dir_file_map in master_dir_file_map {
                // let file_extension_str = file_extension;
                if dir_file_map.filetypes.contains(&&*file_extension) {
                    file_match_maps.push(FileMatch::new(
                        dir_file_map.subdirectory,
                        directory_file
                    ));
                    break;
                }
            }
        }
    }
    Ok(file_match_maps)
}

fn main() -> Result<(), Box<dyn Error>> {
    let current_dir = current_dir()?;
    let dir_file_map: &[DirFileMap<'static>] = &[
    DirFileMap { subdirectory: &Subdirectories::Archives, filetypes: vec!["tar", "gz", "xz", "bz2", "zip", "7z", "rar", "dmg,"] },
    DirFileMap { subdirectory: &Subdirectories::Music, filetypes: vec!["mp3", "mp4a", "wav"] },
    DirFileMap { subdirectory: &Subdirectories::Pictures, filetypes: vec!["jpg", "jpeg", "gif", "bmp", "pmg", "svg", "heif"] },
    DirFileMap { subdirectory: &Subdirectories::Videos, filetypes: vec!["mp4", "mkv", "mov", ] },
    DirFileMap { subdirectory: &Subdirectories::Documents, filetypes: vec!["pdf", "epub", "doc", "docx", "xls", "xlsx", "ppt", "pptx", "odt", "rtf", "md", ] },
    DirFileMap { subdirectory: &Subdirectories::Documents, filetypes: vec!["py", "rs", "rb", "html", "htm", "js", "jar", "java", "c", "cpp", "toml", "json", "yaml", "tex", "cs", "nix", "pl", "h", "hpp", "css", "php", "swift", "go", "lua", "ts", "scss", "sass", "dart", "sql", "ini", "sh", "bash", "zsh", "ps1", "fish", "r", "yml", "bat", "cmd", "asm"] }
];
    let files_to_organize = match get_files(current_dir, dir_file_map) {
        Ok(files) => files,
        Err(err) => panic!("Failed to get files: {:?}", err)
    };
    Ok(())
}
