use std::io::stdin;

pub fn main() {
    let mut food_order = String::new();
    let food_table: [Food; 5] = [
        Food { _food: "Cachorro Quente".to_string(), price: 4. },
        Food { _food: "X-Salada".to_string(), price: 4.5 },
        Food { _food: "X-Bacon".to_string(), price: 5. },
        Food { _food: "Torrada simples".to_string(), price: 2. },
        Food { _food: "Refrigerante".to_string(), price: 1.5 }
    ];
    
    stdin().read_line(&mut food_order).expect("Value expected!");
    
    let food_order: Vec<&str> = food_order.split(" ").collect();
    let opt: usize = food_order.get(0).unwrap().trim().parse().unwrap();
    let qnt: f64 = food_order.get(1).unwrap().trim().parse().unwrap();
    
    println!("Total: R$ {:.2}", food_table[opt-1].price*qnt);
}

struct Food {
    _food: String,
    price: f64
}
