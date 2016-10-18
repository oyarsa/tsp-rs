#![allow(ptr_arg)]

extern crate rand;

use std::f64;
use std::time::{Duration, Instant};
use self::rand::Rng;
use grafo::{Solucao, Grafo, Caminho, Vertice, Peso};

fn vizinho_mais_proximo<R: Rng + Sized>(mut rng: &mut R,
                                        grafo: &Grafo,
                                        alfa: f64)
                                        -> Option<Caminho> {
    let num_vertices = grafo.len();
    let mut caminho = Vec::with_capacity(num_vertices);
    let mut marcados = vec![false; num_vertices];
    let mut num_marcados = 0;

    let inicial = rng.gen::<Vertice>() % num_vertices;
    caminho.push(inicial);
    marcados[inicial] = true;
    num_marcados += 1;

    while num_marcados < num_vertices {
        let atual = caminho[caminho.len() - 1];
        let adjacentes = &grafo[atual];

        let mut abertos = adjacentes.iter()
            .zip(marcados.iter())
            .enumerate()
            .filter(|&(_, (_, marc))| !marc)
            .map(|(vert, (&peso, _))| (vert, peso))
            .collect::<Vec<(Vertice, Peso)>>();
        abertos.sort_by(|&(_, a), &(_, b)| a.cmp(&b));

        let num_candidatos = (abertos.len() as f64 * alfa).ceil() as usize;
        if num_candidatos == 0 {
            return None;
        }

        let (proximo, _) = abertos[rng.gen::<Vertice>() % num_candidatos];
        caminho.push(proximo);
        marcados[proximo] = true;
        num_marcados += 1;
    }

    Some(caminho)
}

fn construcao<R: Rng + Sized>(mut rng: &mut R, grafo: &Grafo, alfa: f64) -> Solucao {
    loop {
        if let Some(caminho) = vizinho_mais_proximo(&mut rng, grafo, alfa) {
            return Solucao::new(grafo, caminho);
        }
    }
}

fn busca_local_vizinho(grafo: &Grafo, solucao: &Solucao) -> Solucao {
    let mut atual = solucao.clone();
    while let Some(nova) = two_opt_loop(grafo, &atual) {
        atual = nova;
    }
    atual
}

fn two_opt_swap(grafo: &Grafo, caminho: &Caminho, i: Vertice, k: Vertice) -> Solucao {
    let mut novo = caminho.clone();
    novo[i..k].reverse();

    Solucao::new(grafo, novo)
}

fn two_opt_loop(grafo: &Grafo, solucao: &Solucao) -> Option<Solucao> {
    let num_vertices = solucao.caminho().len();
    let best = (0..num_vertices - 1)
        .flat_map(|i| {
            (i + 1..num_vertices).map(move |k| two_opt_swap(grafo, solucao.caminho(), i, k))
        })
        .min_by_key(Solucao::fo)
        .unwrap_or_else(Solucao::vazia);
    if best.fo() < solucao.fo() {
        Some(best)
    } else {
        None
    }
}

fn busca_local(grafo: &Grafo, solucao: Solucao, num_vizinhos: u32) -> Solucao {
    (0..num_vizinhos)
        .map(|_| busca_local_vizinho(grafo, &solucao))
        .min_by_key(Solucao::fo)
        .unwrap_or(solucao.clone())
}

pub fn grasp(grafo: &Grafo,
             alfa: f64,
             timeout: u64,
             num_vizinhos: u32,
             max_iter: u64)
             -> (Solucao, u64) {
    let mut rng = rand::thread_rng();
    let timeout = Duration::from_secs(timeout);
    let t = Instant::now();

    let mut it = 0;
    let mut it_alvo = 0;
    let mut best = Solucao::vazia();

    while it - it_alvo < max_iter && t.elapsed() < timeout {
        if it % max_iter == 0 {
            println!("i: {}", it);
        }

        let atual = construcao(&mut rng, grafo, alfa);
        let vizinho = busca_local(grafo, atual, num_vizinhos);

        if vizinho.fo() < best.fo() {
            best = vizinho;
            it_alvo = it;
        }

        it += 1;
    }

    (best, it_alvo)
}
