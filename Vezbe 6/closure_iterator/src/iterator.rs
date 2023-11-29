pub fn iterator(){
    let v1 = vec![1, 2, 3, 4, 5, 6];

    let  v1_iter = v1.iter(); //definisan iterator 

    // println!("Next element is {:?}", v1_iter.next());


    for val in v1_iter {  //for petlja preuzme vlasnistvo nad v1_iter i napravi ga promenljivim
        println!("Got: {}", val);
    }




}

pub fn adapters() {
    let v1 = vec![1, 2, 3, 4, 5, 6];
 
    let v1_iter = v1.iter();
    let total: i32 = v1_iter.sum(); // sum preuzima vlasnistvo tako da ne moze v1_iter vise da se koristi

    println!("Result is {}", total);
}

pub fn method_adapters() {
    let v1 = vec![1, 2, 3, 4, 5, 6];
    // v1.iter().map(|x| x+1); //ovaj kod ne radi nista, svaki iterator koji se napravi mora i da se iskoristi

    let v2: Vec<_> = v1.iter().map(|x| x+1).collect();
    println!("After map function: {:?}", v2);



}