mod challenges;

use std::collections::HashMap;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let executor: HashMap<&str, fn()> = HashMap::from([
    	("hack", get_as_fn(challenges::hacker_rank::main)),
        ("1001", get_as_fn(challenges::basico::main)),
        ("1002", get_as_fn(challenges::calcular_area::main)),
        ("1003", get_as_fn(challenges::soma_simples::main)),
        ("1004", get_as_fn(challenges::produto_simples::main)),
        ("1005", get_as_fn(challenges::media_1::main)),
        ("1006", get_as_fn(challenges::media_2::main)),
        ("1007", get_as_fn(challenges::diferenca::main)),
        ("1008", get_as_fn(challenges::salario::main)),
        ("1009", get_as_fn(challenges::salario_com_bonus::main)),
        ("1010", get_as_fn(challenges::calculo_simples::main)),
        ("1011", get_as_fn(challenges::esfera::main)),
        ("1013", get_as_fn(challenges::o_maior::main)),
        ("1014", get_as_fn(challenges::consumo::main)),
        ("1015", get_as_fn(challenges::dois_pontos::main)),
        ("1016", get_as_fn(challenges::distancia::main)),
        ("1017", get_as_fn(challenges::gasto_combustivel::main)),
        ("1018", get_as_fn(challenges::cedulas::main)),
        ("1019", get_as_fn(challenges::conversao_de_tempo::main)),
        ("1020", get_as_fn(challenges::idade_em_dias::main)),
        ("1035", get_as_fn(challenges::teste_de_selecao_1::main)),
        ("1036", get_as_fn(challenges::bhaskara::main)),
        ("1037", get_as_fn(challenges::intervalo::main)),
        ("1038", get_as_fn(challenges::lanche::main)),
        ("1042", get_as_fn(challenges::sort_simples::main)),
        ("1050", get_as_fn(challenges::ddd::main)),
        ("1051", get_as_fn(challenges::imposto_de_renda::main)),
        ("1052", get_as_fn(challenges::mes::main)),
        ("1059", get_as_fn(challenges::numeros_pares::main)),
        ("1060", get_as_fn(challenges::numeros_positivos::main))
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
