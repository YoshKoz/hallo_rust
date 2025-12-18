use std::env;
use std::process::Command;

fn main() {
    // Haal de gebruikersnaam op uit de omgevingsvariabelen
    let user = env::var("USER").unwrap_or_else(|_| String::from("onbekende gebruiker"));

    // Voer het Linux 'uptime' commando uit
    let uptime = Command::new("uptime")
        .arg("-p")
        .output()
        .expect("Kon uptime niet ophalen");

    let uptime_str = String::from_utf8_lossy(&uptime.stdout);

    println!("---------------------------------------");
    println!("Hallo, {}!", user);
    println!("Je Arch WSL systeem is: {}", uptime_str.trim());
    println!("Geprogrammeerd in Neovim!");
    println!("---------------------------------------");
}
