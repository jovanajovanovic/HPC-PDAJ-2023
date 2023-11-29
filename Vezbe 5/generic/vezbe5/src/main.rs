mod duplicate;
mod generic;
mod read_file;
mod trait_struct;
fn main() {
    duplicate::duplicate_code();
    println!("=======================================================");
    duplicate::remove_duplicate_code();


    println!("=======================================================");
    println!("------------------GENERICS-----------------------------");
    // genericnost 
    generic::generic_duplicate();

    println!("=======================================================");
    println!("------------------FILE-----------------------------");
    read_file::read_line();

    println!("=======================================================");
    println!("------------------TRAIT-----------------------------");
        //trait
    trait_struct::trait_struct();

}
