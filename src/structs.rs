
#[derive(Debug,PartialEq,Clone)]
pub struct FastqRead{
    device_id: String,
    run_id: u16,
    flowcell_id: String,
    lane: u8,
    tile: u16,
    x_coord: u32,
    y_coord: u32,
    read_number: u8,
    filter_status: char,
    control_bits: u8,
    index_seq: String,
}


impl FastqRead {

    pub fn from_str(title: &str) -> Option<Self> { //move to from_str 
    let parts: Vec<&str> = title.split(|c| c == ':' || c == ' ').collect();

    if parts.len() < 11 {
        println!("Fehler: Die Eingabe enthält nicht genügend Felder!");
        return None;
    }

    Some(Self {
        device_id: parts[0].to_string(),
        run_id: parts[1].parse().ok()?,   // Convert to u16
        flowcell_id: parts[2].to_string(),
        lane: parts[3].parse().ok()?,     // Convert to u8
        tile: parts[4].parse().ok()?,     // Convert to u16
        x_coord: parts[5].parse().ok()?,  // Convert to u32
        y_coord: parts[6].parse().ok()?,  // Convert to u32
        read_number: parts[7].parse().ok()?,  // Convert to u8
        filter_status: parts[8].chars().next()?, // Convert single char
        control_bits: parts[9].parse().ok()?,   // Convert to u8
        index_seq: parts[10].to_string(),
    })
    }


   



