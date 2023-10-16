// Ova biblioteka nece raditi nista specijalno, samo demonstrira kako se kreira biblioteka u radnom okviru

//! # Nasa prva biblioteka (sa # pravimo sekciju)
//! biblioteka za sada ima dve funkcije, jedna koja ispisuje pozdrav na konzolu i druga koja sabira dva broja

/// Hello - ispisuje pozdrav na konzolu
pub fn hello(){
    println!("Zdravo iz biblioteke!")
}


// Ovo je podrzumevan kod koji se dobije kada kreiramo biblioteku, ostavicu ga ovde, necu ga brisati
/// Add - sabira dva broja
/// 
/// # Examples
/// sekcija sa primerom koda kako da se pozove funkcija, ovde je dodat i test koji ce se izvrsiti kada se uradi komanda za generisanje dokumentacije
/// ```
/// let l = 5;
/// let r = 3;
/// let answer = biblioteka::add(l, r)
/// 
/// assert_eq!(8, answer);
/// ```
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
