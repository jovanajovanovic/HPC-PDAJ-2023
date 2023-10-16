// Prvi program u rust-u
// Zadatak: Napraviti Rust program koji na konzoli ispisuje "Ovo je nas prvi kod napisan u Rust-u!"

// Rust kod se nalazi u fajlovima sa ekstenzijom .rs i kompajlira se pomocu rustc kompajlera komandom rustc ime_fajla
// Kompajliranjem rust koda dobija se izvrsni fajl koji se pokrece komandom ./naziv_programa, izvrsni program ce imati isto ime kao ime fajla


// main funkcija - uvek treba da postoji ako pravimo izvrsivi program, a ne biblioteku 
fn main(){
    // println! - makro za ispis podataka na konzolu, makroe razlikujemo od funkcija po tome sto se na mestu koriscenja posle imena makroa pise !
    println!("Ovo je nas prvi kod napisan u Rust-u!")
}


