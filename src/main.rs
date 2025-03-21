mod args;
mod structs;
mod functions;
use clap::Parser;
use args::Args;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use serde::{Serialize, Deserialize};
use serde_json;
use functions::{ average_base_quality, read_length, average_quality_of_all};
use structs::average_g_c_read;
use flate2::read::GzDecoder;


fn read_fastq(file_path: &str) -> Vec<(String, Vec<u8>)> {
    let file = File::open(file_path).expect("Could not open FASTQ file.");
    let decoder = GzDecoder::new(file); // Decompress the .gz file
    let reader = BufReader::new(decoder);

    let mut reads = Vec::new();
    let mut lines = reader.lines();

    while let Some(Ok(header)) = lines.next() {
        let sequence = lines.next().unwrap().unwrap();
        let _plus = lines.next(); // Skip '+'
        let quality_str = lines.next().unwrap().unwrap();

        // Convert quality scores from ASCII
        let quality_scores: Vec<u8> = quality_str.bytes().collect();
        
        reads.push((sequence, quality_scores));
    }

    reads
}


#[derive(Serialize, Deserialize, Debug)]
struct ReadStats {
    gc_content: (f64, f64),
    length: usize,
    avg_quality: f64,
}

#[derive(Serialize, Deserialize, Debug)]
struct FastqStats {
    reads: Vec<ReadStats>,
    overall_avg_quality: f64,
}

fn write_to_json(output_path: &str, data: &FastqStats) {
    let json = serde_json::to_string_pretty(data).expect("Failed to serialize JSON");
    let mut file = File::create(output_path).expect("Failed to create output file");
    file.write_all(json.as_bytes()).expect("Failed to write JSON");
}

fn main() {
    let args = Args::parse();
    let reads_one = read_fastq(&args.one);
    let mut all_qualities = Vec::new();
    let mut stats = Vec::new();

    for (seq, qual) in &reads_one {
        let gc = average_g_c_read(seq);
        let avg_q = average_base_quality(qual).unwrap_or(0.0);
        let read_len = read_length(qual);

        all_qualities.push(avg_q);
        stats.push(ReadStats { gc_content: gc, length: read_len, avg_quality: avg_q });
    }

    if let Some(pe_file) = args.two {
        let reads_two = read_fastq(&pe_file);

        for (seq, qual) in &reads_two {
            let gc = average_g_c_read(seq);
            let avg_q = average_base_quality(qual).unwrap_or(0.0);
            let read_len = read_length(qual);

            all_qualities.push(avg_q);
            stats.push(ReadStats { gc_content: gc, length: read_len, avg_quality: avg_q });
        }
    }

    let overall_avg_q = average_quality_of_all(all_qualities).unwrap_or(0.0);
    let fastq_stats = FastqStats { reads: stats, overall_avg_quality: overall_avg_q };

    write_to_json("fastq_results.json", &fastq_stats);
    println!("Results saved to fastq_results.json");
}
