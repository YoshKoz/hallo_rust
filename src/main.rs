use std::io; // Voor input/output
use rand::Rng; // Voor het genereren van getallen
use std::cmp::Ordering; // Voor het vergelijken van getallen

fn main() {
    println!("--- Raad het getal! ---");

    // Genereer een getal tussen 1 en 100
    let geheim_getal = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Type je gok:");

        let mut gok = String::new();

        io::stdin()
            .read_line(&mut gok)
            .expect("Kon regel niet lezen");

        // Zet de tekst om naar een getal, negeer fouten (letters)
        let gok: u32 = match gok.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Je gokte: {}", gok);

        // Vergelijk de gok met het geheime getal
        match gok.cmp(&geheim_getal) {
            Ordering::Less => println!("Te laag!"),
            Ordering::Greater => println!("Te hoog!"),
            Ordering::Equal => {
                println!("GEWONNEN! Goed gedaan.");
                break; // Stop de loop
            }
        }
    }
}
