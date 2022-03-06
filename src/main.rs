mod challenges;

use std::collections::HashMap;
use std::env;
use crate::challenges::{basico, calcular_area, soma_simples, produto_simples, media_1, media_2,
diferenca, salario, salario_com_bonus, calculo_simples, esfera, o_maior, consumo, dois_pontos,
distancia, gasto_combustivel, cedulas, conversao_de_tempo, idade_em_dias, teste_de_selecao_1,
intervalo, bhaskara};

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
        ("1018", get_as_fn(cedulas::main)),
        ("1019", get_as_fn(conversao_de_tempo::main)),
        ("1020", get_as_fn(idade_em_dias::main)),
        ("1035", get_as_fn(teste_de_selecao_1::main)),
        ("1036", get_as_fn(bhaskara::main)),
        ("1037", get_as_fn(intervalo::main))
    ]);

    println!("{}", if let Some(val) = args.get(1) {
        if let Some(f) = executor.get(val.as_str()) { f(); "Executado!" }
        else { "Problema não encontrado!" }
    }

    else { "Precisa de um argumento para encontrar o problema!" })
}

fn get_as_fn(f: fn() -> ()) -> fn() {
    f as fn()
}