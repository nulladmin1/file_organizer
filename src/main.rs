// Acknowledgements:
// All the Standard Libraries are created by the Rust Team
// Clap is by the clap-rs team (https://clap.rs)

use std::error::Error;
use std::fs;
use std::env::current_dir;
use std::path::{Path, PathBuf};
use clap::{command, Arg};

fn main() -> Result<(), Box<dyn Error>> {
    let arg_matches = command!()
        .arg(
            Arg::new("directory")
                .required(false)
                .help("The directory where files should be organized (CWD if not given).")
        )
        .arg(
            Arg::new("verbosity")
                .short('v')
                .long("verbosity")
                .index(1)
        )
        .version("1.0")
        .about("Organizes files in a given directory (CWD if not given).")
        .get_matches();

    println!("{:?}", arg_matches.get_one::<String>("verbosity").unwrap());

    let path: PathBuf = arg_matches.get_one("directory").unwrap().into();
    // let current_dir = current_dir()?;
    // let files_to_organize = get_files(&current_dir)?;
    // organize_files(&current_dir, &files_to_organize)?;

    Ok(())
}

enum Subdirectories {
    Archives,
    Code,
    Documents,
    Music,
    Pictures,
    Videos,
}

impl AsRef<Path> for Subdirectories {
    fn as_ref(&self) -> &Path {
        match self {
            Subdirectories::Archives => Path::new("Archives"),
            Subdirectories::Code => Path::new("Code"),
            Subdirectories::Documents => Path::new("Documents"),
            Subdirectories::Music => Path::new("Music"),
            Subdirectories::Pictures => Path::new("Pictures"),
            Subdirectories::Videos => Path::new("Videos"),
        }
    }
}

struct DirFileMap<'a>{
    subdirectory: &'a Subdirectories,
    filetypes: &'a [&'a str],
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

const DIR_FILE_MAP: &[DirFileMap<'static>] = &[
    DirFileMap { subdirectory: &Subdirectories::Archives, filetypes: &["tar", "gz", "xz", "bz2", "zip", "7z", "rar", "dmg,"] },
    DirFileMap { subdirectory: &Subdirectories::Music, filetypes: &["mp3", "mp4a", "wav"] },
    DirFileMap { subdirectory: &Subdirectories::Pictures, filetypes: &["jpg", "jpeg", "gif", "bmp", "pmg", "svg", "heif"] },
    DirFileMap { subdirectory: &Subdirectories::Videos, filetypes: &["mp4", "mkv", "mov", ] },
    DirFileMap { subdirectory: &Subdirectories::Documents, filetypes: &["pdf", "epub", "doc", "docx", "xls", "xlsx", "ppt", "pptx", "odt", "rtf", "md", ] },
    DirFileMap { subdirectory: &Subdirectories::Code, filetypes: &["py", "rs", "rb", "html", "htm", "js", "jar", "java", "c", "cpp", "toml", "json", "yaml", "tex", "cs", "nix", "pl", "h", "hpp", "css", "php", "swift", "go", "lua", "ts", "scss", "sass", "dart", "sql", "ini", "sh", "bash", "zsh", "ps1", "fish", "r", "yml", "bat", "cmd", "asm"] }
];


fn get_files(directory: &PathBuf) -> Result<Vec<FileMatch<'static>>, Box<dyn Error>> {
    let directory_files = fs::read_dir(&directory)?;
    let mut file_match_maps: Vec<FileMatch> = Vec::new();

    for directory_file in directory_files {
        let directory_file = directory_file?;
        if directory_file.path().is_file() {
            let path = directory_file.path();
            let path_extension = path.extension();

            let file_extension = match path_extension {
                Some(extension) => {
                    if let Some(extension_str) = extension.to_str() {
                        extension_str
                    } else {
                        continue
                    }
                }
                None => {continue}
            };

            for dir_file_map in DIR_FILE_MAP {
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

fn organize_files(directory: &PathBuf, filematches: &Vec<FileMatch<'static>>) -> Result<(), Box<dyn Error>> {
    for subdirectory in DIR_FILE_MAP {
        let subdir_path = Path::join(&directory, subdirectory.subdirectory);
        fs::create_dir_all(subdir_path)?;
    }
    for file_match in filematches {
        println!("{:?}", file_match.file_path);

        let subdirectory_file_path = Path::new(file_match.directory.as_ref());
        let file_name = match file_match.file_path.path().file_name() {
            Some(file_name) => file_name.to_owned(),
            None => continue,
        };
        let destination_file = subdirectory_file_path.join(&file_name);

        fs::rename(&file_match.file_path.path(), &destination_file)?;
    }

    Ok(())
}

