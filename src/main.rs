extern crate clap;
extern crate reqwest;

mod ensembl_api;

use std::fs::{self, OpenOptions};
use std::io::{Write, BufReader, BufRead};
use std::path::Path;
use clap::{Arg, App, SubCommand};
use std::io::{stdout, BufWriter};

use ensembl_api::*;

fn main() -> std::io::Result<()> {
    let matches = App::new("ensembl api cli")
        .version("v0.1.0")
        .author("kanna. <kanna@protonmail.ch>")
        .about("Call ensembl API easily")
        .subcommand(SubCommand::with_name("sequence")
            .about("Call get sequence endpoint.")
            .subcommand(SubCommand::with_name("id")
                .about("Get sequence by ID.")
                .arg(Arg::with_name("id")
                    .help("Ensembl static ID.")
                    .value_name("ID")
                    )
                .arg(Arg::with_name("type")
                    .long("type")
                    .help("Type of sequence.")
                    .possible_values(&["genomic", "cds", "cdna", "protein"])
                    .takes_value(true)
                    )
                .arg(Arg::with_name("format")
                    .long("format")
                    .help("Format of return data.")
                    .possible_values(&["json", "fasta"])
                    .takes_value(true)
                    )
                .arg(Arg::with_name("directory")
                    .short("d")
                    .long("directory")
                    .value_name("DIRECTORY")
                    .help("To import IDs form files, specify path to directory includes the files.")
                    .takes_value(true)
                    )
                )
            )
        .arg(Arg::with_name("id")
            .long("id")
            .value_name("ID")
            .help("Specify the ID."))
        .get_matches();

    if let Some(sequence_command) = matches.subcommand_matches("sequence") {
        if let Some(id_command) = sequence_command.subcommand_matches("id") {
            let sequence_type = id_command.value_of("type").unwrap_or("cdna").to_string();

            if id_command.is_present("directory") {
                let directory = Path::new(matches.value_of("directory").unwrap_or("./data"));
            
                fs::create_dir_all("./result")?;
            
                if directory.is_dir() {
                    let files_path = loop_files_in_directory(directory.to_str().unwrap());
                
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
            
                            let mut response = get_transcript_sequence_by_id(&id, &sequence_type).unwrap();
                
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
            } else {
                let mut path = "/sequence/id/".to_string();
                let id = id_command.value_of("id").unwrap();
                path.push_str(id);
    
                let mut response = ensembl_client(&path, &sequence_type).unwrap();
    
                let stdout = stdout();
                let mut out = BufWriter::new(stdout.lock());
                writeln!(out, "{}", response.text().unwrap()).unwrap();
            }
        }
    }

    Ok(())
}

fn loop_files_in_directory(directory: &str) -> Vec<std::path::PathBuf> {
    let mut pathes = Vec::new();
    for entry in fs::read_dir(directory).unwrap() {
        let entry = entry.unwrap();
        pathes.push(entry.path());
    }

    pathes
}
