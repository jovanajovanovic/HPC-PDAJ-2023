pub fn fn_mut() {
    let mut list = [
        Rectangle {width: 19, height: 8},
        Rectangle {width: 10, height: 6},
        Rectangle {width: 24, height: 6},  
    ];

    list.sort_by_key(|r| r.width);  //prima closure koji je fn_mut zato sto ce da se pozove vise puta, tj onoliko puta koliko stavki ima u listi

    println!("FnMut trait");
    println!("{:#?}", list);


}


pub fn fn_mut_counter(){
    let mut list = [
        Rectangle {width: 19, height: 8},
        Rectangle {width: 10, height: 6},
        Rectangle {width: 24, height: 6},  
    ];
    let mut num_sort_operations = 0;
    let value = String::from("by key called");
    list.sort_by_key(|r| {
        num_sort_operations += 1;
        r.width
    });

    println!("FnMut trait counter");
    println!("{:#?}, sorted in {num_sort_operations} operations", list);
}


#[derive(Debug)]
struct Rectangle {
    width: u32, 
    height: u32,
}