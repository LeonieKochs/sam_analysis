
#[derive(Debug, Default, Clone, PartialEq, Eq)]

pub struct SamRecord {
    pub qname: String,
    pub flag: u16,
    pub rname: String,
    pub pos: u32,
    pub mapq: u8,
    pub cigar: String,
    pub rnext: String,
    pub pnext: u32,
    pub tlen: i32,
    pub seq: String,
    pub qual: String,
}


impl SamRecord {

    //creates SamRecord from a line
    pub fn from_line(line: &str) -> Option<Self> {
        let parts: Vec<&str> = line.trim().split('\t').collect();
        if parts.len() < 11 {
            return None;
        }

        Some(Self {
            qname: parts[0].to_string(),
            flag: parts[1].parse().ok()?, //might fail, ok() turns a Result into an Option
            rname: parts[2].to_string(),
            pos: parts[3].parse().ok()?,
            mapq: parts[4].parse().ok()?,
            cigar: parts[5].to_string(),
            rnext: parts[6].to_string(),
            pnext: parts[7].parse().ok()?,
            tlen: parts[8].parse().ok()?,
            seq: parts[9].to_string(),
            qual: parts[10].to_string(),
        })
    }
}