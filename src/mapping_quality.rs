use crate::record::SamRecord;
use crate::statistic::Statistic;
//	Druchschnittliche Mapping Qualtiät aller Reads in einer SAM-Datei

pub struct MappingQuality{
    sum_quality: u64,
    count: u64,
}

impl MappingQuality{
    pub fn new()-> Self { 
        Self {sum_quality: 0, count: 0} }

    /* 
    pub fn update(&mut self, mapq: u8) {
        self.sum_quality += mapq as u64;
        self.count += 1;
    } */

    pub fn get_average(&self) -> f64 {
        if self.count > 0 {
            self.sum_quality as f64 / self.count as f64
        } else {
            0.0
        }
    }
}

impl Statistic for MappingQuality {
    fn process (&mut self, record: &SamRecord) {
        self.sum_quality += record.mapq as u64;
        self.count +=1;
    }

    fn name(&self) -> &str {
        "Average Mapping Quality"
    }

    fn report(&self) -> String {
        format!("{:.2}", self.get_average()) //returns String, shows 2 decimal places
    }
}



#[cfg(test)]
mod tests {
    use super::MappingQuality;
    use crate::{record::SamRecord, statistic::Statistic};

    fn fake_record(mapq: u8) -> SamRecord {
        SamRecord {
            mapq,
            ..Default::default() // Fills in the rest of the fields with default values
        }
    }

    #[test]
    fn test_mapping_quality_average() {
        let mut mq = MappingQuality::new();
        let qualities = vec![30, 20, 15];

        for q in &qualities {
            let record = fake_record(*q);
            mq.process(&record);
        }

        let expected = (30.0 + 20.0 + 15.0) / 3.0;
        let result = mq.get_average();

        assert_eq!(expected, result);
    }

    #[test]
    fn test_empty_mapping_quality() {
        let mq = MappingQuality::new();
        assert_eq!(mq.get_average(), 0.0);
    }
}

