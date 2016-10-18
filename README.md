# TSP-rs

## Implementação de algoritmos para resolução do TSP em Rust

Projeto criado com o fim de exercitar o uso de Rust, tendo como projeto
a resolução do TSP.

### Algoritmos implementados
- GRASP:
    - Construção: vizinho mais próximo semi-guloso
    - Busca local: first-improvement hill climbing com 2-opt

### Como usar
O programa recebe um arquivo de texto como parâmetro da linha de comando. Este
arquivo deve ser a matriz de distâncias do grafo, propriamente formatada.
Se nenhum arquivo for fornecido, o algoritmo executa o GRASP em uma pequena
instância de testes.