    pub fn explain(&self){
        println!("Geräte-ID: {} → Die eindeutige Bezeichnung des Sequenziergeräts", self.device_id);
        println!("Lauf-ID: {} → Dies ist das {}. Mal, dass dieses Gerät betrieben wurde", self.run_id, self.run_id);
        println!("Flowcell-ID: {} → Die eindeutige ID der verwendeten Flowcell", self.flowcell_id);
        println!("Flowcell-Lane: {} → Die Lane der Flowcell (1–8)", self.lane);
        println!("Tile-Nummer: {} → Die Kachel (Tile) innerhalb der Lane", self.tile);
        println!("X-Koordinate: {} → Die X-Koordinate des Clusters auf dem Tile", self.x_coord);
        println!("Y-Koordinate: {} → Die Y-Koordinate des Clusters auf dem Tile", self.y_coord);
        
        println!("Read-Nummer: {} → 1 = Vorwärts-Read, 2 = Rückwärts-Read für Paired-End-Sequenzierung", self.read_number);
        
        match self.filter_status {
            'Y' => println!("Filterstatus: {} → Read hat den \"Chastity\"-Filter verletzt", self.filter_status),
            'N' => println!("Filterstatus: {} → Read hat den \"Chastity\"-Filter nicht verletzt", self.filter_status),
            _ => println!("Filterstatus: {} → Unbekannter Wert!", self.filter_status),// panic / fail
        }

        match self.control_bits {
            0 => println!("Kontrollbits: {} → Kein Kontrollbit aktiviert", self.control_bits),
            n if n%2==0 => println!("Kontrollbits: {} → Mindestens ein Kontrollbit aktiviert", self.control_bits),
            _ => println!("Kontrollbits: {} → Unbekannter Wert!", self.control_bits),// fail here too
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct CountBase {
    pub position: u64,
    pub A: u32,
    pub C: u32,
    pub G: u32,
    pub T: u32,
    pub N: u32,
}

impl CountBase {
    pub fn new(position: u64) -> Self {
        Self {
            position,
            A: 0,
            C: 0,
            G: 0,
            T: 0,
            N: 0,
        }
    }

    pub fn count(&mut self, base: u8) {
        match base {
            b'A' => self.A += 1,
            b'C' => self.C += 1,
            b'G' => self.G += 1,
            b'T' => self.T += 1,
            b'N' => self.N += 1,
            _ => (),
        }
    }

    pub fn average_gc_at_pos(&self) -> f64 {
        let total = (self.A + self.C + self.G + self.T) as f64;
        if total == 0.0 {
            return 0.0; // Avoid division by zero
        }
        (self.G + self.C) as f64 * 100.0 / total // Return GC content as a percentage
    }

    pub fn base_percentage(&self) -> (f64, f64, f64, f64, f64) {
        let total = (self.A + self.C + self.G + self.T + self.N) as f64;
        if total == 0.0 {
            return (0.0, 0.0, 0.0, 0.0, 0.0);
        }
        (
            self.A as f64 * 100.0 / total,
            self.C as f64 * 100.0 / total,
            self.G as f64 * 100.0 / total,
            self.T as f64 * 100.0 / total,
            self.N as f64 * 100.0 / total,
        )
    }
}


pub fn average_g_c_read(s: &str) -> f64 { // Returns GC content as f64
    let mut count = CountBase::new(0);

    for &c in s.as_bytes() {
        count.count(c);
    }

    count.average_gc_at_pos() // Now returns f64 directly
}


#[cfg(test)]
    mod tests {
        use crate::structs::FastqRead;
        use super::*;


    #[test]
    fn test_from_str() {
        let title = "HWI-ST1234:123:C4Y5UACXX:8:1101:1234:5678 1:N:0:ACAGTG";
        let fastq_title = FastqRead::from_str(title).unwrap();

        assert_eq!(fastq_title.device_id, "HWI-ST1234");
        assert_eq!(fastq_title.run_id, 123);
        assert_eq!(fastq_title.flowcell_id, "C4Y5UACXX");
        assert_eq!(fastq_title.lane, 8);
        assert_eq!(fastq_title.tile, 1101);
        assert_eq!(fastq_title.x_coord, 1234);
        assert_eq!(fastq_title.y_coord, 5678);
        assert_eq!(fastq_title.read_number, 1);
        assert_eq!(fastq_title.filter_status, 'N');
        assert_eq!(fastq_title.control_bits, 0);
        assert_eq!(fastq_title.index_seq, "ACAGTG");
    }


#[test]
    fn test_explain() {
    let title = "HWI-ST1234:123:C4Y5UACXX:8:1101:1234:5678 1:N:0:ACAGTG";
    let fastq_title = FastqRead::from_str(title).unwrap();

    fastq_title.explain();
}

#[test]
    fn test_control_bits_even() {
    let title = "HWI-ST1234:123:C4Y5UACXX:8:1101:1234:5678 1:N:2:ACAGTG";
    let fastq_title = FastqRead::from_str(title).unwrap();

    assert_eq!(fastq_title.control_bits, 2);
    fastq_title.explain();
    //missing assert_eq!()
}

#[test]
    fn test_control_bits_odd() {
    let title = "HWI-ST1234:123:C4Y5UACXX:8:1101:1234:5678 1:N:3:ACAGTG";
    let fastq_title = FastqRead::from_str(title).unwrap();

    assert_eq!(fastq_title.control_bits, 3);
    fastq_title.explain();
}

#[test]
    fn test_unvalid_input() {
    let title = "HWI-ST1234:123:C4Y5UACXX:8:1101:1234:5678 1:N:ACAGTG";
    let fastq_title = FastqRead::from_str(title);

    assert_eq!(fastq_title, None);
    }

    #[test]
    fn test_new() {
        let count = CountBase::new(10);
        assert_eq!(count.position, 10);
        assert_eq!(count.A, 0);
        assert_eq!(count.C, 0);
        assert_eq!(count.G, 0);
        assert_eq!(count.T, 0);
        assert_eq!(count.N, 0);
    }

    #[test]
    fn test_count() {
        let mut count = CountBase::new(5);//shorter tests
        count.count(b'A');
        count.count(b'A');
        count.count(b'C');
        count.count(b'G');
        count.count(b'T');
        count.count(b'N');

        assert_eq!(count.A, 2);
        assert_eq!(count.C, 1);
        assert_eq!(count.G, 1);
        assert_eq!(count.T, 1);
        assert_eq!(count.N, 1);
    }

    #[test]
    fn test_base_percentage() {
        let mut count = CountBase::new(2);
        count.count(b'A');
        count.count(b'A');
        count.count(b'C');
        count.count(b'C');
        count.count(b'G');
        count.count(b'T');
        count.count(b'T');
        count.count(b'N');

        let (a, c, g, t, n) = count.base_percentage();

        assert_eq!(a, 25.0);
        assert_eq!(c, 25.0);
        assert_eq!(g, 12.5);
        assert_eq!(t, 25.0);
        assert_eq!(n, 12.5);
    }

    #[test]
    fn test_amino_percentage_zero_division() {
        let count = CountBase::new(3);
        let (a, c, g, t, n) = count.base_percentage();
        assert_eq!((a, c, g, t, n), (0.0, 0.0, 0.0, 0.0, 0.0));
    }

  
}