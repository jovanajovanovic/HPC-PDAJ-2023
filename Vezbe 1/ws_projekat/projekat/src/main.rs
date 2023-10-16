// Projekat koji se nalazi u istom radnom okviru kao i biblioteka i on moze da poziva funkcija iz biblioteke tako sto:
// 1. biblioteka doda u sekciju dependencies u Cargo.toml fajl
// 2. upotrebom kljucne reci use uveze biblioteka tamo gde hocemo da je koristimo
// 3. pozivamo funkcije iz biblioteke 

use biblioteka;
use rand::prelude::*;

fn main() {
    println!("Hello, world!");
    biblioteka::hello();
    let x: u8 = random();
    println!("Random broj: {}", x);
}
