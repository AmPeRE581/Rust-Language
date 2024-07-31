use std::io;
use std::f64;

fn main() {
    println!("Programma di calcoli matematici in Rust");

    // Leggere i numeri dall'utente
    let mut input1 = String::new();
    let mut input2 = String::new();

    println!("Inserisci il primo numero:");
    io::stdin().read_line(&mut input1).expect("Errore nella lettura del numero");
    let num1: f64 = input1.trim().parse().expect("Inserisci un numero valido");

    println!("Inserisci il secondo numero:");
    io::stdin().read_line(&mut input2).expect("Errore nella lettura del numero");
    let num2: f64 = input2.trim().parse().expect("Inserisci un numero valido");

    // Addizione
    let somma = num1 + num2;
    println!("La somma di {} e {} è: {}", num1, num2, somma);

    // Sottrazione
    let differenza = num1 - num2;
    println!("La differenza tra {} e {} è: {}", num1, num2, differenza);

    // Moltiplicazione
    let prodotto = num1 * num2;
    println!("Il prodotto di {} e {} è: {}", num1, num2, prodotto);

    // Divisione
    if num2 != 0.0 {
        let quoziente = num1 / num2;
        println!("Il quoziente di {} diviso {} è: {}", num1, num2, quoziente);
    } else {
        println!("Errore: divisione per zero");
    }

    // Radice quadrata del primo numero
    if num1 >= 0.0 {
        let radice_quadrata = num1.sqrt();
        println!("La radice quadrata di {} è: {}", num1, radice_quadrata);
    } else {
        println!("Errore: impossibile calcolare la radice quadrata di un numero negativo");
    }
}
