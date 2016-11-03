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
use std::cell::RefCell;
use std::collections::HashMap;

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
        .max_iter(INF)
        .timeout(5)
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

#[allow(dead_code)]
fn bfs_run() {
    println!("Digite o tamanho da matriz, seguido por ela: ");
    let g = Grafo::from_stdin();
    let dist = grafo::bfs_distancia(&g, 3);
    println!("{:?}", dist);
}

struct Professor(String);

#[derive(PartialEq, Eq, Hash)]
struct Disciplina(String);

struct ProfessorDisciplina<'a> {
    professor: &'a Professor,
    disciplina: &'a Disciplina,
}

impl<'a> ProfessorDisciplina<'a> {
    fn new(p: &'a Professor, d: &'a Disciplina) -> ProfessorDisciplina<'a> {
        ProfessorDisciplina {
            professor: p,
            disciplina: d,
        }
    }

    fn set_professor(&mut self, p: &'a Professor) {
        self.professor = p;
    }
}

fn teste() {
    let p1 = Professor("p1".to_string());
    let p2 = Professor("p2".to_string());

    let d1 = Disciplina("d1".to_string());
    let d2 = Disciplina("d2".to_string());
    let d3 = Disciplina("d3".to_string());

    let mut h = HashMap::new();
    h.insert(&d1, RefCell::new(ProfessorDisciplina::new(&p1, &d1)));
    h.insert(&d2, RefCell::new(ProfessorDisciplina::new(&p1, &d2)));
    h.insert(&d3, RefCell::new(ProfessorDisciplina::new(&p2, &d3)));

    let mut v = Vec::new();
    v.push(&h[&d1]);
    v.push(&h[&d1]);
    v.push(&h[&d2]);
    v.push(&h[&d3]);
    v.push(&h[&d3]);
    v.push(&h[&d2]);

    let mut x = h[&d1].borrow_mut();
    x.set_professor(&p2);

}

fn main() {
    let args: Vec<String> = env::args().collect();

    let grafo: Grafo = match args.len() {
        1 => Grafo::toy(),
        2 => Grafo::from_arquivo(&args[1]),
        _ => {
            println!("Opções inválidas");
            process::exit(1);
        }
    };

    teste_ag(&grafo);
    // teste_grasp(&grafo);
    // bfs_run();
    // teste();
}