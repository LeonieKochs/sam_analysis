use crate::record::SamRecord;
use crate::statistic::Statistic;
// Berechnen Sie den Anteil der Reads mit einer Mindestqualität von Q30

pub struct Q30{
    total_count: u64,
    q30_count: u64,
}

impl Q30{
    pub fn new()-> Self { 
        Self {total_count: 0, q30_count: 0} }
    /* 
    pub fn update(&mut self, mapq:u8) {
        if mapq >= 30 {
            self.q30_count += 1;
        }
        self.total_count += 1;
    } */

    pub fn get_ratio(&self) -> f64 {
        if self.total_count > 0 {
            (self.q30_count as f64 / self.total_count as f64) * 100.0
        } else {
            0.0
        }
    }
}

impl  Statistic for Q30 {

    fn process (&mut self, record: &SamRecord) {
        if record.mapq >= 30 {
            self.q30_count +=1;
        }
        self.total_count +=1;
    }

    fn name(&self) -> &str {
        "Q30 Percentage"
    }

    fn report(&self) -> String {
        format!("{:.2}%", self.get_ratio()) // returns String, shows 2 decimal places
    }
    
}


#[cfg(test)]
mod tests {

    use crate::{record::{self, SamRecord}, statistic::Statistic};

    use super::Q30;

    fn fake_record(mapq: u8) -> SamRecord {
        SamRecord {
            mapq,
            ..Default::default() // Fills in the rest of the fields with default values
        }
    }

    #[test]
    fn test_q30_ratio() {
        let mut q30 = Q30::new();
        let qualities = vec![30, 20, 15];

        for &q in &qualities {
            let record = fake_record(q);
            q30.process(&record);
        }

        let expected = (1.0 / 3.0) * 100.0;
        let result = q30.get_ratio();

        assert_eq!(expected, result);
    }

    #[test]
    fn test_empty_mapping_quality() {
        let q30 = Q30::new();
        assert_eq!(q30.get_ratio(), 0.0);
    }
}
