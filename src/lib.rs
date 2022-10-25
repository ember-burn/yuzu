#![forbid(unsafe_code)]

use std::fs;
use std::path::PathBuf;
use std::collections::HashMap;

/// Represents an instance of a `Yuzu` plaintext, key-value database. 
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Yuzu{
    /// The parsed version of the file.
    pub db: HashMap<String, String>,
    /// Path to the plaintext database storage file.
    pub path: PathBuf,
    /// Separator for the key and the value.
    pub delimiter: String
}

impl Yuzu {
    /// Public builder function to create a new `Yuzu` instance from a file and delimiter.
    pub fn new(path: PathBuf, delimiter: String) -> std::io::Result<Yuzu> {
        let contents = fs::read_to_string(&path)?;
        let db = Yuzu::parse(contents, &delimiter).unwrap();

        Ok(Yuzu {
            db,
            path,
            delimiter
        })
    }

    /// Parse internal HashMap representation into a String and save to the associated path.
    pub fn save(&self) -> std::io::Result<()>{
        fs::write(&self.path, self.parse_to_string())?;
        Ok(())
    }

    /// Retrieve a value from a key.
    pub fn get(&self, key: String) -> Option<&String> {
        self.db.get(&key)
    }

    /// Set a value to a key.
    pub fn set(&mut self, key: String, value: String) -> Option<String> {
        let set = self.db.insert(key, value);
        self.save().unwrap();
        set
    }

    /// Remove a row from the database.
    pub fn remove(&mut self, key: String) -> Option<String> {
        let removed = self.db.remove(&key);
        self.save().unwrap();
        removed
    }

    /// Convert the raw plaintext format to a HashMap. 
    fn parse(raw: String, delimiter: &String) -> Result<HashMap<String, String>, String> {
        let mut db: HashMap<String, String> = HashMap::new();
        let rows: Vec<&str> = raw.lines().collect();

        for row in rows {
            let pair: Vec<&str> = row.split(delimiter).collect();

            if pair.len() != 2 {
                return Err("failed: parser error".to_string());
            }

            db.insert(
                pair[0].to_string(),
                pair[1].to_string()
            );
        }

        Ok(db)
    }

    /// Parse a HashMap to the raw plaintext format.
    fn parse_to_string(&self) -> String {
        let mut parsed = String::new();
        for (key, value) in self.db.clone().into_iter() {
            parsed.push_str(&format!("{}{}{}\n", key, self.delimiter, value));
        }
        parsed
    }
}