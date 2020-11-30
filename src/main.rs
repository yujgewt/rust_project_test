/*
Seed generator
     - get Target File
     - recognize file formats...

*/
use std::env;
use std::fs;

enum Format{
    EXE,
    BIN
}


struct InputFile<'a>{
    name : &'a str,
    format : Format,

}



fn main(){
    
    let args: Vec<String> = env::args().collect();
    println!("{:?}",args);




    let file_name = &args[1];
    println!("file_name : {:?}",file_name);

    let contents = fs::read_to_string(file_name).expect("Something went wrong reading the file.");
    println!("contents : {:?}",contents);
    


    let input = InputFile {name:file_name,format:Format::EXE};

    


}