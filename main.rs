use lru::LruCache;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::num::NonZeroUsize;

// Function to convert binary to hexadecimal
fn bin_to_hex(bin: &str) -> String {
    bin.chars()
        .collect::<Vec<char>>()
        .chunks(4)
        .map(|chunk| {
            let s: String = chunk.iter().collect();
            format!("{:X}", u8::from_str_radix(&s, 2).unwrap())
        })
        .collect()
}

// Function to convert hexadecimal to binary
fn hex_to_bin(hex: &str) -> String {
    hex.chars()
        .map(|c| format!("{:04b}", u8::from_str_radix(&c.to_string(), 16).unwrap()))
        .collect()
}

// Function to convert mat.in to mat.in.x with caching
fn convert_to_x(input_file: &str, output_file: &str, cache_size: usize) {
    let input = File::open(input_file).expect("Unable to open input file");
    let output = File::create(output_file).expect("Unable to create output file");
    let reader = BufReader::new(input);
    let mut writer = BufWriter::new(output);

    let cache_size = NonZeroUsize::new(cache_size).expect("Cache size must be greater than zero");
    let mut cache = LruCache::new(cache_size);

    for line in reader.lines() {
        let line = line.unwrap();
        if let Some(cached) = cache.get(&line) {
            writeln!(writer, "{}", cached).unwrap();
        } else {
            let mut parts = line.split(':');
            let dimensions = parts.next().unwrap();
            let binary = parts.next().unwrap();
            let hex = bin_to_hex(binary);
            let compressed = format!("{}:{}", dimensions, hex);
            cache.put(line.clone(), compressed.clone());
            writeln!(writer, "{}", compressed).unwrap();
        }
    }
}

// Function to convert mat.in.x to mat.in with caching
fn convert_to_in(input_file: &str, output_file: &str, cache_size: usize) {
    let input = File::open(input_file).expect("Unable to open input file");
    let output = File::create(output_file).expect("Unable to create output file");
    let reader = BufReader::new(input);
    let mut writer = BufWriter::new(output);

    let cache_size = NonZeroUsize::new(cache_size).expect("Cache size must be greater than zero");
    let mut cache = LruCache::new(cache_size);

    for line in reader.lines() {
        let line = line.unwrap();
        if let Some(cached) = cache.get(&line) {
            writeln!(writer, "{}", cached).unwrap();
        } else {
            let mut parts = line.split(':');
            let dimensions = parts.next().unwrap();
            let hex = parts.next().unwrap();
            let binary = hex_to_bin(hex);
            let decompressed = format!("{}:{}", dimensions, binary);
            cache.put(line.clone(), decompressed.clone());
            writeln!(writer, "{}", decompressed).unwrap();
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <input_file> [cache_size]", args[0]);
        std::process::exit(1);
    }

    let input_file = &args[1];
    let cache_size = args.get(2).and_then(|s| s.parse().ok()).unwrap_or(1000);

    if input_file.ends_with(".x") {
        let output_file = input_file.trim_end_matches(".x");
        convert_to_in(input_file, output_file, cache_size);
    } else {
        let output_file = format!("{}.x", input_file);
        convert_to_x(input_file, &output_file, cache_size);
    }
}
