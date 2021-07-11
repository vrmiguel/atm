pub struct Configurations {
    pub more_higher_bills: Vec<i32>,
    pub more_lower_bills: Vec<i32>,
}

impl Configurations {
    pub fn new(higher: &[i32], lower: &[i32]) -> Self {
        Self {
            more_higher_bills: higher.to_owned(),
            more_lower_bills: lower.to_owned()
        }
    }

    pub fn higher_bills_sum(&self) -> i32 {
        self.more_higher_bills.iter().sum()
    }

    pub fn lower_bills_sum(&self) -> i32 {
        self.more_lower_bills.iter().sum()
    }
}
