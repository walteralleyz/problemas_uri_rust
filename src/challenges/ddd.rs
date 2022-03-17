use std::io::stdin;
use std::collections::hash_map::HashMap;

pub fn main() {
    let mut ddd_table: HashMap<u8, &str> = HashMap::new();
    ddd_table.insert(61, "Brasilia");
    ddd_table.insert(71, "Salvador");
    ddd_table.insert(11, "Sao Paulo");
    ddd_table.insert(21, "Rio de Janeiro");
    ddd_table.insert(32, "Juiz de Fora");
    ddd_table.insert(19, "Campinas");
    ddd_table.insert(27, "Vitoria");
    ddd_table.insert(31, "Belo Horizonte");
    
    let mut ddd = String::new();
    
    stdin().read_line(&mut ddd).expect("Expect a value!");
    
    let ddd: u8 = ddd.trim().parse().unwrap();
    
    if let Some(res) = ddd_table.get(&ddd) {
        println!("{}", res);
    }
    
    else {
        println!("DDD nao cadastrado");
    }
    
    
}
