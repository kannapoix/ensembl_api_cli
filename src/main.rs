extern crate clap;
extern crate reqwest;

use std::fs::{self, File, OpenOptions};
use std::io::{Write, BufReader, BufRead};
use std::path::Path;
use clap::{Arg, App};
use std::io::{stdout, BufWriter};

fn main() -> std::io::Result<()> {
    let matches = App::new("ensembl rest client")
        .version("v0.1.0")
        .author("kanna. <kanna@protonmail.ch>")
        .about("Call ensembl API easily")
        .arg(Arg::with_name("directory")
            .short("d")
            .long("directory")
            .value_name("DIRECTORY")
            .help("Path to Directory."))
        .arg(Arg::with_name("id")
            .long("id")
            .value_name("ID")
            .help("Specify the ID."))
        .get_matches();

    if matches.is_present("directory") {
        let directory = Path::new(matches.value_of("directory").unwrap_or("./data"));
    
        fs::create_dir_all("./result")?;
    
        if directory.is_dir() {
            let files_path = loop_files_in_directory(directory.to_str().unwrap().to_string());
        
            for file_path in files_path {
                let file_name = file_path.as_path().file_name().unwrap();
                let file = open_file(file_path.clone());
        
                let f = BufReader::new(file);
    
                let result_target_path = Path::new("./result").join(file_name).with_extension("fasta");
    
                if result_target_path.exists() {
                    println!("{} is exists.", result_target_path.to_str().unwrap());
                    break;
                }
    
                let mut result_file = OpenOptions::new()
                    .create(true)
                    .append(true)
                    .open(result_target_path)?;
        
                for line in f.lines() {
                    let id = line.unwrap();
    
                    let mut response = get_transcript_sequence_by_id(&id).unwrap();
        
                    if response.status().is_success() {
                        let text = response.text().unwrap();
                        result_file.write(text.as_bytes()).unwrap();
                    } else {
                        println!("Error happened. Status: {:?}", response.status());
                        fs::create_dir_all("./failed")?;
                        let mut failed_file = OpenOptions::new()
                            .create(true)
                            .append(true)
                            .open(Path::new("./failed").join(file_name))?;
    
                        failed_record_to_file(&mut failed_file, &id);
                    }
                }        
            }
        }
        }


    if matches.is_present("id") {
        let id = matches.value_of("id").unwrap().to_string();
        let mut response = get_transcript_sequence_by_id(&id).unwrap();

        let stdout = stdout();
        let mut out = BufWriter::new(stdout.lock());
        writeln!(out, "{}", response.text().unwrap()).unwrap();
    }

    Ok(())
}

fn loop_files_in_directory(directory: String) -> Vec<std::path::PathBuf> {
    let mut pathes = Vec::new();
    for entry in fs::read_dir(directory.as_str()).unwrap() {
        let entry = entry.unwrap();
        pathes.push(entry.path());
    }

    pathes
}

fn open_file(path: std::path::PathBuf) -> std::fs::File {
    let file = match File::open(path) {
        Err(_) => panic!("couldn't open file."),
        Ok(file) => file,
    };

    file
}

fn failed_record_to_file(file: &mut File, failed_id: &String) {
    file.write_fmt(format_args!("{}\n", failed_id)).unwrap();
}

fn get_transcript_sequence_by_id(id: &String) -> Result<reqwest::Response, reqwest::Error> {
    let client = reqwest::Client::new();
    let path = format!("{}{}", "https://rest.ensembl.org/sequence/id/", id);
                
    client.get(path.as_str())
        .header(reqwest::header::CONTENT_TYPE, "text/x-fasta")
        .query(&[("object_type", "transcript")])
        .send()
}
