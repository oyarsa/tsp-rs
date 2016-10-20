use std::io::{BufRead, BufReader};
use std::io;
// use std::io::prelude::*;
use std::path::Path;
use std::fs::File;
use std::iter::Enumerate;
use std::slice::Iter;

pub const INF: u64 = 1e9 as u64;

pub type Peso = u64;
pub type Vertice = usize;
// pub type Grafo = Vec<Vec<Peso>>;
pub type Caminho = Vec<Vertice>;

#[derive(Clone)]
#[derive(Debug)]
pub struct Grafo(Vec<Vec<Peso>>);

impl Grafo {
    pub fn num_vertices(&self) -> usize {
        self.0.len()
    }

    pub fn adjacentes(&self, vertice: Vertice) -> Enumerate<Iter<u64>> {
        self.0[vertice].iter().enumerate()
    }

    pub fn distancia(&self, src: Vertice, dst: Vertice) -> Peso {
        self.0[src][dst]
    }

    #[cfg_attr(rustfmt, rustfmt_skip)]
    pub fn toy() -> Grafo {
        Grafo(vec![
            vec![0, 1, 4, 2],
            vec![1, 0, 2, 5],
            vec![4, 2, 0, 3],
            vec![2, 5, 3, 0]
        ])
    }

    #[allow(dead_code)]
    pub fn from_stdin() -> Grafo {
        let stdin = io::stdin();
        let mut stdin = stdin.lock();
        let mut buf = String::new();

        stdin.read_line(&mut buf).expect("Unable to read number of vertexes");
        let n = buf.trim().parse().unwrap_or(0);

        let x: Vec<Vec<u64>> = stdin.lines()
            .take(n)
            .map(|l| {
                l.expect("Failed to read line")
                    .split_whitespace()
                    .take(n)
                    .map(|n| n.parse().unwrap_or(0))
                    .collect()
            })
            .collect();

        Grafo(x)
    }

    pub fn from_arquivo(file: &str) -> Grafo {
        let path = Path::new(file);
        let file = BufReader::new(File::open(&path).expect("Failed to open file"));

        Grafo(file.lines()
            .map(|l| {
                l.expect("Failed to read line")
                    .split_whitespace()
                    .map(|number| number.parse().unwrap_or(INF))
                    .collect()
            })
            .collect())
    }
}


#[derive(Clone)]
pub struct Solucao {
    caminho: Caminho,
    fo: Peso,
}

fn is_factivel(c: &Caminho, num_vertices: usize) -> bool {
    c.len() == num_vertices && frequencias(c).iter().all(|&n| n == 1)
}

fn frequencias(caminho: &Caminho) -> Vec<u64> {
    let mut freq = vec![0; caminho.len()];
    for &vert in caminho {
        freq[vert] += 1;
    }
    freq
}

impl Solucao {
    fn calcula_fo(grafo: &Grafo, caminho: &Caminho) -> Peso {
        if !is_factivel(caminho, grafo.num_vertices()) {
            return INF;
        }
        let inicio = caminho[0];
        let fim = caminho[caminho.len() - 1];
        caminho.iter()
            .zip(&caminho[1..])
            .map(|(&src, &dst)| grafo.distancia(src, dst))
            .sum::<Peso>() + grafo.distancia(fim, inicio)
    }

    pub fn new(grafo: &Grafo, caminho: Caminho) -> Solucao {
        Solucao {
            fo: Solucao::calcula_fo(grafo, &caminho),
            caminho: caminho,
        }
    }

    #[allow(dead_code)]
    pub fn vazia() -> Solucao {
        Solucao {
            fo: INF,
            caminho: vec![],
        }
    }

    pub fn caminho(&self) -> &Caminho {
        &self.caminho
    }

    pub fn fo(&self) -> Peso {
        self.fo
    }
}

#[allow(dead_code)]
#[allow(needless_range_loop)]
fn perm2inv(perm: &Caminho) -> Caminho {
    let mut inv = vec![0; perm.len()];
    for i in 0..perm.len() {
        let mut m = 0;
        while perm[m] != i {
            if perm[m] > i {
                inv[i] += 1;
            }
            m += 1;
        }
    }

    inv
}

#[allow(dead_code)]
#[allow(needless_range_loop)]
fn inv2perm(inv: &Caminho) -> Caminho {
    let n = inv.len();
    let mut perm = vec![0; n];
    let mut pos = vec![0; n];

    println!("i {:?}", inv);

    for i in (0..n).rev() {
        for m in i + 1..n {
            if pos[m] >= inv[i] {
                pos[m] += 1;
            }
            pos[i] = inv[i];
        }
    }

    println!("hm");
    println!("{:?}", pos);
    for i in 0..n {
        perm[pos[i]] = i;
    }
    perm
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fo_correta() {
        let g = Grafo::toy();
        let c = vec![0, 1, 2, 3];
        assert_eq!(Solucao::calcula_fo(&g, &c), 8);
    }

    #[test]
    fn fo_infactivel_vertice_repetido() {
        let g = Grafo::toy();
        let c = vec![0, 1, 2, 1];
        assert_eq!(Solucao::calcula_fo(&g, &c), INF);
    }

    #[test]
    fn fo_infactivel_vertice_faltando() {
        let g = Grafo::toy();
        let c = vec![0, 1, 2];
        assert_eq!(Solucao::calcula_fo(&g, &c), INF);
    }
}
