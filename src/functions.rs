use std::vec;

fn average_base_quality(quals: &[u8]) -> Result<f64, &'static str> {
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

fn calculate_phred(qual: u8) -> Result<f64, &'static str> {
    if qual < 33 || qual > 126 {
        return Err("Invalid Phred score");
    }
    Ok((qual - 33) as f64)
}

    
fn read_length(quals: &[u8]) -> usize {
    quals.len()
}	

fn average_quality_of_all(qualities: Vec<f64>) -> Result<f64, &'static str> {
    let n = qualities.len();
    if n == 0 {
        return Err("No reads calculated yet! Please run average_base_quality first on each read.");
    }

    let qual_sum: f64 = qualities.iter().sum(); // Summing all values in the vector
    Ok(qual_sum / n as f64) // Compute average
}

#[cfg(test)]
mod tests {
    use super::*; // Import functions from the outer scope

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