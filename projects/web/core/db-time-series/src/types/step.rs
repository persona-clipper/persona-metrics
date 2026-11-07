pub struct Step(pub i32);

pub enum StepRange {
    Seconds,
    Minutes,
    Hours,
}

impl StepRange {
    pub fn to_str(&self) -> &'static str {
        match &self {
            StepRange::Seconds => "s",
            StepRange::Minutes => "m",
            StepRange::Hours => "h",
        }
    }
}
impl Step {
    pub fn to_metrics_step(&self, range: StepRange) -> String {
        format!("{}{}", self.0, range.to_str())
    }
}