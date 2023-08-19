use std::io::{self, Read, Write};
use std::env;

const JOYOKANJI: &str = include_str!("joyokanji.txt");
const CHUNK_SIZE_BITS: u64 = 44;
const MAX_CHUNK_VALUE: u64 = (1 << CHUNK_SIZE_BITS) - 1;

fn encode_single_block(mut n: u64) -> String {
    let kanji_chars: Vec<char> = JOYOKANJI.chars().collect();
    let mut block = String::new();

    for _ in 0..4 {
        let index = (n % 2136) as usize;
        block.insert(0, kanji_chars[index]);
        n /= 2136;
    }

    block
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

fn encode_integer(n: u64) -> String {
    let mut number = n;
    let mut result = String::new();

    while number > 0 {
        let chunk = number & MAX_CHUNK_VALUE;
        result = encode_single_block(chunk) + &result;
        number >>= CHUNK_SIZE_BITS;
    }

    if result.is_empty() {
        return encode_single_block(0);
    }

    result
}

fn decode_integer(encoded: &str) -> u64 {
    let kanji_vec = encoded.chars().collect::<Vec<_>>();
    let blocks = kanji_vec.chunks(4);
    let mut result = 0u64;

    for block in blocks {
        let block_str: String = block.iter().collect();
        result <<= CHUNK_SIZE_BITS;
        result += decode_block(&block_str);
    }

    result
}

fn encode_bytestream(input: &[u8]) -> String {
    let mut bit_buffer: u64 = 0;
    let mut bit_count: usize = 0;
    let mut output = String::new();

    for byte in input.iter() {
        bit_buffer |= (*byte as u64) << (40 - bit_count);
        bit_count += 8;

        while bit_count >= CHUNK_SIZE_BITS as usize {
            let chunk = (bit_buffer >> (64 - CHUNK_SIZE_BITS)) & MAX_CHUNK_VALUE;
            output.push_str(&encode_single_block(chunk));
            bit_buffer <<= CHUNK_SIZE_BITS;
            bit_count -= CHUNK_SIZE_BITS as usize;
        }
    }

    if bit_count > 0 {
        let chunk = bit_buffer >> (64 - CHUNK_SIZE_BITS);
        output.push_str(&encode_single_block(chunk));
    }

    output
}

fn decode_bytestream(encoded: &str) -> Vec<u8> {
    let mut bit_buffer: u64 = 0;
    let mut bit_count: usize = 0;
    let mut output = Vec::new();

    for block in encoded.chars().collect::<Vec<_>>().chunks(4) {
        let block_str: String = block.iter().collect();
        let decoded = decode_block(&block_str);
        bit_buffer |= decoded << (64 - CHUNK_SIZE_BITS - bit_count as u64);
        bit_count += CHUNK_SIZE_BITS as usize;

        while bit_count >= 8 {
            let byte = (bit_buffer >> (64 - 8)) as u8;
            output.push(byte);
            bit_buffer <<= 8;
            bit_count -= 8;
        }
    }

    output
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
            println!("{}", decode_integer(&input.trim()));
        } else {
            let mut input = String::new();
            io::stdin().read_to_string(&mut input).unwrap();
            let decoded = decode_bytestream(&input);
            io::stdout().write_all(&decoded).unwrap();
        }
    } else {
        if use_integer_mode {
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            let _number: u64 = input.trim().parse().expect("Failed to parse number!");
            println!("{}", encode_integer(_number));
        } else {
            let mut input = Vec::new();
            io::stdin().read_to_end(&mut input).unwrap();
            let encoded = encode_bytestream(&input);
            println!("{}", encoded);
        }
    }
}
