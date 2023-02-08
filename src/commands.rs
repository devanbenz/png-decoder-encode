use std::{path::PathBuf, fs, str::FromStr};

use clap::Parser;

use crate::{args::{Cli, Commands}, png::Png, chunk::Chunk, chunk_type::ChunkType};

pub fn app() {
    parse_cli();
}

fn parse_cli() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Encode { file, chunk, message, output_file } => {
            encode(file, chunk, message, output_file)
        },
        Commands::Decode { file, chunk } => decode(file, chunk),
        Commands::Remove { file, chunk } => remove(file, chunk),
        Commands::Print { file } => print(file)

    }
}

fn encode(
    file: String, 
    chunk: String, 
    message: String, 
    output_file: Option<String>
    ) {
    let file_raw = read_file(file).expect("could not read file");
    let mut png_from_file = Png::try_from(file_raw.as_ref()).expect("cannot create PNG from file");
    let message_as_bytes: Vec<u8> = message.as_bytes().try_into().expect("could not create bytes from message"); 
    let chunk = Chunk::new(ChunkType::from_str(&chunk).expect(""), message_as_bytes);
    png_from_file.append_chunk(chunk);
    if let Some(file) = output_file {
        let output_file_path = PathBuf::from_str(file.as_str())
            .expect("cannot parse string to path");
        fs::write(output_file_path, png_from_file.as_bytes())
            .expect("cannot write data to file");

    }
    println!("[PNG CREATED] {:?}", png_from_file);
}

fn decode(file: String, chunk: String) {
    let file_raw = read_file(file).expect("could not read file");
    let png_from_file = Png::try_from(file_raw.as_ref()).expect("cannot create PNG from file");
    let chunk_raw = &png_from_file.chunk_by_type(chunk.as_str())
        .expect("cannot parse chunk type")
        .message_bytes;

    println!("{:?}", String::from_utf8(chunk_raw.to_owned()).expect("could not parse string from chunk"));
}

fn remove(file: String, chunk: String) {
    let file_raw = read_file(file.clone()).expect("could not read file");
    let mut png_from_file = Png::try_from(file_raw.as_ref()).expect("cannot create PNG from file");
    match png_from_file.remove_chunk(&chunk) {
        Ok(result) => {
            fs::write(file, png_from_file.as_bytes()).unwrap();
            println!("[REMOVED] {:?}", result);
        },
        Err(err) => println!("{:?}", err)
    }
}

fn print(file: String) {
    let file_raw = read_file(file).expect("could not read file");
    println!("{}", 
             Png::try_from(file_raw.as_ref())
             .expect("cannot create PNG from file")
             );
}

fn read_file(file: String) -> Result<Vec<u8>, std::io::Error> {
    fs::read(PathBuf::from_str(file.as_str())
             .expect("cannot parse path"))
}
