
use std::ops::{Index, IndexMut};
use std::io::Result;
use std::fs::{read_dir, read_to_string};
use std::ffi::OsStr;

const DATA_FILETYPE_EXTENSION: &str = "mtga";
const DATA_TYPES: [&str; 8] = [
    "cards",
    "loc",
    "enums",
    "abilities",
    "prompts",
    "altArtCredits",
    "altPrintings",
    "altFlavorTexts"
];



#[derive(Debug, Default)]
pub struct ClientData {
    data_dir: std::path::PathBuf,
    pub cards: String,
    pub localization: String,
    pub enums: String,
    pub abilities: String,
    pub prompts: String,
    pub alt_art_credits: String,
    pub alt_printings: String,
    pub alt_flavor_texts: String
}

impl ClientData {
    pub fn new(data_dir: std::path::PathBuf) -> ClientData {
        ClientData {
            data_dir,
            ..Default::default()
        }
    }
}

impl Index<&'_ str> for ClientData {
    type Output = String;
    fn index(&self, s: &str) -> &String {
        match s {
            "cards" => &self.cards,
            "loc" => &self.localization,
            "enums" => &self.enums,
            "abilities" => &self.abilities,
            "prompts" => &self.prompts,
            "altArtCredits" => &self.alt_art_credits,
            "altPrintings" => &self.alt_printings,
            "altFlavorTexts" => &self.alt_flavor_texts,
            _ => panic!("unknown field: {}", s),
        }
    }
}

impl IndexMut<&'_ str> for ClientData {
    fn index_mut(&mut self, s: &str) -> &mut String {
        match s {
            "cards" => &mut self.cards,
            "loc" => &mut self.localization,
            "enums" => &mut self.enums,
            "abilities" => &mut self.abilities,
            "prompts" => &mut self.prompts,
            "altArtCredits" => &mut self.alt_art_credits,
            "altPrintings" => &mut self.alt_printings,
            "altFlavorTexts" => &mut self.alt_flavor_texts,
            _ => panic!("unknown field: {}", s),
        }
    }
}

impl ClientData {
    pub fn collect(&mut self) -> Result<&mut ClientData> {

        // Parse directory and read filenames
        // for entry in read_dir(dir)?.into_iter().filter_map(|e| e.ok()) {
        for entry in read_dir(&self.data_dir)?
            .into_iter()
            .filter_map(|e| e.ok())
            {

                let path = entry.path();
                let is_file = entry.metadata().unwrap().is_file();
                let is_data_file = is_file && path.extension() == Some(OsStr::new(DATA_FILETYPE_EXTENSION));

                if is_data_file {
                    let filename = entry.file_name();
                    let filename_string = filename.to_string_lossy();
                    let tokens: Vec<&str> = filename_string.split("_").collect();
                    let data_type = tokens[1];

                    println!("Reading from {} file.", tokens[1]);

                    // Store data in memory
                    if DATA_TYPES.contains(&data_type) {
                        let data = read_to_string(&path).expect("Error while reading the file.");
                        self[&data_type] = data;
                    }
                }
            }

        Ok(self)

    }
}
