pub struct Interval {
    pub min: f64,
    pub max: f64,
}

impl Interval {
    pub fn new(min: f64, max: f64) -> Self {
        Interval { min, max }
    }

    pub fn contains(&self, val: f64) -> bool {
        self.min < val && self.max > val
    }

    pub fn contains_including(&self, val: f64) -> bool {
        self.min <= val && self.max >= val
    }

    pub fn size(&self) -> f64 {
        self.max - self.min
    }

    pub const INFINITY: Interval = Interval {
        min: f64::NEG_INFINITY,
        max: f64::INFINITY,
    };
}
