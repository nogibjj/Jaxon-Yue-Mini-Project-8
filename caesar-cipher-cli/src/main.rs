/*

To run:

cargo run --  --message "Off to the bunker. Every person for themselves" --encrypt --shift 10

To decrypt:

cargo run --  --message "Ypp dy dro lexuob. Ofobi zobcyx pyb drowcovfoc" --decrypt --shift 10

To convert to all caps:

cargo run -- --message "Hello, World!" --caps

*/

use caeser_cipher_cli::{decrypt, encrypt, to_all_caps};
use clap::Parser;

/// CLI tool to encrypt and decrypt messages using the caeser cipher
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Encrypt the message
    #[arg(short, long)]
    encrypt: bool,

    /// decrypt the message
    #[arg(short, long)]
    decrypt: bool,

    /// Convert the message to all caps
    #[arg(short, long)]
    caps: bool,

    /// The message to encrypt, decrypt, or convert to all caps
    #[arg(short, long)]
    message: String,

    /// The shift to use for the cipher
    /// Must be between 1 and 25, the default is 3
    #[arg(short, long, default_value = "3")]
    shift: u8,
}

// run it
fn main() {
    let args = Args::parse();
    if args.encrypt {
        println!("{}", encrypt(&args.message, args.shift));
    } else if args.decrypt {
        println!("{}", decrypt(&args.message, args.shift));
    } else if args.caps {
        println!("{}", to_all_caps(&args.message));
    } else {
        println!("Please specify an action: --encrypt, --decrypt, or --caps");
    }
}
