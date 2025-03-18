
#[derive(Debug,PartialEq)]
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

    fn from_str(title: &str) -> Option<Self> {
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







    fn explain(&self){
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
            _ => println!("Filterstatus: {} → Unbekannter Wert!", self.filter_status),
        }

        match self.control_bits {
            0 => println!("Kontrollbits: {} → Kein Kontrollbit aktiviert", self.control_bits),
            n if n%2==0 => println!("Kontrollbits: {} → Mindestens ein Kontrollbit aktiviert", self.control_bits),
            _ => println!("Kontrollbits: {} → Unbekannter Wert!", self.control_bits),
        }
    }
}


#[cfg(test)]
    mod tests {
        use crate::struct_read_name::FastqRead;


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
    }