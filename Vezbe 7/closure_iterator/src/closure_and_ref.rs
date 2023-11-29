pub fn closure(){
    println!("Nepromenljivo pozajmljivanje");
    only_borrows();
    println!("Promenljivo pozajmljivanje");
    borrows_mutably();

}


fn only_borrows(){
    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    let closure_only_borrows = || println!("From closure: {:?}", list);

    println!("After defining closure and before calling closure: {:?}", list);
    closure_only_borrows();
    println!("After calling closure: {:?}", list);
    closure_only_borrows();
}


fn borrows_mutably(){
    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    let mut closure_borrows_mutably = || {
        list.push(7);
        println!("From closure: {:?}", list);
    };

    // println!("After defining closure and before calling closure: {:?}", list); //ne moze ovde da se pozove zato sto je lista prosledjena closure-u i on ce da je menja
    closure_borrows_mutably();
    println!("After calling closure: {:?}", list);
}