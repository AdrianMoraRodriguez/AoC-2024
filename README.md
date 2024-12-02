# AoC-2024
## Adrián Mora Rodríguez
### Objetivos
- [ ] Aprender a usar `rust` para resolver problemas de programación.
- [ ] Resolver los problemas de [Advent of Code 2024](https://adventofcode.com/2024).
- [ ] Aprender a usar cargo de manera eficiente. 
### Día 1: Distancias y similitudes entre dos listas
- He aprendido a leer ficheros, a usar `unwrap` y a usar `collect`.
- He finailizado las dos partes del problema.
- En la primera parte tuve un error porque eliminaba elementos de la lista mientras la recorría y accedía, la soluión fue no acceder al elemento i-ésimo sino al primer elemento como mínimo inicial.
- En la segunda parte tuve el error de hacer un break dónde no debía, quitando el break se solucionó.
### Día 2: Niveles seguros y el problema de Dampener
- He aprendido que no se puede volver a utilizar un vector después de haber iterado sobre él con un for element in vector.
- La línea ```let niveles: Vec<Vec<i32>> = niveles.iter().map(|x| x.split(" ").map(|y| y.parse().unwrap()).collect()).collect();``` es muy útil para parsear un vector de vectores de ints a partir de un vector de strings.