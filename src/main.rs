use std::io::{self, Write};

use clap::{ArgGroup, Parser};

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
