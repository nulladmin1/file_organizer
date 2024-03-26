// Acknowledgements:
// All the Standard Libraries are created by the Rust Team
// Clap is made and maintained by the clap-rs team (https://clap.rs)
// Ansi_Term is made and maintaned mainly by Benjamin Sago (https://github.com/ogham)

use ansi_term;
use std::fmt::{Display, Formatter};
use std::fs::{self, read_dir};
use std::env::current_dir;
use std::error::Error;
use std::path::{Path, PathBuf};
use ansi_term::Color;
use clap::{command, Arg, ArgAction};

fn main() -> Result<(), Box<dyn Error>> {
    let arg_matches = command!()
        .arg(
            Arg::new("directory")
                .required(false)
                .help("The directory where files should be organized (CWD if not given).")
        )
        .arg(
            Arg::new("verbose")
                .short('v')
                .long("verbose")
                .action(ArgAction::SetTrue)
                .help("Enable verbosity.")
        )
        .arg(
            Arg::new("list-supported-files")
                .short('l')
                .long("list")
                .action(ArgAction::SetTrue)
                .help("List files supported by file_organizer")
                .exclusive(true)
        )
        .version("1.0")
        .about("Organizes files in a given directory (CWD if not given).")
        .get_matches();

    if arg_matches.get_flag("list-supported-files") {
        println!("{}", Color::White.bold().paint("Supported files: "));
        for dir_file_map in DIR_FILE_MAP {
            println!("{}", dir_file_map)
        }
    } else {
        let verbose = arg_matches.get_flag("verbose");

        let directory_path: PathBuf = match arg_matches.get_one::<String>("directory") {
            Some(path) => path.into(),
            None => current_dir()?,
        };

        if verbose {
            let files = read_dir(&directory_path).unwrap();
            println!("{}", Color::White.bold().paint("Files in directory: "));
            for file in files {
                println!("{}", Color::Green.paint(file.unwrap().file_name().to_str().unwrap()));
            }
            println!();
        }

        let files_to_organize = get_files(&directory_path)?;
        organize_files(&directory_path, &files_to_organize, verbose)?;
    }
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

impl Display for Subdirectories {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let subdirectory_string = match self {
            Subdirectories::Archives => "Archives",
            Subdirectories::Code => "Code",
            Subdirectories::Documents => "Documents",
            Subdirectories::Music => "Music",
            Subdirectories::Pictures => "Pictures",
            Subdirectories::Videos => "Videos",
        };
        write!(f, "{}", subdirectory_string)        
    }
}

struct DirFileMap<'a>{
    subdirectory: &'a Subdirectories,
    filetypes: &'a [&'a str],
}

impl<'a> Display for DirFileMap<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {:?}", Color::White.paint(self.subdirectory.to_string()), self.filetypes)
    }
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

impl <'a> Display for FileMatch<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Directory {}; File: {}", self.directory, self.file_path.path().to_str().unwrap())
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
    let directory_files = fs::read_dir(directory)?;
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
                if dir_file_map.filetypes.contains(&file_extension) {
                    file_match_maps.push(
                        FileMatch::new(
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

fn organize_files(directory: &Path, filematches: &Vec<FileMatch<'static>>, verbose: bool) -> Result<(), Box<dyn Error>> {
    for subdirectory in DIR_FILE_MAP {
        let subdir_path = Path::join(directory, subdirectory.subdirectory);

        if verbose {
            println!("{} {}", Color::White.bold().paint("Created directory: "),  Color::Cyan.paint(&subdirectory.subdirectory.to_string()));
        }
        fs::create_dir_all(subdir_path)?;
    }
    for file_match in filematches {
        let subdirectory_file_path = Path::join(directory, file_match.directory.as_ref());

        let file_name = match file_match.file_path.path().file_name() {
            Some(file_name) => file_name.to_owned(),
            None => continue,
        };
        let destination_file = subdirectory_file_path.join(&file_name);
        
        println!("Moving {} to {}... ", Color::Green.paint(file_match.file_path.path().to_str().unwrap()), Color::Green.paint(destination_file.to_str().unwrap()));

        fs::rename(&file_match.file_path.path(), &destination_file)?;
    }

    Ok(())
}

