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
        /// number of words
        #[arg(short, long, default_value_t = 6)]
        count: usize,
        /// word separator
        #[arg(short, long, default_value = "-")]
        sep: String,
        /// capitalize each word
        #[arg(short = 'C', long)]
        capitalize: bool,
    },
    /// Random character password
    Pass {
        /// password length
        #[arg(short, long, default_value_t = 20)]
        len: usize,
        /// include uppercase
        #[arg(short = 'U', long, default_value_t = true)]
        upper: bool,
        /// include digits
        #[arg(short = 'D', long, default_value_t = true)]
        digits: bool,
        /// include symbols
        #[arg(short = 'S', long, default_value_t = true)]
        symbols: bool,
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
            let phrase = passphrase(&mut rng, count, &sep, capitalize);
            report(&phrase, entropy_bits(WORDS.len(), count));
        }
        Cmd::Pass { len, upper, digits, symbols } => {
            let pool = charset(upper, digits, symbols);
            let pw = password(&mut rng, len, &pool);
            report(&pw, entropy_bits(pool.chars().count(), len));
        }
    }
}
