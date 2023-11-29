
pub fn generic_duplicate(){
    let num_list = vec![223, 45, 64,9, 80, 450, 45];
    let largest = find_larges_num(&num_list);
    println!("The largest number is {}", largest);


    let char_list = vec!['f', 'g', 'd', 'a', 'w'];
    let largest = find_larges_char(&char_list);
    println!("The largest char is {}", largest);

    let char_list = vec!['f', 'g', 'd', 'a', 'w', 'q'];
   // let largest = largest(&char_list);
    println!("The largest char is {}", largest);
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

fn find_larges_char(list: &[char]) -> &char{

    let mut largest = &list[0];

    for num in list{
        if num > largest{
            largest = num;
        }
    }

    largest

}
/*
fn largest<T>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for num in list{
        if num > largest{
            largest = num;
        }
    }

    largest

}
*/
