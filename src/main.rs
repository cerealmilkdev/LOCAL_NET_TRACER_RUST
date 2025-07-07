    use clap::Parser;

    #[derive(Parser, Debug)]
    #[command(version, about, long_about = None)]
    struct Cli {
        #[arg(short, long)]
        name: String,

        #[arg(short, long, default_value_t = 1)]
        count: u8,
    }

    fn main() {
        let args = Cli::parse();

        for _ in 0..args.count {
            println!("Welcome, {} !", args.name);
        }
    }