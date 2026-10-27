use clap::{Parser, Subcommand};
use passphrase::{
    charset, entropy_bits, passphrase, password, strength, WORDS,
};
use rand::thread_rng;

#[derive(Parser)]
#[command(name = "passphrase", version, about = "Strong passphrases & passwords with entropy")]
struct Cli {
    #[command(subcommand)]
    command: Option<Cmd>,
}

#[derive(Subcommand)]
enum Cmd {
    /// Diceware-style passphrase (default)
    Words {
        /// number of words (1..=64)
        #[arg(short, long, default_value_t = 6, value_parser = clap::value_parser!(u32).range(1..=64))]
        count: u32,
        /// word separator
        #[arg(short, long, default_value = "-")]
        sep: String,
        /// capitalize each word
        #[arg(short = 'C', long)]
        capitalize: bool,
    },
    /// Random character password
    Pass {
        /// password length (1..=1024)
        #[arg(short, long, default_value_t = 20, value_parser = clap::value_parser!(u32).range(1..=1024))]
        len: u32,
        /// exclude uppercase letters
        #[arg(long)]
        no_upper: bool,
        /// exclude digits
        #[arg(long)]
        no_digits: bool,
        /// exclude symbols
        #[arg(long)]
        no_symbols: bool,
    },
}

fn report(secret: &str, bits: f64) {
    println!("{secret}");
    eprintln!("  ~{:.0} bits of entropy ({})", bits, strength(bits));
}

fn main() {
    let cli = Cli::parse();
    let mut rng = thread_rng();

    match cli.command.unwrap_or(Cmd::Words { count: 6, sep: "-".into(), capitalize: false }) {
        Cmd::Words { count, sep, capitalize } => {
            let count = count as usize;
            let phrase = passphrase(&mut rng, count, &sep, capitalize);
            report(&phrase, entropy_bits(WORDS.len(), count));
        }
        Cmd::Pass { len, no_upper, no_digits, no_symbols } => {
            let len = len as usize;
            let pool = charset(!no_upper, !no_digits, !no_symbols);
            let pw = password(&mut rng, len, &pool);
            report(&pw, entropy_bits(pool.chars().count(), len));
        }
    }
}
