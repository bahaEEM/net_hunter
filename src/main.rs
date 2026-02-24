use std::env;
use std::io::{self, Write};
use std::net::{IpAddr, TcpStream, ToSocketAddrs};
use std::process;
use std::time::Duration;
use colored::*; // Renkleri açalım

fn main() {
    // EKRANI TEMİZLE
    // print! makrosu satır sonu yapmaz, println! yapar.
    print!("\x1B[2J\x1B[1;1H"); // Terminali temizleyen sihirli kod
    
    println!("{}", "========================================".bright_purple());
    println!("{}", "                NET-HUNTER              ".bold().bright_green());
    println!("{}", "========================================".bright_purple());

    // 2. KULLANICI NE GİRDİ BAKALIM
    let args: Vec<String> = env::args().collect();

    // Eğer IP adresi girmeyi unuttuysa, ona aptalmış gibi davran
    if args.len() < 2 {
        println!("\n{}", "HEY! YOU FORGOT THE IP ADDRESS! 🤦‍♂️".red().bold());
        println!("{}", "I cannot scan the air. Give me a target.".yellow());
        println!("\nUsage like a pro:");
     
        println!("   cargo run 8.8.8.8");
        process::exit(1); // Programı hatayla kapat
    }

    let ip_str = &args[1];

    // IP Adresi geçerli mi?
    let ip: IpAddr = match ip_str.parse() {
        Ok(addr) => addr,
        Err(_) => {
            println!("\n{}", "DUDE... THAT IS NOT AN IP ADDRESS.".red().bold());
            println!("Example: 192.168.1.1 (Numbers and dots only!)");
            process::exit(1);
        }
    };

    println!("\nOkay, target acquired: {}", ip.to_string().cyan().bold());
    println!("{}", "I am knocking on the first 1000 doors now...".italic());
    println!("{}", "(Don't panic if I'm silent, I'm working!)".dimmed());
    println!("----------------------------------------");

    // 3. TARAMA DÖNGÜSÜ
    let ports_to_scan = 1..1024; // İlk 1024 port kritik olanlardır
    let mut open_doors = 0;

    for port in ports_to_scan {
        // Kullanıcıya çalıştığını hissettir 
        // \r satır başına döner, böylece alt alta spam yapmaz
        print!("\rTrying door number: {} 🔨", port);
        io::stdout().flush().unwrap(); // Tamponu boşalt ki ekranda hemen görünsün

       
        let socket_address = format!("{}:{}", ip, port);
        
        // BAĞLANTI DENEMESİ (Timeout: 200 milisaniye)
        // 200ms (0.2 saniye) ideal bir "tıkla-kaç" süresidir.
        let timeout = Duration::from_millis(200);

        // connect_timeout fonksiyonu, Rust'ın "Kapıyı çal, açan yoksa kaç" fonksiyonudur.
        match socket_address.to_socket_addrs() {
            Ok(mut addrs) => {
                if let Some(addr) = addrs.next() {
                    if TcpStream::connect_timeout(&addr, timeout).is_ok() {
                        //Kapı açıldı.
                        print!("\r"); 
                        println!(
                            "{} Door {} is OPEN! --> (Something is running here!)", 
                            "🔓 BINGO!".green().bold(), 
                            port.to_string().yellow()
                        );
                        open_doors += 1;
                    }
                }
            }
            Err(_) => {} // Adres çözülemediyse sessizce geç
        }
    }

    // 4. SONUÇ RAPORU 
    println!("\n{}", "========================================".bright_purple());
    println!("{}", "⚠️  WARNING: Use this tool for educational purposes only!".red());
    println!("{}", "⚠️  UYARI: Bu araç sadece eğitim amaçlıdır!".red());
    println!("{}", "SCAN FINISHED.".bold());
    
    if open_doors > 0 {
        println!(
            "Result: I found {} open doors. This machine is ALIVE.", 
            open_doors.to_string().green().bold()
        );
    } else {
        println!("{}", "Result: Zero open doors found. Is this thing even plugged in?".red());
        println!("{}", "(Or maybe it has a really good firewall...)".dimmed());
    }
}