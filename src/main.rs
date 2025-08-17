use clap::Parser;
use pnet::datalink;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]

struct Cli {
    #[arg(short, long)]
    name: String,

    #[arg(short, long, default_value_t = 1)]
    count: u8,
}

fn list_interfaces() {
    println!("=== Interfaces réseau disponibles ===");
    let interfaces = datalink::interfaces();
    
    for (i, interface) in interfaces.iter().enumerate() {
        println!("{}. {}", i + 1, interface.name);
        println!("   Status: {}", if interface.is_up() { "UP" } else { "DOWN" });
        println!("   MAC: {}", interface.mac.unwrap_or_else(|| "N/A".to_string()));
        
        for ip in &interface.ips {
            println!("   IP: {}", ip);
        }
        println!();
    }
}

fn main() {
    let args = Cli::parse();

    if args.name == "interfaces" {
        list_interfaces();
    } else {
        for _ in 0..args.count {
            println!("Welcome, {} !", args.name);
        }
    }
}