extern crate rand;

use std::u64;
use std::time::{Duration, Instant};
use self::rand::Rng;
use grafo::{Solucao, Grafo, Caminho, Vertice};

#[allow(dead_code)]
pub fn solve(grafo: &Grafo,
             alfa: f64,
             timeout: Duration,
             num_vizinhos: u32,
             max_iter: u64)
             -> (Solucao, u64) {
    let mut rng = rand::weak_rng();
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

#[allow(dead_code)]
fn vizinho_mais_proximo<R: Rng + Sized>(mut rng: &mut R,
                                        grafo: &Grafo,
                                        alfa: f64)
                                        -> Option<Caminho> {
    let num_vertices = grafo.num_vertices();
    let mut caminho = Vec::with_capacity(num_vertices);
    let mut marcados = vec![false; num_vertices];
    let mut num_marcados = 0;

    let inicial = rng.gen::<Vertice>() % num_vertices;
    caminho.push(inicial);
    marcados[inicial] = true;
    num_marcados += 1;

    while num_marcados < num_vertices {
        let atual = caminho[caminho.len() - 1];
        let mut abertos = grafo.adjacentes(atual)
            .zip(marcados.iter())
            .filter(|&((_, _), marc)| !marc)
            .map(|((vert, &peso), _)| (vert, peso))
            .collect::<Vec<_>>();
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

#[allow(dead_code)]
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

fn two_opt_swap(mut caminho: Caminho, i: Vertice, k: Vertice) -> Caminho {
    caminho[i..k].reverse();
    caminho
}

#[allow(dead_code)]
fn two_opt_loop(grafo: &Grafo, solucao: &Solucao) -> Option<Solucao> {
    let num_vertices = solucao.caminho().len();
    let mut best = solucao.clone();

    for i in 0..num_vertices - 1 {
        for k in i + 1..num_vertices {
            let nova = two_opt_swap(solucao.caminho().clone(), i, k);
            let nova = Solucao::new(grafo, nova);
            if nova.fo() < best.fo() {
                best = nova;
            }
        }
    }

    if best.fo() < solucao.fo() {
        Some(best)
    } else {
        None
    }
}

#[allow(dead_code)]
fn busca_local(grafo: &Grafo, s: Solucao, num_vizinhos: u32) -> Solucao {
    (0..num_vizinhos)
        .map(|_| busca_local_vizinho(grafo, &s))
        .min_by_key(Solucao::fo)
        .unwrap_or(s)
}

pub struct Grasp<'a> {
    grafo: &'a Grafo,
    alfa: f64,
    timeout: u64,
    num_vizinhos: u32,
    max_iter: u64,
}

impl<'a> Grasp<'a> {
    #[allow(dead_code)]
    pub fn new(grafo: &Grafo) -> Grasp {
        Grasp {
            grafo: grafo,
            alfa: 0.35,
            timeout: u64::MAX,
            num_vizinhos: 10,
            max_iter: 40,
        }
    }

    #[allow(dead_code)]
    pub fn alfa(&mut self, alfa: f64) -> &mut Grasp<'a> {
        self.alfa = alfa;
        self
    }

    #[allow(dead_code)]
    pub fn timeout(&mut self, timeout: u64) -> &mut Grasp<'a> {
        self.timeout = timeout;
        self
    }

    #[allow(dead_code)]
    pub fn num_vizinhos(&mut self, num_vizinhos: u32) -> &mut Grasp<'a> {
        self.num_vizinhos = num_vizinhos;
        self
    }

    #[allow(dead_code)]
    pub fn max_iter(&mut self, max_iter: u64) -> &mut Grasp<'a> {
        self.max_iter = max_iter;
        self
    }

    #[allow(dead_code)]
    pub fn solve(&self) -> (Solucao, u64) {
        solve(self.grafo,
              self.alfa,
              Duration::from_secs(self.timeout),
              self.num_vizinhos,
              self.max_iter)
    }
}
