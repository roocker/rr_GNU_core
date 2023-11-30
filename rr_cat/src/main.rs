use clap::Parser;
use std::{
    error, fmt,
    fs::{self, File},
    io,
    io::{BufRead, Write},
    path::Path,
};

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
    Read(io::Error),
    PermissionDenied,
    IsSymLink, // #rev which i could propably handle instead of error outputting
    IsFolder(String),
    Other,
}

impl fmt::Display for CatError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let message = match self {
            CatError::IO(e) => format!("IO error:{}", e),
            CatError::NotFound(val) => format!("{}: No such file or directory.", val),
            CatError::Read(e) => format!("file read error: {}", e),
            CatError::PermissionDenied => "file permission denied".to_string(),
            CatError::IsSymLink => "ahh symlink, is this a problem?".to_string(),
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

/// - i decided that i prefer having vectors in the `Results` so i dont have to split lines manually and make this os compatible; Would having a single String and moving that around make my program more lightwight, or effient?

type VecCatResult = Result<Vec<String>, CatError>;
// type StringCatResult = Result<String, CatError>;

/* fn read_file(file: &str) -> StringCatResult {
    Ok(fs::read_to_string(file)?)
} */

fn read_file_bylines(file: &str) -> VecCatResult {
    let content = fs::read_to_string(file)?
        .lines()
        .map(String::from)
        .collect();
    Ok(content)
}

// type ImplCatResult = Re

// the right way:
/* fn read_file_bylines<F: AsRef<Path>>(filepath: F) -> io::Result<io::Lines<io::BufReader<File>>> {
    let file = File::open(filepath)?;
    Ok(io::BufReader::new(file).lines())
} */

fn remove_blank_lines(input: VecCatResult) -> VecCatResult {
    let mut lines = match input {
        Ok(vec) => vec,
        Err(e) => return Err(e),
    };
    lines.retain(|line| !line.is_empty());
    Ok(lines)
}

fn squeeze_blank_lines(input: VecCatResult, max_empty: u8) -> VecCatResult {
    /* let lines = match input {
        Ok(vec) => vec,
        Err(e) => return Err(CatError::Read(e)),
    }; */
    let lines = input?;
    // eprintln!("read lines");

    let mut counter = 0;
    let mut output = Vec::new();

    for line in lines.iter() {
        if line.trim().is_empty() {
            // eprintln!(" empty line, counter: {} line:{}", counter, line);
            if counter > max_empty - 1 {
                continue;
            } else {
                counter += 1;
                output.push("".to_string());
                // output.push_str(&format!("----:{}\n", counter));
            }
        } else {
            counter = 0;
            output.push(line.to_string());
        }
    }

    // eprintln!("squeeze blank done");
    Ok(output)
}

fn add_linenumbers(input: VecCatResult, empty_lines: bool) -> VecCatResult {
    /* let input = match input {
        Ok(vec) => vec,
        Err(e) => return Err(CatError::Read(e)),
    }; */

    let input = input?;

    let mut output = Vec::new();

    let mut linenumber = 0;
    for line in input.iter() {
        if empty_lines & line.trim().is_empty() {
            output.push("\n".to_string())
        } else {
            linenumber += 1;
            output.push(format!("{:6}\t{}", linenumber, line))
        }
    }

    Ok(output)
}

fn add_ends(input: VecCatResult) -> VecCatResult {
    /* let input = match input {
        Ok(vec) => vec,
        Err(e) => return Err(CatError::Read(e)),
    }; */
    let input = input?;
    let mut output = Vec::new();
    for line in input.iter() {
        output.push(format!("{}$", line))
    }
    Ok(output)
}

fn get_input_type(input: &String) -> Result<InputType, CatError> {
    if input == "-" {
        return Ok(InputType::TextInput);
    }
    match fs::metadata(input) {
        Ok(meta) => {
            // eprintln!("\nMETADATA:\n{:?}", meta);
            if meta.is_file() {
                Ok(InputType::File)
            } else if meta.is_symlink() {
                Ok(InputType::SymLink)
            } else if meta.is_dir() {
                Ok(InputType::Folder)
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

fn open_interactive() -> VecCatResult {
    // #rev
    todo!()
}

fn try_cat_this(config: Config, mut writer: impl io::Write) -> Result<(), Vec<CatError>> {
    let mut errors: Vec<CatError> = Vec::new();

    let input = config.args;

    for (_i, arg) in input.iter().enumerate() {
        let ft = get_input_type(arg);

        let output = match ft {
            Ok(InputType::TextInput) => continue,
            /* Ok(InputType::File) => {
                if config.squeeze_blank {
                    squeeze_blank_lines(read_file_byline(arg), 2)
                } else if config.number {
                    add_linenumbers(read_file_byline(arg), false)
                } else if config.number_nonblank {
                    add_linenumbers(read_file_byline(arg), true)
                } else if config.show_ends {
                    add_ends(read_file_byline(arg))
                } else if false {
                    todo!();
                } else {
                    read_file(arg)
                }
            } */
            Ok(InputType::File) => {
                let mut v = read_file_bylines(arg);

                if config.squeeze_blank {
                    v = squeeze_blank_lines(v, 2)
                }
                if config.number {
                    v = add_linenumbers(v, false)
                }
                if config.number_nonblank {
                    v = add_linenumbers(v, true)
                }
                if config.show_ends {
                    v = add_ends(v)
                }
                v
            }
            // Ok(InputType::Folder) => Err(CatError::IsFolder(arg.to_string())),
            Ok(InputType::Folder) => {
                errors.push(CatError::IsFolder(arg.to_string()));
                continue;
            }
            Ok(InputType::SymLink) => {
                errors.push(CatError::IsSymLink);
                continue;
            }
            Err(e) => {
                errors.push(e);
                continue;
            }
        };

        match output {
            Ok(s) => {
                let s: String = s.join("\n");
                if let Err(e) = write!(writer, "{}", s) {
                    errors.push(CatError::IO(e));
                }
            }
            Err(e) => {
                errors.push(e);
            }
        }
    }
    if errors.is_empty() {
        Ok(())
    } else {
        for e in &errors {
            eprintln!("rr_cat: {}", e);
        }
        Err(errors)
    }
}

#[derive(Parser)]
struct Config {
    /* #[arg(
    short = 'A',
    // default_value = "false",
    // conflicts_with="",
    )] */
    // config_show_all: bool,
    // show_tabs: bool,
    // show_nonprinting: bool,
    // #[arg(short = 'h', long = "help")]
    // help: bool,
    // version: bool,
    #[arg(
        short = 's',
        default_value = "false",
        long = "squeeze-blank",
        // conflicts_with = "number",
        help = "suppress repeated empty output lines",
    )]
    squeeze_blank: bool,
    #[arg(
        short = 'b',
        default_value = "false",
        long = "number-nonblank",
        conflicts_with = "number",
        help = "number nonempty output lines, override -n"
    )]
    number_nonblank: bool,
    #[arg(
        short = 'E',
        default_value = "false",
        long = "show-ends",
        help = "display $ at the end of each line"
    )]
    show_ends: bool,
    #[arg(
        short = 'n',
        default_value = "false",
        long = "number",
        conflicts_with = "number_nonblank",
        help = "show line numbers"
    )]
    number: bool,
    args: Vec<String>,
}

