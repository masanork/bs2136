use std::io::{self, Read, Write};
use std::env;

const JOYOKANJI: &str = include_str!("joyokanji.txt");

fn encode_block(mut n: u64) -> String {
    let kanji_chars: Vec<char> = JOYOKANJI.chars().collect();
    let mut result = String::new();

    for _ in 0..4 {
        let index = (n % 2136) as usize;
        result.insert(0, kanji_chars[index]);
        n /= 2136;
    }

    result
}

fn decode_block(kanji_str: &str) -> u64 {
    let kanji_chars: Vec<char> = JOYOKANJI.chars().collect();
    let mut result = 0u64;

    for kanji in kanji_str.chars() {
        let index = kanji_chars.iter().position(|&c| c == kanji).expect("Kanji not found!");
        result = result * 2136 + index as u64;
    }

    result
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.contains(&"-h".to_string()) {
        println!("Usage: kanji_converter [-d] [-i] [-h]");
        println!("-d: Convert from kanji to byte stream/integer");
        println!("-i: Use integer input/output mode");
        println!("-h: Display this help");
        return;
    }

    let use_integer_mode = args.contains(&"-i".to_string());

    if args.contains(&"-d".to_string()) {
        if use_integer_mode {
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            let number: u64 = input.trim().parse().expect("Failed to parse number!");
            println!("{}", encode_block(number));
        } else {
            let mut input = String::new();
            io::stdin().read_to_string(&mut input).unwrap();
            let kanji_vec = input.chars().collect::<Vec<_>>();
            let blocks = kanji_vec.chunks(4);
            let mut output = Vec::new();

            for block in blocks {
                let block_str: String = block.iter().collect();
                let number = decode_block(&block_str);
                output.extend(&number.to_be_bytes());
            }

            io::stdout().write_all(&output).unwrap();
        }
    } else {
        if use_integer_mode {
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            let number: u64 = input.trim().parse().expect("Failed to parse number!");
            println!("{}", encode_block(number));
        } else {
            let mut input = Vec::new();
            io::stdin().read_to_end(&mut input).unwrap();
            let blocks = input.chunks(8);
            let mut output = String::new();

            for block in blocks {
                let mut number = [0u8; 8];
                for (i, byte) in block.iter().enumerate() {
                    number[i] = *byte;
                }
                let number = u64::from_be_bytes(number);
                output.push_str(&encode_block(number));
            }

            println!("{}", output);
        }
    }
}
