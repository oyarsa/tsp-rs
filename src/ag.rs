extern crate rand;

use std::u64;
use std::time::{Duration, Instant};
use self::rand::{Rng, sample};
use std::cmp::{min, max};
use grafo::{Solucao, Grafo, Caminho, Vertice};
use grafo;

type Populacao = Vec<Solucao>;

#[allow(dead_code)]
pub fn solve(grafo: &Grafo,
             timeout: Duration,
             max_iter: u64,
             pop_tam: usize,
             xo_chance: f64,
             mut_chance: f64)
             -> (Solucao, u64) {
    let mut pop = populacao_inicial(grafo, pop_tam);
    let mut best_fo = pop[0].fo();
    let mut it = 0;
    let mut it_melhor = 0;
    let xo_num = (xo_chance * pop_tam as f64).ceil() as usize;
    let t = Instant::now();

    while it - it_melhor < max_iter && t.elapsed() < timeout {
        let filhos;
        {
            let pais = selecao(&pop, xo_num);
            filhos = recombinacao(grafo, pais, mut_chance);
        }
        pop = proxima_geracao(pop, filhos, pop_tam);

        if pop[0].fo() < best_fo {
            it_melhor = it;
            best_fo = pop[0].fo();
        }
        it += 1;
    }

    (pop.swap_remove(0), it_melhor)
}

#[allow(dead_code)]
fn gen_roleta(pop: &Populacao) -> Vec<f32> {
    let total = pop.iter().map(|s| 1.0 / s.fo() as f32).sum::<f32>();
    pop.iter()
        .scan(0.0, |state, prob| {
            *state += prob.fo() as f32 / total;
            Some(*state)
        })
        .collect()
}

#[allow(dead_code)]
fn get_index_from_roleta(roleta: &[f32]) -> usize {
    let x = rand::thread_rng().next_f32();
    for (i, &prob) in roleta.iter().enumerate() {
        if x <= prob {
            return i;
        }
    }
    0
}


#[allow(dead_code)]
fn selecao(pop: &Populacao, xo_num: usize) -> Vec<(&Solucao, &Solucao)> {
    let roleta = gen_roleta(pop);
    (0..xo_num)
        .map(|_| {
            let pai1 = &pop[get_index_from_roleta(&roleta)];
            let pai2 = &pop[get_index_from_roleta(&roleta)];
            (pai1, pai2)
        })
        .collect()
}

#[allow(dead_code)]
fn proxima_geracao(atual: Populacao, filhos: Populacao, pop_tam: usize) -> Populacao {
    let mut proxima = atual;
    proxima.extend(filhos.into_iter());
    proxima.sort_by_key(Solucao::fo);
    proxima.truncate(pop_tam);
    proxima
}

#[allow(dead_code)]
fn populacao_inicial(grafo: &Grafo, pop_tam: usize) -> Populacao {
    let mut pop = (0..pop_tam).map(|_| individuo_aleatorio(grafo)).collect::<Vec<_>>();
    pop.sort_by_key(Solucao::fo);
    pop
}

#[allow(dead_code)]
fn individuo_aleatorio(grafo: &Grafo) -> Solucao {
    loop {
        if let Some(caminho) = caminho_aleatorio(grafo) {
            return Solucao::new(grafo, caminho);
        }
    }
}

#[allow(dead_code)]
fn caminho_aleatorio(grafo: &Grafo) -> Option<Caminho> {
    let mut rng = rand::thread_rng();
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
        let abertos = grafo.adjacentes(atual)
            .zip(marcados.iter())
            .filter(|&((_, &peso), marc)| !marc && peso != grafo::INF)
            .map(|((vert, _), _)| vert)
            .collect::<Vec<_>>();

        if abertos.is_empty() {
            return None;
        }

        let proximo = abertos[rng.gen::<Vertice>() % abertos.len()];
        caminho.push(proximo);
        marcados[proximo] = true;
        num_marcados += 1;
    }

    Some(caminho)
}

#[allow(dead_code)]
fn two_opt_aleatorio(mut caminho: Caminho) -> Caminho {
    let (i, k) = gen_points(caminho.len());
    caminho[i..k].reverse();
    caminho
}

#[allow(dead_code)]
fn gen_points(num_vertices: usize) -> (Vertice, Vertice) {
    let mut rng = rand::thread_rng();

    let r = sample(&mut rng, 0..num_vertices, 2);
    let i = r[0];
    let j = r[1];

    (min(i, j), max(i, j))
}

#[allow(dead_code)]
fn gen_pmx_points(num_vertices: usize) -> (Vertice, Vertice) {
    let (mut i, mut j) = gen_points(num_vertices);
    if i == 0 {
        i = 1;
    }
    if j == num_vertices - 1 {
        j -= 1;
    }
    (i, j)
}