fn main() -> io::Result<()> {
    let c = Config::parse();

    let mut io = io::stdout();

    /* if c.help {
        writeln!(io, "{}", HELP)?; //#rev write error possible?! handle!
    } else {
    } */

    let _ = try_cat_this(c, io); // what to do with this unused Result in main?, did i handle all errors already? why is this function then returning a result even?

    // let result = byline(c.args);

    Ok(())
}

#[cfg(tests)]
mod tests {
    use super::*;

    fn testhelper_generate_args(arg: &str) -> Vec<String> {
        if arg == "files" {
            vec!["test1.md".to_string(), "test2.md".to_string()]
        } else if arg == "folders" {
            vec!["folder1".to_string(), "folder2".to_string()]
        } else if arg == "folders_and_string" {
            vec!["folder1".to_string(), "dis_is_nono_folder".to_string()]
            // vec!["folder1".to_string(), "folder2".to_string()]
        } else if arg == "random" {
            vec!["somestring".to_string(), "someotherstring".to_string()]
        } else if arg == "test3" {
            vec!["test3.md".to_string()]
        } else {
            vec![]
        }
    }

    #[test]
    fn test_try_cat_this_files() {
        let v = testhelper_generate_args("files");

        let mut result = Vec::new();
        let _ = try_cat_this(v, &mut result);

        let expected = b"# test1\r\nLorem ipsum dolor sit amet, qui minim labore adipisicing minim sint cillum sint consectetur cupidatat.\r\n# test2\r\ntest file 2\r\n";

        assert_eq!(result, expected);
    }

    #[test]
    fn test_try_cat_this_err_folders() {
        let v = testhelper_generate_args("folders");

        let mut demostdout = Vec::new();

        let expected = v.clone();
        let result = try_cat_this(v, &mut demostdout);

        if let Err(errors) = result {
            assert_eq!(errors.len(), expected.len());

            for (error, expected) in errors.iter().zip(expected.iter()) {
                assert!(matches!(error, CatError::IsFolder(_)));
                if let CatError::IsFolder(result_msg) = error {
                    assert_eq!(result_msg, expected)
                }
            }
        } else {
            panic!("Expected an error, but something went all to good");
        }
    }

    #[test]
    fn test_try_cat_this_err_folders_and_notfound() {
        let v = testhelper_generate_args("folders_and_string");

        let mut demostdout = Vec::new();

        let expected = v.clone();
        let result = try_cat_this(v, &mut demostdout);

        if let Err(errors) = result {
            assert_eq!(errors.len(), expected.len());

            match &errors[0] {
                CatError::IsFolder(result_msg) => assert_eq!(result_msg, &expected[0]),
                _ => panic!("Unexpected error variant. Error 1 should be IsFolder"),
            }

            match &errors[1] {
                CatError::NotFound(result_msg) => assert_eq!(result_msg, &expected[1]),
                _ => panic!("Unexpected error variant. Error 2 should be NotFound"),
            }
        } else {
            panic!("Expected an error, but something went all too good");
        }
    }
}
