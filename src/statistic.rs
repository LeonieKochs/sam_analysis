use crate::record::SamRecord;

pub trait Statistic {
    fn process (&mut self, record: &SamRecord);
    fn name(&self) -> &str;
    fn report(&self) -> String;
}