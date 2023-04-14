use flate2::bufread::GzDecoder;
use flate2::write::GzEncoder;
use flate2::Compression;
use std::env::args;
use std::fs::File;
use std::io::Write;
use std::io::copy;
use std::io::BufReader;
use std::time::Instant;

fn main() {
    // println!("Enter <source file> and <destination> ");
    if args().len() < 3 {
        return eprintln!("Usage: <source> <target>");
    }

    let mut input = BufReader::new(File::open(args().nth(1).unwrap()).unwrap());
    let output = File::create(args().nth(2).unwrap()).unwrap();
    let mut encoder = GzEncoder::new(output, Compression::default());
    let start = Instant::now();
    copy(&mut input, &mut encoder).unwrap();
    let output = encoder.finish().unwrap();

    println!("source {:?}", input.get_ref().metadata().unwrap().len());
    println!("target {:?}", output.metadata().unwrap().len());
    println!("elapsed time {:?}", start.elapsed());

    let input = BufReader::new(File::open(args().nth(2).unwrap()).unwrap());
    let mut decoder = GzDecoder::new(input);

    let mut decompressed = File::create("decompressed.txt").unwrap();
    decompressed.write(&mut decoder.get_ref().buffer()).unwrap();
    // println!("decompressed {:?}", decoder.get_ref().buffer());

    // GzDecoder::new(&mut output);
}
