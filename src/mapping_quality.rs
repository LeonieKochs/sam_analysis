//	Druchschnittliche Mapping Qualtiät aller Reads in einer SAM-Datei

pub struct MappingQuality{
    sum_quality: u64,
    count: u64,
}

impl MappingQuality{
    pub fn new()-> Self { 
        Self {sum_quality: 0, count: 0} }

    pub fn update(&mut self, mapq: u8) {
        self.sum_quality += mapq as u64;
        self.count += 1;
    } 

    pub fn get_average(&self) -> f64 {
        if self.count > 0 {
            self.sum_quality as f64 / self.count as f64
        } else {
            0.0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::MappingQuality;

    #[test]
    fn test_mapping_quality_average() {
        let mut mq = MappingQuality::new();
        let qualities = vec![30, 20, 15];

        for q in &qualities {
            mq.update(*q);
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
