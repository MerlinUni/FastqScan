pub struct ReadLengthStatistic {
    pub lengths: Vec<usize>,
    pub total_length: usize,
}

impl ReadLengthStatistic {
    pub fn new() -> Self {
        ReadLengthStatistic { lengths: Vec::new(), total_length: 0 }
    }
}

impl Statistic for ReadLengthStatistic {
    fn process(&mut self, record: &FastqRecord) {
        let length = record.seq.len();
        self.lengths.push(length);
        self.total_length += length;
    }
}

impl ReadLengthStatistic {
    pub fn report(&self) {
        if self.lengths.is_empty() {
            println!("No reads processed.");
            return;
        }

        let min_length = *self.lengths.iter().min().unwrap();
        let max_length = *self.lengths.iter().max().unwrap();
        let avg_length = self.lengths.iter().sum::<usize>() as f64 / self.lengths.len() as f64;

        println!("Read Length Statistics:");
        println!("Min length: {}", min_length);
        println!("Max length: {}", max_length);
        println!("Average length: {:.2}", avg_length);
        println!("Total length: {}", self.total_length);
    }
}