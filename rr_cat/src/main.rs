use clap::Parser;
use std::{error, fmt, fs, io};

/* #[clap(
short = 'A',
default_value = "false",
confilicts_with="",
)] */

enum InputType {
    TextInput,
    File,
    Folder,
    SymLink,
    // BlockDevice,
}
#[derive(Debug)]
enum CatError {
    IO(io::Error),
    NotFound(String),
    Read,
    PermissionDenied,
    IsFolder(String),
    Other,
}

impl fmt::Display for CatError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let message = match self {
            CatError::IO(e) => format!("IO error:{}", e),
            CatError::NotFound(val) => format!("{}: No such file or directory.", val),
            CatError::Read => "file read error".to_string(),
            CatError::PermissionDenied => "file permission denied".to_string(),
            CatError::IsFolder(val) => format!("{}: Is a directory.", val),
            _ => "other error, but we dont know which one".to_string(),
        };
        f.write_str(&message)
    }
}

impl error::Error for CatError {}

impl From<std::io::Error> for CatError {
    fn from(error: std::io::Error) -> Self {
        match error.kind() {
            std::io::ErrorKind::NotFound => CatError::NotFound(error.to_string()),
            std::io::ErrorKind::PermissionDenied => CatError::PermissionDenied,
            // std::io::ErrorKind::IsADirectory => CatError::IsFolder,
            _ => CatError::Other,
        }
    }
}

fn read_file(file: &String) -> Result<String, CatError> {
    Ok(fs::read_to_string(file)?)
}

fn get_input_type(input: &String) -> Result<InputType, CatError> {
    if input == "-" {
        return Ok(InputType::TextInput);
    }
    match fs::metadata(input) {
        Ok(meta) => {
            if meta.is_file() {
                Ok(InputType::File)
            } else if meta.is_dir() {
                Ok(InputType::Folder)
            } else if meta.is_symlink() {
                Ok(InputType::SymLink)
            /* can only be Permissions::readonly
            } else if meta.permissions() == Permiss {
            return Err(CatError::PermissionDenied); */
            } else {
                todo!()
            }
        }
        Err(_) => Err(CatError::NotFound(input.to_string())),
    }
}

fn open_interactive() -> Result<String, CatError> {
    // #rev
    println!("deppata nur bei stricherl is Type Text net wenn irgendeinstring");
    todo!()
}

fn try_cat_this(input: &String) -> Result<String, CatError> {
    let ft = get_input_type(&input);
    let noflags = true;

    let output = if noflags {
        match ft {
            Ok(InputType::TextInput) => open_interactive(),
            Ok(InputType::File) => read_file(input),
            Ok(InputType::Folder) => Err(CatError::IsFolder(input.to_string())),
            Err(e) => Err(e),
            _ => Err(CatError::Other),
        }
    } else {
        println!("why else?");
        read_file(input)
    };
    output
}

/*
#
try_cat_this(input)
Errors
    --> get_input_type(&input) -> InputType
        if no_flags
            if InputType::Text -> String
            if InputType::File -> read_file -> String
        if show_ends -E
            if InputType::Text -> show_ends -> String
            if InputType::File -> read_file -> show_ends -> String
        if number -n
            if InputType::Text -> number -> String
            if InputType::File -> read_file -> number -> String
*/
#[derive(Parser)]
struct Config {
    /* #[clap(
    short = 'A',
    // default_value = "false",
    // confilicts_with="",
    )] */
    // config_show_all: bool,
    // number_nonblank: bool,
    // show_ends: bool,
    // number: bool,
    // show_tabs: bool,
    // show_nonprinting: bool,
    // help: bool,
    // version: bool,
    args: Vec<String>,
}
fn main() -> io::Result<()> {
    let c = Config::parse();

    let mut output = String::new();
    for (_i, input) in c.args.iter().enumerate() {
        match try_cat_this(input) {
            Ok(out) => output = out,
            Err(e) => eprintln!("rr_cat: {}", e),
        };

        print!("{}", output);
    }

    Ok(())
}


    /* fn testhelper_generate_args(args: &str) -> Vec<String> {
        if args == "files" {
            vec!["test1.md".to_string(), "test2.md".to_string()]
        } else if args == "folders" {
            vec!["folder1".to_string(), "folder2".to_string()]
        } else {
            vec!["somestring".to_string(), "someotherstring".to_string()]
        }
    } */

// #[cfg(tests)]
mod tests {
/*
fn try_cat_this(input: &String) -> Result<String, CatError> {
fn get_input_type(input: &String) -> Result<InputType, CatError> {
fn read_file(file: &String) -> Result<String, CatError> {



*/
    use super::*;
    fn test_try_cat_this_notfound(input: &String){

    }

    #[test]
    fn test_files_test1_test2() {
        let c = Config {
            args: testhelper_generate_args("files"),
        };

        let results: try_cat_this("ramsamama");
            assert_eq!(, );

        /* assert_eq!(
        results,
            "# test1
Lorem ipsum dolor sit amet, qui minim labore adipisicing minim sint cillum sint consectetur cupidatat.
# test2
Lorem ipsum dolor sit amet, officia excepteur ex fugiat reprehenderit enim labore culpa sint ad nisi Lorem pariatur mollit ex esse exercitation amet. Nisi anim cupidatat excepteur officia. Reprehenderit nostrud nostrud ipsum Lorem est aliquip amet voluptate voluptate dolor minim nulla est proident. Nostrud officia pariatur ut officia. Sit irure elit esse ea nulla sunt ex occaecat reprehenderit commodo officia dolor Lorem duis laboris cupidatat officia voluptate. Culpa proident adipisicing id nulla nisi laboris ex in Lorem sunt duis officia eiusmod. Aliqua reprehenderit commodo ex non excepteur duis sunt velit enim. Voluptate laboris sint cupidatat ullamco ut ea consectetur et est culpa et culpa duis.".to_string()
        ) */
    }
}
