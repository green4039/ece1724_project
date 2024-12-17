//use anyhow::Error;
//use dirs::config_dir;
use rusqlite::{Connection, Result};
use std::env;
//use std::fmt::format;
//use std::ffi::OsStr;
use regex::Regex;
use std::fs;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::io::{self, BufRead};
use std::path::Path;
use std::path::PathBuf;

#[derive(Debug)]
struct Person {
    id: i32,
    name: String,
    data: i32,
}

#[derive(Debug)]
struct Database {
    table_names: Vec<String>,
    config_dir: PathBuf,
    conn: Connection,
}

impl Database {
    fn new() -> Self {
        Database {
            table_names: Vec::<String>::new(),
            config_dir: PathBuf::new(),
            conn: Connection::open_in_memory().expect("Unable to set up connection"),
        }
    }

    // TODO: decide how we want to implement this
    fn execute_query(&mut self, query: String) -> Result<(), anyhow::Error> {
        self.conn.execute(
            query.as_str(),
            (), // empty list of parameters.
        )?;
        Ok(())
    }

    fn load_table_definitions(&mut self) -> Result<(), anyhow::Error> {
        // https://stackoverflow.com/questions/46749360/how-to-get-only-the-directory-portion-of-the-current-executables-path
        let toml_dir = env!("CARGO_MANIFEST_DIR");
        let mut table_dir = PathBuf::from(String::from(toml_dir));
        table_dir.extend(&["src", "table_definitions"]);
        for entry in fs::read_dir(table_dir)? {
            let entry = entry?;
            let path = entry.path();
            let sql_def: String = fs::read_to_string(path.clone())?;
            println!("table def: {}", sql_def);
            // save table names
            // assumption: file name is the table name
            // https://stackoverflow.com/questions/35007513/how-to-get-file-path-without-extension-in-rust
            if let Some(table_name) = Path::new(&path).file_stem() {
                if let Some(table_name_str) = table_name.to_str() {
                    self.table_names.push(table_name_str.to_string());
                }
            }
            // create tables based on definition
            self.conn.execute(
                sql_def.as_str(),
                (), // empty list of parameters.
            )?;
        }
        println!("tables created: {:?}", self.table_names);
        Ok(())
    }

    fn load_existing_data(&mut self) -> Result<(), anyhow::Error> {
        /* the assumption is to have <config_dir>/fintrak/data/<table>.data
        available for a give <table> */
        let mut user_data_dir = match dirs::config_dir() {
            Some(user_data_dir) => user_data_dir,
            None => panic!("Unable to load existing data."),
        };
        user_data_dir.extend(&["fintrak", "data"]);
        println!("User data directory: {}", user_data_dir.display());
        // if the directory doesn't exist, create it
        // https://stackoverflow.com/questions/48053933/how-to-check-if-a-directory-exists-and-create-a-new-one-if-it-doesnt
        fs::create_dir_all(user_data_dir.clone()).expect("Unable to load existing data");
        self.config_dir = user_data_dir;
        let valid_row = Regex::new(r"(\s*)(.*): \((.*), (.*)\)?").unwrap();
        // iterate over files in directory, set up the database
        for table_name in self.table_names.iter() {
            let file_str = format!("{}.data", table_name);
            let table_data_file = self.config_dir.join(file_str);
            if table_data_file.exists() {
                println!("Table {} has existing data.", table_name);
                // load the file content
                if let Ok(lines) = read_lines(table_data_file) {
                    // Consumes the iterator, returns an (Optional) String
                    for line in lines.flatten() {
                        let mut row: String = rem_first_and_last(line.as_str()).to_owned();
                        row.push_str(",");
                        let parts = row.split("),");
                        // information about this row
                        let mut cols: Vec<&str> = Vec::new();
                        let mut vals: Vec<&str> = Vec::new();
                        for part in parts {
                            let _ = match valid_row.captures(part) {
                                Some(caps) => {
                                    let col = caps.get(2).unwrap().as_str();
                                    cols.push(col);
                                    let val = caps.get(4).unwrap().as_str();
                                    vals.push(val);
                                }
                                _ => {}
                            };
                        }
                        // insert this line
                        if cols.len() > 0 {
                            let query = format!(
                                "INSERT INTO {} ({}) VALUES ({})",
                                table_name,
                                cols.join(", "),
                                vals.join(", ")
                            );
                            println!("Inserting query: {}", query);
                            self.conn
                                .execute(query.as_str(), ())
                                .expect("failed insert query");
                        }
                    }
                }
            }
        }
        Ok(())
    }

    fn save_existing_data(&self) -> Result<(), anyhow::Error> {
        // the data would be stored in a json format:
        // {col1: [v0, v1, v2, ...], col2: [v0, v1, v2, ...]}
        for table_name in self.table_names.iter() {
            let query = format!("select * from {}", table_name);
            // create file to store data
            let file_str = format!("{}.data", table_name);
            let table_data_file = self.config_dir.join(file_str);
            if table_data_file.exists() {
                // file exists, clean up its content
                fs::remove_file(table_data_file.clone()).expect("Fail to delete existing entries");
                println!("Removing old file: {}", table_data_file.display());
            }
            File::create(table_data_file.clone())
                .expect("unable to create file for storing data of table");

            // https://stackoverflow.com/questions/30684624/what-is-the-best-variant-for-appending-a-new-line-in-a-text-file
            let mut data_file = OpenOptions::new()
                .write(true)
                .append(true)
                .open(table_data_file)
                .unwrap();
            let mut stmt = self.conn.prepare(query.as_str())?;
            // get columns
            //let column_names = stmt.column_names();
            //println!("table {} has columns {:?}", table_name, column_names);
            // get rows
            let mut rows = stmt.raw_query();
            while let Some(row) = rows.next()? {
                let row_str = format!("{:?}\n", row);
                if let Err(e) = writeln!(data_file, "{}", row_str) {
                    eprintln!("Couldn't write to table data file: {}", e);
                }
                println!("row: {:?}", row);
            }
        }
        Ok(())
    }
}

// a helper function for reading lines
// https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

// https://stackoverflow.com/questions/65976432/how-to-remove-first-and-last-character-of-a-string-in-rust
fn rem_first_and_last(value: &str) -> &str {
    let mut chars = value.chars();
    chars.next();
    chars.next_back();
    chars.as_str()
}

fn main() -> Result<()> {
    /*  I assume the following code would be used in backend? */
    let mut db = Database::new();
    // load and run table definitions
    let _ = db.load_table_definitions();
    let _ = db.load_existing_data();

    let mut stmt = db.conn.prepare("SELECT id, name, data FROM person")?;
    let person_iter = stmt.query_map([], |row| {
        Ok(Person {
            id: row.get(0)?,
            name: row.get(1)?,
            data: row.get(2)?,
        })
    })?;

    for person in person_iter {
        println!("Found person {:?}", person.unwrap());
    }

    let me = Person {
        id: 2,
        name: "Marple".to_string(),
        data: 32,
    };
    db.conn.execute(
        "INSERT INTO person (name, data) VALUES (?1, ?2)",
        (&me.name, &me.data),
    )?;

    let mut stmt = db.conn.prepare("SELECT id, name, data FROM person")?;
    let person_iter = stmt.query_map([], |row| {
        Ok(Person {
            id: row.get(0)?,
            name: row.get(1)?,
            data: row.get(2)?,
        })
    })?;

    for person in person_iter {
        println!("Found person {:?}", person.unwrap());
    }
    // save the enires per table
    let _ = db.save_existing_data();
    Ok(())
}
