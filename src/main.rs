#![feature(plugin)]
#![plugin(clippy)]

mod grasp;
mod grafo;

use std::env;
use std::process;
use std::time::Instant;
use std::u64;
use grafo::Grafo;
use grasp::grasp;

fn main() {
    let args: Vec<String> = env::args().collect();

    let grafo: Grafo = match args.len() {
        1 => grafo::toy(),
        2 => grafo::grafo_from_arquivo(&args[1]),
        _ => {
            println!("Opções inválidas");
            process::exit(1);
        }
    };

    let t = Instant::now();
    let (solucao, it) = grasp(&grafo, 0.35, u64::MAX, 10, 40);
    let tempo = t.elapsed();

    println!("Caminho: {:?}", solucao.caminho());
    println!("Iteração alvo: {}", it);
    println!("Fo: {}", solucao.fo());
    println!("Tempo: {}.{}", tempo.as_secs(), tempo.subsec_nanos());
}
