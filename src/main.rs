/*
Seed generator
     - get Target File
     - recognize file formats...

*/
use std::env;
use std::fs;
use std::path::Path;
use std::process;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use goblin::{error, Object};

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



fn run (file_name:&str) -> error::Result<()> {
            let path = Path::new(file_name);
            println!("path: {:#?}", path);
            let buffer = fs::read(path)?;
            match Object::parse(&buffer)? {
                Object::Elf(elf) => {
                    println!("elf: {:#?}", &elf);
                },
                Object::PE(pe) => {
                    println!("pe: {:#?}", &pe);
                },
                Object::Mach(mach) => {
                    println!("mach: {:#?}", &mach);
                },
                Object::Archive(archive) => {
                    println!("archive: {:#?}", &archive);
                },
                Object::Unknown(magic) => { println!("unknown magic: {:#x}", magic) }
            }
  
    Ok(())
}



fn main() -> std::io::Result<()> {
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

    run(file_name);   
 
 





    // let input = InputFile {name:file_name,format:Format::EXE};
    // println!("input : {:?}",input);
    Ok(())

    // if !check_files(file_name){
    //     println!("File")
    //     process::exit(0);
    // }


    // let contents = fs::read_to_string(file_name).expect("Something went wrong reading the file.");
    

    
}
