use std::io::{self, Write};
use clap::{ArgGroup, Parser};

//! # Base64 Encoder/Decoder
//!
//! This program encodes and decodes files using the Base64 algorithm.
//!
//! ## Encoding Algorithm
//!
//! 1. The input is processed in chunks of **3 bytes** (24 bits).
//! 2. Each 24-bit block is split into **4 groups of 6 bits**.
//! 3. Each 6-bit group is mapped to a printable character using the Base64
//!    conversion table:
//!    - `A–Z` → values 0–25
//!    - `a–z` → values 26–51
//!    - `0–9` → values 52–61
//!    - `+`   → 62
//!    - `/`   → 63
//! 4. If the input length isn’t divisible by 3, padding is added:
//!    - 1 leftover byte → output 2 Base64 characters + `==`
//!    - 2 leftover bytes → output 3 Base64 characters + `=`
//!
//! Example: `Man` → `TWFu`
//!
//! ## Decoding Algorithm
//!
//! 1. Input characters are read in **groups of 4** (each representing 6 bits).
//! 2. Each Base64 symbol is converted back into its **6-bit value** using a
//!    reverse lookup table.
//! 3. The 4 groups of 6 bits (24 bits total) are recombined into **3 bytes**.
//! 4. If the input contained padding (`=`), the appropriate number of bytes
//!    (1 or 2) is restored instead of 3.
//!
//! Example: `TWFu` → `Man`
//!
//! ## Notes
//! - Whitespace is ignored in decode mode.
//! - Padding is required for valid Base64 but is handled automatically.
//!
//! This is a simplified reference implementation — no streaming, just in-memory.


#[derive(Parser, Debug)]
#[command(name = "base64", about = "Encode/decode files in Base64", long_about = None)]
#[command(group(
    ArgGroup::new("mode")
        .args(["encode", "decode"])
        .required(true)
))]
struct Cli {
    /// Encode the input
    #[arg(long)]
    encode: bool,

    /// Decode the input
    #[arg(long)]
    decode: bool,

    /// Input file
    input: String,
}

fn encode(bytes: Vec<u8>) -> String {

    // Define convertion table
    let convertion_table: [char; 64] = [
        'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P',
        'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p',
        'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
        '0', '1', '2', '3', '4', '5', '6', '7', '8', '9',
        '+', '/',                                         
    ];

    // Generate buffer for result
    let mut out = String::new();

    for chunk in bytes.chunks(3) {
        match chunk.len() {
            3 => {
                let b0 = chunk[0];
                let b1 = chunk[1];
                let b2 = chunk[2];

                let c1 = b0 >> 2;
                let c2 = ((b0 & 0b00000011) << 4) | (b1 >> 4);
                let c3 = ((b1 & 0b00001111) << 2) | (b2 >> 6);
                let c4 = b2 & 0b00111111;

                out.push(convertion_table[c1 as usize]);
                out.push(convertion_table[c2 as usize]);
                out.push(convertion_table[c3 as usize]);
                out.push(convertion_table[c4 as usize]);
            }
            2 => {
                let b0 = chunk[0];
                let b1 = chunk[1];

                let c1 = b0 >> 2;
                let c2 = ((b0 & 0b00000011) << 4) | (b1 >> 4);
                let c3 = (b1 & 0b00001111) << 2;

                out.push(convertion_table[c1 as usize]);
                out.push(convertion_table[c2 as usize]);
                out.push(convertion_table[c3 as usize]);
                out.push('=');
            }
            1 => {
                let b0 = chunk[0];

                let c1 = b0 >> 2;
                let c2 = (b0 & 0b00000011) << 4;

                out.push(convertion_table[c1 as usize]);
                out.push(convertion_table[c2 as usize]);
                out.push('=');
                out.push('=');
            }
            _ => {}
        }
    }
    out
}

fn decode(chars: Vec<char>) -> Vec<u8> {

    // Define convertion table
    let convertion_table: [char; 64] = [
        'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P',
        'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p',
        'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
        '0', '1', '2', '3', '4', '5', '6', '7', '8', '9',
        '+', '/',                                         
    ];

    // Generate buffer for result
    let mut out = Vec::new();

    // Define reverse convertion table
    let mut reverse_table = [0; 256];
    for (i, c) in convertion_table.iter().enumerate() {
        reverse_table[*c as usize] = i;
    }

    for chunk in chars.chunks(4) {
        match chunk.len() {
            4 => {
                // Get symbols
                let s1 = reverse_table[chunk[0] as usize];
                let s2 = reverse_table[chunk[1] as usize];
                let s3 = reverse_table[chunk[2] as usize];
                let s4 = reverse_table[chunk[3] as usize];

                let b0 = (s1 << 2) | (s2 >> 4);
                let b1 = ((s2 & 0b00001111) << 4) | (s3 >> 2);
                let b2 = ((s3 & 0b00000011) << 6) | s4;

                out.push(b0 as u8);
                out.push(b1 as u8);
                out.push(b2 as u8);

            }
            3 => {
                // Get symbols
                let s1 = reverse_table[chunk[0] as usize];
                let s2 = reverse_table[chunk[1] as usize];
                let s3 = reverse_table[chunk[2] as usize];

                let b0 = (s1 << 2) | (s2 >> 4);
                let b1 = ((s2 & 0b00001111) << 4) | (s3 >> 2);

                out.push(b0 as u8);
                out.push(b1 as u8);
            }
            2 => {
                // Get symbols
                let s1 = reverse_table[chunk[0] as usize];
                let s2 = reverse_table[chunk[1] as usize];

                let b0 = (s1 << 2) | (s2 >> 4);

                out.push(b0 as u8);
            }
            _ => {}
        }
    }

    out
}


fn main() -> std::io::Result<()> {
    let cli = Cli::parse();

    if cli.encode {
        let dat = std::fs::read(&cli.input)?;
        let out = encode(dat);
        println!("{}", out);
    } else {
        let dat = std::fs::read_to_string(&cli.input)?;
        let res = decode(dat.chars()
            .filter(|c| !c.is_whitespace()) 
            .collect());
        // Write to stdout
        let mut out = io::stdout();
        out.write(&res)?;
        out.flush()?;
    }

    Ok(())
}
