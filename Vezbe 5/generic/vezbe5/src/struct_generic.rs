struct Point<T> {
    x: T,
    y: T
}

struct Point<T, U> {
    x: T, 
    y: U
}

pub fn struct_gen(){
    let point = Point {x: 5, y: 3};

    let point2 = Point {x: 4, y: 3.0}; // ovo nece raditi zato sto T odnosi na jedan tip

    let point3 = Point{x: 4, y: 3.45};

}