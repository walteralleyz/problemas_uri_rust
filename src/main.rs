mod basico;
mod calcular_area;
mod calculo_simples;
mod cedulas;
mod consumo;
mod diferenca;
mod dois_pontos;
mod distancia;
mod esfera;
mod gasto_combustivel;
mod media_1;
mod media_2;
mod o_maior;
mod produto_simples;
mod salario;
mod salario_com_bonus;
mod soma_simples;

use std::collections::HashMap;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let executor: HashMap<&'static str, fn()> = HashMap::from([
        ("1001", get_as_fn(basico::main)),
        ("1002", get_as_fn(calcular_area::main)),
        ("1003", get_as_fn(soma_simples::main)),
        ("1004", get_as_fn(produto_simples::main)),
        ("1005", get_as_fn(media_1::main)),
        ("1006", get_as_fn(media_2::main)),
        ("1007", get_as_fn(diferenca::main)),
        ("1008", get_as_fn(salario::main)),
        ("1009", get_as_fn(salario_com_bonus::main)),
        ("1010", get_as_fn(calculo_simples::main)),
        ("1011", get_as_fn(esfera::main)),
        ("1013", get_as_fn(o_maior::main)),
        ("1014", get_as_fn(consumo::main)),
        ("1015", get_as_fn(dois_pontos::main)),
        ("1016", get_as_fn(distancia::main)),
        ("1017", get_as_fn(gasto_combustivel::main)),
        ("1018", get_as_fn(cedulas::main))
    ]);

    let challenge: &String = match args.get(1) {
        Some(val) => val,
        None => panic!("Número do desafio esperado!")
    };

    match executor.get(challenge.as_str()) {
        Some(f) => f(),
        None => panic!("Desafio não encontrado!")
    };
}

fn get_as_fn(f: fn() -> ()) -> fn() {
    f as fn()
}