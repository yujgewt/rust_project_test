/*
Seed generator
     - get Target File
     - recognize file formats...

*/
use goblin::{error, Object};
use std::env;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;
use std::process;

#[derive(Debug)]
enum Format {
    EXE,
    BIN,
}

#[derive(Debug)]
struct InputFile<'a> {
    name: &'a str,
    format: Format,
}

fn check_files(file_name: &str) -> bool {
    return Path::new(file_name).exists();
}

fn parse_file<'a>(buffer: &'a Vec<u8>, obj: &'a mut Object<'a>) -> error::Result<&'a Object<'a>> {
    match Object::parse(buffer)? {
        Object::Elf(elf) => {
            *obj = goblin::Object::Elf(elf);
        }
        Object::PE(pe) => {
            *obj = goblin::Object::PE(pe);
        }
        Object::Mach(mach) => {
            *obj = goblin::Object::Mach(mach);
        }
        Object::Archive(archive) => {
            *obj = goblin::Object::Archive(archive);
        }
        Object::Unknown(magic) => println!("unknown magic: {:#x}", magic),
    }
    println!("archive: {:#?}", obj);

    Ok(obj)
}

fn main() -> std::io::Result<()> {
    let mut obj: Object = goblin::Object::Unknown(0);
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    let file_name = &args[1];
    println!("file_name : {:?}", file_name);

    println!("try to open the file.");
    let mut file = File::open(file_name)?;
    println!("try to read the file.");
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    // buf_reader.read_to_string(&mut contents)?;
    // println!("contents : {:?}",contents);

    let path = Path::new(file_name);
    println!("path: {:#?}", path);
    let buffer: Vec<u8> = fs::read(path)?;

    let result = parse_file(&buffer, &mut obj);
    println!("{:?}", result);

    // let input = InputFile {name:file_name,format:Format::EXE};
    // println!("input : {:?}",input);
    Ok(())

    // if !check_files(file_name){
    //     println!("File")
    //     process::exit(0);
    // }

    // let contents = fs::read_to_string(file_name).expect("Something went wrong reading the file.");
}
