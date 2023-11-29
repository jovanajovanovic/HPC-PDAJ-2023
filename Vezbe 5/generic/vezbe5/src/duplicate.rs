// Rukovanje dupliranim kodom 

use core::num;

pub fn duplicate_code(){
    let num_list = vec![23, 45, 6,7, 8, 233, 45];
    let mut largest = &num_list[0];

    for num in &num_list{
        if num > largest{
            largest = num;
        }
    }

    println!("The largest number is {}", largest);

    // ako zelim da pronadjem najveci broj za jos jedan niz mogu da dupliram kod koji smo iznad napisali
    let num_list = vec![223, 45, 64,9, 80, 450, 45];
    let mut largest = &num_list[0];

    for num in &num_list{
        if num > largest{
            largest = num;
        }
    }

    println!("The largest number is {}", largest);

}

pub fn remove_duplicate_code(){
    println!("Without duplicate code");
    // prva opcija za izbegavanje dupliranog koda je da pronadjemo kod koji se duplira i izdvojimo ga u funkciju
    let num_list = vec![23, 45, 6,7, 8, 233, 45];
    let largest = find_larges_num(&num_list);

    println!("The largest number is {}", largest);

    let num_list = vec![223, 45, 64,9, 80, 450, 45];
    let largest = find_larges_num(&num_list);

    println!("The largest number is {}", largest);

}

fn find_larges_num(num_list: &[i32]) -> &i32 {

    let mut largest = &num_list[0];

    for num in num_list{
        if num > largest{
            largest = num;
        }
    }

    largest
}