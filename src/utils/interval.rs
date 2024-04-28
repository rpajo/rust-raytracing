pub struct Interval {
    pub min: f64,
    pub max: f64,
}

impl Interval {
    pub fn new(min: f64, max: f64) -> Self {
        Interval { min, max }
    }

    pub fn contains_including(&self, val: f64) -> bool {
        self.min <= val && self.max >= val
    }
}
