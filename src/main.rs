use {
    clap::{Error as ClapError, ErrorKind as ClapErrorKind},
    std::{
        fs::File,
        io::{stdin, Read, Write},
        path::PathBuf,
        str::FromStr,
    },
    structopt::StructOpt,
    toml_edit::{value, Document, Item},
};

#[derive(Debug)]
struct Key(pub Vec<String>, pub String);

impl FromStr for Key {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let key: Vec<String> = s
            .split('.')
            .filter_map(|s| {
                if s.is_empty() {
                    None
                } else {
                    Some(s.to_string())
                }
            })
            .collect();

        if key.is_empty() {
            Err("Key must be form of KEY[.SUBKEY ...]".to_string())
        } else {
            Ok(Key(key, s.to_string()))
        }
    }
}

#[derive(Debug)]
struct Entry {
    key: Key,
    value: String,
}

impl FromStr for Entry {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        static ERR_MSG: &'static str = "Entry must be form of KEY[.SUBKEY...]=VALUE";

        let mut split = s.split('=');
        let key = split
            .next()
            .ok_or_else(|| ERR_MSG.to_string())
            .and_then(|s| Key::from_str(s))?;
        let value = split.next().ok_or_else(|| ERR_MSG.to_string())?.to_string();

        Ok(Entry { key, value })
    }
}

#[derive(Debug, StructOpt)]
struct Opt {
    /// Toml file to read. default: stdin
    #[structopt(short, long = "input-file", raw(value_name = "\"FILE\""))]
    input_file: Option<PathBuf>,
    /// Toml file to write. default: stdout
    #[structopt(short, long = "output-file", raw(value_name = "\"FILE\""))]
    output_file: Option<PathBuf>,
    /// Keys to delete
    #[structopt(
        short,
        long,
        raw(value_name = "\"KEY\"", multiple = "true", number_of_values = "1")
    )]
    delete: Vec<Key>,
    /// Entries to modify. Must be form of KEY[.SUBKEY...]=VALUE
    #[structopt(parse(try_from_str), raw(value_name = "\"KEY=VALUE\""))]
    entries: Vec<Entry>,
}

fn find_value_by_key<'a>(key: &[String], item: &'a mut Item) -> Option<&'a mut Item> {
    if key.is_empty() {
        if !item.is_none() {
            Some(item)
        } else {
            None
        }
    } else if let Some(table) = item.as_table_mut() {
        find_value_by_key(&key[1..], table.entry(&key[0]))
    } else {
        None
    }
}

fn error_into(error: impl std::fmt::Display) -> ClapError {
    ClapError::with_description(&format!("{}", error), ClapErrorKind::Io)
}

fn run() -> Result<(), ClapError> {
    let opt = Opt::from_args();

    let content = if let Some(path) = opt.input_file {
        let mut file = File::open(path).map_err(error_into)?;
        let mut content = String::new();

        file.read_to_string(&mut content).map_err(error_into)?;

        content
    } else {
        let stdin = stdin();
        let mut stdin = stdin.lock();
        let mut content = String::new();

        stdin.read_to_string(&mut content).map_err(error_into)?;

        content
    };

    let mut document = Document::from_str(&content).map_err(error_into)?;

    for key in opt.delete {
        let v = find_value_by_key(&key.0, &mut document.root)
            .ok_or_else(|| error_into(format!("could not find `{}`", key.1)))?;
        *v = toml_edit::Item::None;
    }

    for entry in opt.entries {
        let v = find_value_by_key(&entry.key.0, &mut document.root)
            .ok_or_else(|| error_into(format!("could not find `{}`", entry.key.1)))?;
        *v = value(entry.value);
    }

    if let Some(path) = opt.output_file {
        let mut file = File::create(path).map_err(error_into)?;

        file.write_all(&document.to_string().as_bytes())
            .map_err(error_into)?;
    } else {
        print!("{}", document);
    }

    Ok(())
}

fn main() {
    let res = run();

    if let Err(err) = res {
        err.exit();
    }
}
