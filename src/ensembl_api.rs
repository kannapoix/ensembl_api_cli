extern crate reqwest;

use std::fs::{File};
use std::io::{stdout, BufWriter, Write};
use reqwest::{RequestBuilder};

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

pub struct EnsemblApi {
    base_url: String,
    client: reqwest::Client,
}

impl EnsemblApi {
    pub fn new() -> EnsemblApi {
        EnsemblApi{
            base_url: "https://rest.ensembl.org".to_string(),
            client: reqwest::Client::new(),
        }
    }

    pub fn get_sequence_id(&self, id: &str, options: Options) {
        let path = format!("{}{}", "/sequence/id/", id);
        match self.get(&path, options) {
            Ok(ref mut response) => {
                let stdout = stdout();
                let mut out = BufWriter::new(stdout.lock());
                writeln!(out, "{}", response.text().unwrap()).unwrap();
            },
            Err(error) => println!("{:?}", error)
        };
    }

    fn get(&self, path: &str, options: Options) -> Result<reqwest::Response, reqwest::Error> {
        let path = format!("{}{}", &self.base_url, path);

        let response_builder = self.client.get(path.as_str());
        self.options(options, response_builder).header(reqwest::header::CONTENT_TYPE, "text/x-fasta").send()
    }

    fn options(&self, options: Options, builder: RequestBuilder) -> RequestBuilder {
        match options.data_type {
            Some(data_type) => match data_type {
                    Type::Genomic => self.data_type("genomic", builder),
                    Type::Cdna => self.data_type("cdna", builder),
                    Type::Cds => self.data_type("cds", builder),
                    Type::Protein => self.data_type("protein", builder),
                },
            None => builder,
        }
    }

    fn data_type(&self, data_type: &str, request_builder: RequestBuilder) -> RequestBuilder {
        request_builder.query(&[("type", data_type), ("object_type", &"transcript".to_string())])
    }
}

pub struct Options {
    pub format: Option<Format>,
    pub data_type: Option<Type>
}

pub enum Format {
    Fasta,
    Full,
    Condensed
}

pub enum Type {
    Genomic,
    Cdna,
    Cds,
    Protein
}

impl From<(&str, &str)> for Options {
    fn from(options: (&str, &str)) -> Options {
        let format = match options.0 {
            "fasta" => Some(Format::Fasta),
            "full" => Some(Format::Full),
            "condensed" => Some(Format::Condensed),
            _ => None,
        };

        let data_type = match options.1 {
            "genomic" => Some(Type::Genomic),
            "cdna" => Some(Type::Cdna),
            "cds" => Some(Type::Cds),
            "protein" => Some(Type::Protein),
            _=> None,
        };

        Options {
            format: format,
            data_type: data_type
        }
    }
}
