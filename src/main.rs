#![feature(plugin)]
#![plugin(clippy)]

#![allow(ptr_arg)]

mod grasp;
mod grafo;
mod ag;

use std::env;
use std::process;
use std::time::Instant;
use grafo::{Grafo, INF};
use grasp::Grasp;
use ag::Ag;

#[allow(dead_code)]
fn teste_grasp(grafo: &Grafo) {
    println!("Grasp");
    let t = Instant::now();
    let (solucao, it) = Grasp::new(grafo).max_iter(40).timeout(INF).solve();
    let tempo = t.elapsed();

    println!("Caminho: {:?}", solucao.caminho());
    println!("Iteração alvo: {}", it);
    println!("Fo: {}", solucao.fo());
    println!("Tempo: {}.{}", tempo.as_secs(), tempo.subsec_nanos());
    println!("-------------------\n");
}

#[allow(dead_code)]
fn teste_ag(grafo: &Grafo) {
    println!("AG");
    let t = Instant::now();
    let (solucao, it) = Ag::new(grafo)
        .max_iter(100)
        .timeout(INF)
        .mut_chance(0.3)
        .pop_tam(1000)
        .xo_chance(1.0)
        .solve();
    let tempo = t.elapsed();

    println!("Caminho: {:?}", solucao.caminho());
    println!("Iteração alvo: {}", it);
    println!("Fo: {}", solucao.fo());
    println!("Tempo: {}.{}", tempo.as_secs(), tempo.subsec_nanos());
    println!("-------------------\n");
}

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

    // teste_ag(&grafo);
    teste_grasp(&grafo);
}