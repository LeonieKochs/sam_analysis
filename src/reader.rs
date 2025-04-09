use std::io::{self, BufRead};
use crate::record::SamRecord;

/// Parses a single line of a SAM file (non-header) into a SamRecord.
/// filters headers
fn parse_line(line: &str) -> Option<SamRecord> {
    if line.starts_with('@') || line.trim().is_empty() {
        return None;
    }
    SamRecord::from_line(line)
}

/// Reads a full SAM file from a buffered reader, returning all parsed records.
pub fn parse_records<R: BufRead>(reader: R) -> io::Result<Vec<SamRecord>> {
    let mut records = Vec::new();

    for line_result in reader.lines() {
        let line = line_result?;
        if let Some(record) = parse_line(&line) {
            records.push(record);
        }
    }

    Ok(records)
}


#[cfg(test)]
mod test {
    use std::io::BufReader;

    use crate::record::SamRecord;
    use crate::reader::parse_records;

    #[test]

    fn test_parse_records (){

        let sam_data = "\
        @HD\tVN:1.0\tSO:unsorted
        r001\t99\tchr1\t7\t30\t8M2I4M1D3M\t=\t37\t39\tACGTACGTACGT\tIIIIIIIIIIII
        r002\t0\tchr1\t9\t20\t5M\t*\t0\t0\tGGCCG\tIIIII
        r003\t16\tchr1\t12\t15\t4M\t*\t0\t0\tATGC\tIIII
        ";

        // Use a BufReader around the string
        let reader = BufReader::new(sam_data.as_bytes());

        // Parse records from the mock SAM data
        let result = parse_records(reader).expect("Failed to parse records");
        
        let expected = vec![
            SamRecord {
                qname: "r001".to_string(),
                flag: 99,
                rname: "chr1".to_string(),
                pos: 7,
                mapq: 30,
                cigar: "8M2I4M1D3M".to_string(),
                rnext: "=".to_string(),
                pnext: 37,
                tlen: 39,
                seq: "ACGTACGTACGT".to_string(),
                qual: "IIIIIIIIIIII".to_string(),
            },
            SamRecord {
                qname: "r002".to_string(),
                flag: 0,
                rname: "chr1".to_string(),
                pos: 9,
                mapq: 20,
                cigar: "5M".to_string(),
                rnext: "*".to_string(),
                pnext: 0,
                tlen: 0,
                seq: "GGCCG".to_string(),
                qual: "IIIII".to_string(),
            },
            SamRecord {
                qname: "r003".to_string(),
                flag: 16,
                rname: "chr1".to_string(),
                pos: 12,
                mapq: 15,
                cigar: "4M".to_string(),
                rnext: "*".to_string(),
                pnext: 0,
                tlen: 0,
                seq: "ATGC".to_string(),
                qual: "IIII".to_string(),
            },
        ];
        assert_eq!(result, expected);


    }

}