# TSP-rs

## Implementação de algoritmos para resolução do TSP em Rust

Projeto criado com o fim de exercitar o uso de Rust, tendo como projeto
a resolução do TSP.

### Algoritmos implementados
- GRASP:
    - Construção: vizinho mais próximo semi-guloso
    - Busca local: best-improvement hill climbing com 2-opt
- AG:
    - População inicial: caminhos aleatórios
    - Seleção: roleta simples
    - Cruzamento: PMX e OX
    - Mutação: 2-opt aleatório e swap
    - Próxima geração: elitismo