use std::fs::File;
use std::io::prelude::*;
use std::io;

fn main() {
  let mut nombre_archivo = String::new();
  println!("Ingrese el nombre del archivo:");
  io::stdin().read_line(&mut nombre_archivo).expect("No se pudo leer el nombre del archivo");
  let nombre_archivo = nombre_archivo.trim();
  let mut  archivo = File::open(nombre_archivo).expect("No se pudo abrir el archivo");
  let mut contenido = String::new();
  archivo.read_to_string(&mut contenido).expect("No se pudo leer el archivo");
  let niveles: Vec<&str> = contenido.split("\n").collect();
  let niveles: Vec<Vec<i32>> = niveles.iter().map(|x| x.split(" ").map(|y| y.parse().unwrap()).collect()).collect();
  let mut niveles_seguros: i32 = 0;
  let mut niveles_no_seguros: Vec<Vec<i32>> = Vec::new();
  println!("Hay un total de: {:?} niveles", niveles.len());
  // Parte 1:
  for nivel in niveles {
    let descenso: bool = if nivel[0] > nivel[1] { true } else { false };
    let mut seguro: bool = true;
    for i in 0..(nivel.len() - 1) {
      if nivel[i] == nivel[i+1] {
        seguro = false;
        break;
      }
      if descenso {
        if nivel[i] < nivel[i+1] || (nivel[i] - nivel[i+1]).abs() > 3 {
          seguro = false;
          break;
        }
      } else {
        if nivel[i] > nivel[i+1] || (nivel[i] - nivel[i+1]).abs() > 3 {
          seguro = false;
          break;
        }
      }
    }
    if seguro {
      niveles_seguros += 1;
    } else {
      niveles_no_seguros.push(nivel);
    }
  }
  println!("Hay {} niveles seguros", niveles_seguros);
  // Parte 2:
  println!("Hay {} niveles no seguros", niveles_no_seguros.len());
  for nivel in niveles_no_seguros {
    let mut seguro: bool = true;
    for i in 0..nivel.len() {
      seguro = true;
      let mut nivel_aux: Vec<i32> = nivel.clone();
      nivel_aux.remove(i);
      let descenso: bool = if nivel_aux[0] > nivel_aux[1] { true } else { false };
      for j in 0..(nivel_aux.len() - 1) {
        if nivel_aux[j] == nivel_aux[j+1] {
          seguro = false;
          break;
        }
        if descenso {
          if nivel_aux[j] < nivel_aux[j+1] || (nivel_aux[j] - nivel_aux[j+1]).abs() > 3 {
            seguro = false;
            break;
          }
        } else {
          if nivel_aux[j] > nivel_aux[j+1] || (nivel_aux[j] - nivel_aux[j+1]).abs() > 3 {
            seguro = false;
            break;
          }
        }
      }
      if seguro {
        println!("El nivel inseguro: {:?} ahora es seguro", nivel);
        break;
      }
    }
    if seguro {
      niveles_seguros += 1;
    }
  }
  println!("Hay {} niveles seguros en el segundo problema", niveles_seguros);
}
