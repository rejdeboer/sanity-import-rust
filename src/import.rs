extern crate sanity;
use std::error::Error;
use std::fs::File;
use std::path::Path;
use std::io::BufReader;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Document {
    _id: String,
    _type: String,
}

pub fn import_data<P: AsRef<Path>>(file_path: &P, _client: sanity::SanityConfig) {
    let data = get_file_data(file_path);
    for document in data.unwrap() {
        println!("{}", document._id);
    }
}


fn get_file_data<P: AsRef<Path>>(file_path: &P) -> Result<Vec<Document>, Box<dyn Error>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let data = serde_json::from_reader(reader)?;

    Ok(data)
}
