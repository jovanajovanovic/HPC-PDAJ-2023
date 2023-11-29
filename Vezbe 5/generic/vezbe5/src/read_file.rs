use std::{fs::File, io::{BufReader, BufRead, Error}};


pub fn read_line() -> Result<(), Error>{
    let file = File::open("proba.txt").expect("File not found");

   
    let reader = BufReader::new(file);

    for line in reader.lines(){
        println!("Line {}", line?); // line - Option(Some, None), operator upitnik raspakuje ukoliko je Some, a ako je None vrati gresku
    }
    Ok(())
}