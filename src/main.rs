mod args;
mod structs;
mod functions;
use clap::Parser;
use args::Args;
use functions::{average_base_quality, read_length, average_quality_of_all, read_fastq, process_sequences, extract_sequences_from_fastq, write_to_json};
use structs::{average_g_c_read, CountBase};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct ReadStats {
    read_num: usize,
    gc_content: f64, // Changed from GCContent struct to f64
    length: usize,
    avg_quality: f64,
}

#[derive(Serialize, Deserialize, Debug)]
struct FastqStats {
    reads: Vec<ReadStats>,
    overall_avg_quality: f64,
}

#[derive(Serialize)]
struct FastqAtPos {
    gc_content: f64, // Changed from GCContent struct to f64
    pos: usize,
}

fn main() {
    let args = Args::parse();
    let reads_one = read_fastq(&args.one);
    let mut all_qualities = vec![];
    let mut stats = vec![];
    let mut result_av_g_c: Vec<f64> = vec![]; // Changed to f64 instead of GCContent
    let mut sequences: Vec<Vec<u8>> = vec![];

    sequences = extract_sequences_from_fastq(&args.one);

    if let Some(ref filename) = args.two {
        println!("Loading sequences from second file: {}", filename);
        sequences.extend(extract_sequences_from_fastq(filename));
    } else {
        println!("No second file provided.");
    }

    result_av_g_c = process_sequences(sequences);

    for (seq, qual) in &reads_one {
        let cou = stats.len() + 1;
        let gc = average_g_c_read(seq); // This now returns a f64
        let avg_q = average_base_quality(qual).unwrap_or(0.0);
        let read_len = read_length(qual);

        all_qualities.push(avg_q);
        stats.push(ReadStats { read_num: cou, gc_content: gc, length: read_len, avg_quality: avg_q });
    }

    if let Some(pe_file) = args.two {
        let reads_two = read_fastq(&pe_file);

        for (seq, qual) in &reads_two {
            let cou = stats.len() + 1;
            let gc = average_g_c_read(seq); // This now returns a f64
            let avg_q = average_base_quality(qual).unwrap_or(0.0);
            let read_len = read_length(qual);

            all_qualities.push(avg_q);
            stats.push(ReadStats { read_num: cou, gc_content: gc, length: read_len, avg_quality: avg_q });
        }
    }

    let overall_avg_q = average_quality_of_all(all_qualities).unwrap_or(0.0);
    let fastq_stats = FastqStats { reads: stats, overall_avg_quality: overall_avg_q };
    write_to_json("fastq_results.json", &fastq_stats);
    write_to_json("test.json", &result_av_g_c);
    println!("Results saved to fastq_results.json");
}