use std::vec;
use serde::de::value::U32Deserializer;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use serde::{Serialize, Deserialize};
use serde_json;
use flate2::read::GzDecoder;
use crate::structs::CountBase;

pub fn average_base_quality(quals: &[u8]) -> Result<f64, &'static str> {
    let n = quals.len();
    if n == 0 {
        return Err("Quality string is empty");
    }

    let mut qual_sum: f64 = 0.0;
    for &qual in quals {
        qual_sum += calculate_phred(qual)?; // Propagate error if invalid
    }

    Ok(qual_sum / n as f64)
}

pub fn calculate_phred(qual: u8) -> Result<f64, &'static str> {
    if qual < 33 || qual > 126 {
        return Err("Invalid Phred score");
    }
    Ok((qual - 33) as f64)
}

    
pub fn read_length(quals: &[u8]) -> usize {
    quals.len()
}	

pub fn average_quality_of_all(qualities: Vec<f64>) -> Result<f64, &'static str> {
    let n = qualities.len();
    if n == 0 {
        return Err("No reads calculated yet! Please run average_base_quality first on each read.");
    }

    let qual_sum: f64 = qualities.iter().sum(); // Summing all values in the vector
    Ok(qual_sum / n as f64) // Compute average
}



pub fn read_fastq(file_path: &str) -> Vec<(String, Vec<u8>)> {
    let file = File::open(file_path).expect("Could not open FASTQ file.");
    
    // Prüfen, ob es sich um eine .gz-Datei handelt
    let reader: Box<dyn BufRead> = if file_path.ends_with(".gz") {
        let decoder = GzDecoder::new(file);
        Box::new(BufReader::new(decoder))
    } else {
        Box::new(BufReader::new(file))
    };

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

pub fn process_sequences(sequences: Vec<Vec<u8>>) -> Vec<f64> {
    let max_length = sequences.iter().map(|seq| seq.len()).max().unwrap_or(0);
    let mut result_av_g_c = Vec::new();

    for pos in 0..max_length {
        let mut count_base = CountBase::new(pos as u64);

        for seq in &sequences {
            if let Some(&base) = seq.get(pos) {
                count_base.count(base);
            }
        }

        result_av_g_c.push(count_base.average_gc_at_pos());
    }

    result_av_g_c
}

pub fn extract_sequences_from_fastq(filename: &str) -> Vec<Vec<u8>> {
    let file = File::open(filename).expect("Error opening file");

    // Check if the file is gzip compressed
    let reader: Box<dyn BufRead> = if filename.ends_with(".gz") {
        let decoder = GzDecoder::new(file);
        Box::new(BufReader::new(decoder))
    } else {
        Box::new(BufReader::new(file))
    };

    let mut sequences = Vec::new();
    let mut lines = reader.lines();

    while let Some(Ok(_)) = lines.next() { // Skip header (@SEQ_ID)
        if let Some(Ok(seq)) = lines.next() { // Sequence line
            sequences.push(seq.into_bytes());
        }
        lines.next(); // Skip '+'
        lines.next(); // Skip quality score line
    }

    sequences
}


pub fn write_to_json<T: Serialize>(output_path: &str, data: &T) {
    let json = serde_json::to_string_pretty(data).expect("Failed to serialize JSON");
    let mut file = File::create(output_path).expect("Failed to create output file");
    file.write_all(json.as_bytes()).expect("Failed to write JSON");
}







#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::{Write, BufWriter};
    use serde_json;

    #[test]
    fn test_calculate_phred() {
        assert_eq!(calculate_phred(b'!').unwrap(), 0.0);
        assert_eq!(calculate_phred(b'I').unwrap(), 40.0);
        assert_eq!(calculate_phred(b'J').unwrap(), 41.0);
        assert_eq!(calculate_phred(b'~').unwrap(), 93.0);
    }

    #[test]
    fn test_average_base_quality() {
        let quality_str = b"IIIIIIIIII"; // All 'I' (Phred score 40)
        assert_eq!(average_base_quality(quality_str).unwrap(), 40.0);

        let quality_str = b"JJJJJJJJJJ"; // All 'J' (Phred score 41)
        assert_eq!(average_base_quality(quality_str).unwrap(), 41.0);

        let quality_str = b"~~~~~~~~~~"; // All '~' (Phred score 93)
        assert_eq!(average_base_quality(quality_str).unwrap(), 93.0);

        let empty_str: &[u8] = b"";
        assert!(average_base_quality(empty_str).is_err()); // Should return an error
    }


    #[test]
    fn test_average_quality_of_all() {
        let qualities = vec![40.0, 41.0, 93.0];
        assert_eq!(average_quality_of_all(qualities).unwrap(), 58.0);

        let empty_vec: Vec<f64> = vec![];
        assert!(average_quality_of_all(empty_vec).is_err()); // Should return an error
    }

    #[test]
    fn test_read_length() {
        let quality_str = b"IIIIIIIIII"; // Length 10
        assert_eq!(read_length(quality_str), 10);

        let quality_str = b"JJJJJJJJJJ"; // Length 10
        assert_eq!(read_length(quality_str), 10);

        let quality_str = b"~~~~~~~~~~"; // Length 10
        assert_eq!(read_length(quality_str), 10);

        let empty_str: &[u8] = b"";
        assert_eq!(read_length(empty_str), 0); // Should return 0
    }


    

}