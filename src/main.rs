use serde_json::Result;
use serde::{Serialize, Deserialize};

//definimos una estructura de tipo Libro
//? si bien en el obj Libro no estan todos los elementos de nuestro json, serde nos permite usarlo de igual manera sin tirar error
//? no vamos a poder imprimir los elementos que nuestro Obj no contenga como atributo
#[derive(Serialize, Deserialize, Debug)]
struct Libro{
    titulo: String,
    total_paginas: u32,
    autores: Vec<String>,
    generos: Vec<String>,
    precios: Vec<Precio>
}
#[derive(Serialize, Deserialize, Debug)]
struct Precio{
    precio: f32,
    tipo: String, 
    moneda: String,
}
fn main() -> Result<()> {
    let libro = r#"{
        "titulo": "The Pragmatic Programmer",
        "autores": [
          "David Thomas",
          "Andrew Hunt"
        ],
        "total_paginas": 352,
        "generos": [
          "programacion",
          "ingenieria",
          "educacion"
        ],
        "precios": [
          {
            "tipo": "digital",
            "precio": 15.00,
            "moneda": "USD"
          },
          {
            "tipo": "tapa dura",
            "precio": 35.50,
            "moneda": "USD"
          }
        ]
    }"#;
    let libro_parsed: Libro = serde_json::from_str(libro)?;

    println!("Libro Porcesado por Rust ");
    println!("Titulo del libro: {}", libro_parsed.titulo);

    const DESCUENTO: f32 = 10.00;

    println!("PRECIOS: ");
    for precio in libro_parsed.precios {
        println!("------------------------------------------------");

        let precio_regular = precio.precio;
        let precio_descuento = precio_regular - (precio_regular * (DESCUENTO / 100.00));

        println!("Tipo: {}", precio.tipo);
        println!("Precio Regular: {} {}", precio.precio, precio.moneda);
        println!("Precio Con Descuento: {} {}", precio_descuento, precio.moneda);
    }
    Ok(())
}
