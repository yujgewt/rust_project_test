/*
Seed generator
     - get Target File
     - recognize file formats...

*/
use std::env;
use std::fs;
use std::path::Path;
use std::process;

enum Format {
    EXE,
    BIN,
}

struct InputFile<'a> {
    name: &'a str,
    format: Format,
}

fn check_files(file_name:&str)->bool{
    let r = Path::new(file_name).exists();

    if(r==true){
        true
    }else{
        false
    }
    true
}



fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    let file_name = &args[1];
    println!("file_name : {:?}", file_name);

 
    // let contents = fs::read_to_string(file_name).expect("Something went wrong reading the file.");
    // println!("contents : {:?}",contents);

    // let input = InputFile {name:file_name,format:Format::EXE};
}
