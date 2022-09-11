use std::error::Error;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::str::FromStr;

use crate::args::{DecodeArgs, EncodeArgs, PrintArgs, RemoveArgs};
use crate::png::{chunk::Chunk, chunk_type::ChunkType, png::Png};

pub fn encode(args: EncodeArgs) -> Result<(), Box<dyn Error>> {
    let chunk_type_raw = args.chunk_type;
    let chunk_type = ChunkType::from_str(&chunk_type_raw).unwrap();

    let message_string = args.message;
    let message_data: Vec<u8> = message_string.bytes().collect();

    let chunk = Chunk::new(chunk_type, message_data);

    let path = Path::new(&args.filepath);
    let mut png = Png::from_file(path).unwrap();

    png.append_chunk(chunk);

    if args.output_file.is_some() {
        let mut file = File::create(args.output_file.unwrap())?;
        file.write_all(png.as_bytes().as_slice())?;
    }

    Ok(())
}

// pub fn decode(args: DecodeArgs) -> Result<()> {
//     unimplemented!()
// }
//
// pub fn remove(args: RemoveArgs) -> Result<()> {
//     unimplemented!()
// }
//
// pub fn print(args: PrintArgs) -> Result<()> {
//     unimplemented!()
//
