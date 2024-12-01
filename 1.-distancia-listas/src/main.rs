use std::fs::File;
use std::io::prelude::*;

fn main() {
  let mut  archivo = File::open("data/data.txt").expect("No se pudo abrir el archivo");
  let mut contenido = String::new();
  archivo.read_to_string(&mut contenido).expect("No se pudo leer el archivo");
  let lista: Vec<&str> = contenido.split("\n").collect();
  let (lista1,lista2) = separar_listas(lista.clone());
  // Parte 1:
  let dist: i32 = distancia(lista1.clone(), lista2.clone());
  println!("La distancia entre las dos listas es: {}", dist);
  // Parte 2:
  let similitud: i32 = similaridad(lista1.clone(), lista2.clone());
  println!("La similitud entre las dos listas es: {}", similitud);
}

fn separar_listas(lista: Vec<&str>) -> (Vec<i32>, Vec<i32>) {
  let mut lista1: Vec<i32> = Vec::new();
  let mut lista2: Vec<i32> = Vec::new();
  for i in 0..lista.len() {
    let mut linea: Vec<&str> = lista[i].split(" ").collect();
    linea.remove(1);
    linea.remove(1);
    lista1.push(linea[0].parse().unwrap());
    lista2.push(linea[1].parse().unwrap());
  }
  return (lista1, lista2);
}

fn distancia(mut lista1: Vec<i32>, mut lista2: Vec<i32>) -> i32 {
  let mut distancia: i32 = 0;
  for _i in 0..lista1.len() {
    let mut min1: i32 = lista1[0];
    let mut min2: i32 = lista2[0];
    let mut min_index1: usize = 0;
    let mut min_index2: usize = 0;
    for j in 0..lista1.len() {
      if lista1[j] < min1 {
        min1 = lista1[j];
        min_index1 = j;
      }
      if lista2[j] < min2 {
        min2 = lista2[j];
        min_index2 = j;
      }
    }
    distancia += (min1 - min2).abs();
    lista1.remove(min_index1);
    lista2.remove(min_index2);
  }
  return distancia;
}

fn similaridad(lista1: Vec<i32>, lista2: Vec<i32>) -> i32 {
  let mut similaridad: i32 = 0;
  for i in 0..lista1.len() {
    let mut veces_en_la_segunda = 0;
    for j in 0..lista2.len() {
      if lista1[i] == lista2[j] {
        veces_en_la_segunda += 1;
      }
    }
    similaridad += veces_en_la_segunda * lista1[i];
  }
  return similaridad;
}