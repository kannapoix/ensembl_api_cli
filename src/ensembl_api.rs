use std::fs::{File};
use std::io::{Write};

pub fn open_file(path: std::path::PathBuf) -> std::fs::File {
    let file = match File::open(path) {
        Err(_) => panic!("couldn't open file."),
        Ok(file) => file,
    };

    file
}

pub fn failed_record_to_file(file: &mut File, failed_id: &str) {
    file.write_fmt(format_args!("{}\n", failed_id)).unwrap();
}

pub fn get_transcript_sequence_by_id(id: &str, sequence_type: &str) -> Result<reqwest::Response, reqwest::Error> {
    let client = reqwest::Client::new();
    let path = format!("{}{}", "https://rest.ensembl.org/sequence/id/", id);
                
    client.get(path.as_str())
        .header(reqwest::header::CONTENT_TYPE, "text/x-fasta")
        .query(&[("type", sequence_type), ("object_type", &"transcript".to_string())])
        .send()
}

pub fn ensembl_client(path: &str, sequence_type: &str) -> Result<reqwest::Response, reqwest::Error> {
    let client = reqwest::Client::new();
    let path = format!("{}{}", "https://rest.ensembl.org", path);

    client.get(path.as_str())
        .header(reqwest::header::CONTENT_TYPE, "text/x-fasta")
        .query(&[("type", sequence_type), ("object_type", &"transcript".to_string())])
        .send()
}
