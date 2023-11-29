// Closure ili mozemo ga jos nazvati i lambda funkcija 
// Sintaksa za definisanje closure-a je: |parm1, param2,param3| {statements}
// moze da nema parametre: || {}
// ako se sastoji samo od jednog statement-a onda se ne pisu {}

#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    WHITE, BLACK
}

pub fn closure_example(){
    let store = Inventory{
        shirts: vec![ShirtColor::WHITE, ShirtColor::BLACK, ShirtColor::WHITE],
    };

    let user_preference = Some(ShirtColor::BLACK);
    let giveaway = store.giveaway(user_preference);

    println!("The user preference {:?} gets {:?}", user_preference, giveaway);


    let user_preference_1 = None;
    let giveaway_1 = store.giveaway(user_preference_1);
    println!("The user preference {:?} gets {:?}", user_preference_1, giveaway_1);
}


struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory{
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor{
        user_preference.unwrap_or_else(|| self.most_stocked()) //prosledimo closure, ne mozemo funkciju zato sto funkcija iz okruzenja gde bude pozvana
        // ne moze da pristupi promenljivim iz okruzenja u kojem je definisana  
    }


    fn most_stocked(&self) -> ShirtColor {
        let mut num_white = 0;
        let mut num_black = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::WHITE => num_white += 1,
                ShirtColor::BLACK => num_black += 1,
            };
        }

        if num_white > num_black {
            ShirtColor::WHITE
        }else{
            ShirtColor::BLACK
        }

    }
}

