extern crate rand;

use std::u64;
use std::time::{Duration, Instant};
use self::rand::Rng;
use grafo::{Solucao, Grafo, Caminho, Vertice};

pub struct Grasp<'a> {
    grafo: &'a Grafo,
    alfa: f64,
    timeout: Duration,
    num_vizinhos: u32,
    max_iter: u64,
}

impl<'a> Grasp<'a> {
    #[allow(dead_code)]
    pub fn solve(&self) -> (Solucao, u64) {
        let mut rng = rand::weak_rng();
        let t = Instant::now();

        let mut it = 0;
        let mut it_alvo = 0;
        let mut best = Solucao::vazia();

        while it - it_alvo < self.max_iter && t.elapsed() < self.timeout {
            if it % self.max_iter == 0 {
                println!("i: {}", it);
            }

            let atual = self.construcao(&mut rng);
            let vizinho = self.busca_local(atual);

            if vizinho.fo() < best.fo() {
                best = vizinho;
                it_alvo = it;
            }

            it += 1;
        }

        (best, it_alvo)
    }

    #[allow(dead_code)]
    fn vizinho_mais_proximo<R: Rng + Sized>(&self, mut rng: &mut R) -> Option<Caminho> {
        let num_vertices = self.grafo.num_vertices();
        let mut caminho = Vec::with_capacity(num_vertices);
        let mut marcados = vec![false; num_vertices];
        let mut num_marcados = 0;

        let inicial = rng.gen::<Vertice>() % num_vertices;
        caminho.push(inicial);
        marcados[inicial] = true;
        num_marcados += 1;

        while num_marcados < num_vertices {
            let atual = caminho[caminho.len() - 1];
            let mut abertos = self.grafo
                .adjacentes(atual)
                .zip(marcados.iter())
                .filter(|&((_, _), marc)| !marc)
                .map(|((vert, &peso), _)| (vert, peso))
                .collect::<Vec<_>>();
            abertos.sort_by(|&(_, a), &(_, b)| a.cmp(&b));

            let num_candidatos = (abertos.len() as f64 * self.alfa).ceil() as usize;
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
    fn construcao<R: Rng + Sized>(&self, mut rng: &mut R) -> Solucao {
        loop {
            if let Some(caminho) = self.vizinho_mais_proximo(&mut rng) {
                return Solucao::new(self.grafo, caminho);
            }
        }
    }
    fn busca_local_vizinho(&self, solucao: &Solucao) -> Solucao {
        let mut atual = solucao.clone();
        while let Some(nova) = self.two_opt_loop(&atual) {
            atual = nova;
        }
        atual
    }

    fn two_opt_swap(&self, mut caminho: Caminho, i: Vertice, k: Vertice) -> Solucao {
        caminho[i..k].reverse();
        Solucao::new(self.grafo, caminho)
    }

    #[allow(dead_code)]
    fn two_opt_loop(&self, solucao: &Solucao) -> Option<Solucao> {
        let num_vertices = solucao.caminho().len();
        let mut best = solucao.clone();

        for i in 0..num_vertices - 1 {
            for k in i + 1..num_vertices {
                let nova = self.two_opt_swap(solucao.caminho().clone(), i, k);
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
    fn busca_local(&self, s: Solucao) -> Solucao {
        (0..self.num_vizinhos)
            .map(|_| self.busca_local_vizinho(&s))
            .min_by_key(Solucao::fo)
            .unwrap_or(s)
    }
}

pub struct GraspConfig<'a> {
    grafo: &'a Grafo,
    alfa: f64,
    timeout: u64,
    num_vizinhos: u32,
    max_iter: u64,
}

impl<'a> GraspConfig<'a> {
    #[allow(dead_code)]
    pub fn new(grafo: &Grafo) -> GraspConfig {
        GraspConfig {
            grafo: grafo,
            alfa: 0.35,
            timeout: u64::MAX,
            num_vizinhos: 10,
            max_iter: 40,
        }
    }

    #[allow(dead_code)]
    pub fn alfa(&'a mut self, alfa: f64) -> &mut GraspConfig {
        self.alfa = alfa;
        self
    }

    #[allow(dead_code)]
    pub fn timeout(&'a mut self, timeout: u64) -> &mut GraspConfig {
        self.timeout = timeout;
        self
    }

    #[allow(dead_code)]
    pub fn num_vizinhos(&'a mut self, num_vizinhos: u32) -> &mut GraspConfig {
        self.num_vizinhos = num_vizinhos;
        self
    }

    #[allow(dead_code)]
    pub fn max_iter(&'a mut self, max_iter: u64) -> &mut GraspConfig {
        self.max_iter = max_iter;
        self
    }

    #[allow(dead_code)]
    pub fn build(&self) -> Grasp<'a> {
        Grasp {
            grafo: self.grafo,
            alfa: self.alfa,
            timeout: Duration::from_secs(self.timeout),
            num_vizinhos: self.num_vizinhos,
            max_iter: self.max_iter,
        }
    }
}