#[allow(dead_code)]
fn pmx_crossover(grafo: &Grafo, pai1: &Caminho, pai2: &Caminho) -> Solucao {
    let num_vertices = grafo.num_vertices();

    let mut filho = vec![None; num_vertices];
    let mut marcados = vec![false; num_vertices];
    let (xbegin, xend) = gen_pmx_points(num_vertices);

    for i in xbegin..xend {
        filho[i] = Some(pai1[i]);
        marcados[pai1[i]] = true;
    }

    #[allow(needless_range_loop)]
    for i in xbegin..xend {
        let val = pai2[i];

        if marcados[val] {
            continue;
        }

        let mut idx = i;
        while idx >= xbegin && idx <= xend {
            let v = pai1[idx];
            idx = pai2.iter().position(|&x| x == v).unwrap();
            println!("{} - {}: {}/{}", xbegin, xend, v, idx);
            println!("pai1: {:?}", pai1);
            println!("pai2: {:?}", pai2);
        }

        filho[idx] = Some(val);
    }

    for (i, vert) in filho.iter_mut().enumerate() {
        if vert.is_none() {
            *vert = Some(pai2[i]);
        }
    }

    let filho = filho.into_iter().map(|o| o.unwrap()).collect();
    Solucao::new(grafo, filho)
}

#[allow(dead_code)]
fn ordered_crossover(pai1: &Caminho, pai2: &Caminho) -> Caminho {
    let num_vertices = pai1.len();

    let mut filho = vec![None; num_vertices];
    let mut marcados = vec![false; num_vertices];
    let (xbegin, xend) = gen_points(num_vertices);

    // Drop the swath
    for i in xbegin..xend {
        filho[i] = Some(pai1[i]);
        marcados[pai1[i]] = true;
    }

    let mut j = 0;
    let mut i = 0;
    while i < num_vertices {
        if marcados[pai2[i]] {
            i += 1;
        } else if filho[j].is_some() {
            j += 1;
        } else {
            filho[j] = Some(pai2[i]);
            j += 1;
            i += 1;
        }
    }

    filho.into_iter().map(|o| o.unwrap()).collect()
}

#[allow(dead_code)]
fn recombinacao(grafo: &Grafo, pais: Vec<(&Solucao, &Solucao)>, mut_chance: f64) -> Populacao {
    pais.iter()
        .map(|&(pai1, pai2)| ordered_crossover(pai1.caminho(), pai2.caminho()))
        .chain(pais.iter()
            .map(|&(pai2, pai1)| ordered_crossover(pai2.caminho(), pai1.caminho())))
        .map(|c| mutacao(c, mut_chance))
        .map(|c| Solucao::new(grafo, c))
        .collect()
}

#[allow(dead_code)]
fn swap_vertices(mut caminho: Caminho) -> Caminho {
    let (i, j) = gen_points(caminho.len());
    caminho.swap(i, j);
    caminho
}

#[allow(dead_code)]
fn mutacao(caminho: Caminho, mut_chance: f64) -> Caminho {
    if rand::thread_rng().gen::<f64>() < mut_chance {
        // swap_vertices(caminho);
        two_opt_aleatorio(caminho)
    } else {
        caminho
    }
}

pub struct Ag<'a> {
    grafo: &'a Grafo,
    timeout: u64,
    max_iter: u64,
    pop_tam: usize,
    xo_chance: f64,
    mut_chance: f64,
}

impl<'a> Ag<'a> {
    #[allow(dead_code)]
    pub fn new(grafo: &Grafo) -> Ag {
        Ag {
            grafo: grafo,
            timeout: u64::MAX,
            max_iter: 1000,
            pop_tam: 200,
            xo_chance: 0.8,
            mut_chance: 0.1,
        }
    }

    #[allow(dead_code)]
    pub fn solve(&self) -> (Solucao, u64) {
        solve(self.grafo,
              Duration::from_secs(self.timeout),
              self.max_iter,
              self.pop_tam,
              self.xo_chance,
              self.mut_chance)
    }

    #[allow(dead_code)]
    pub fn timeout(mut self, timeout: u64) -> Ag<'a> {
        self.timeout = timeout;
        self
    }

    #[allow(dead_code)]
    pub fn max_iter(mut self, max_iter: u64) -> Ag<'a> {
        self.max_iter = max_iter;
        self
    }

    #[allow(dead_code)]
    pub fn pop_tam(mut self, pop_tam: usize) -> Ag<'a> {
        self.pop_tam = pop_tam;
        self
    }

    #[allow(dead_code)]
    pub fn xo_chance(mut self, xo_chance: f64) -> Ag<'a> {
        self.xo_chance = xo_chance;
        self
    }

    #[allow(dead_code)]
    pub fn mut_chance(mut self, mut_chance: f64) -> Ag<'a> {
        self.mut_chance = mut_chance;
        self
    }
}
