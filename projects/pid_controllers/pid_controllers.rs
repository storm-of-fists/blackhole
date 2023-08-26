pub trait PIDController {
    fn latest_input(&self) -> &f64;
    fn output(&mut self, input: f64) -> f64;
}

struct PController {
    p_coefficient: f64
    latest_input: f64
}

impl PController {
    pub fn new(p_coefficient: f64) -> Self {
        self.p_coefficient = p_coefficient;
        self.latest_input = 0.0;
    }
}

impl PIDController for PController {
    fn output(&mut self, input: f64) -> f64 {
        self.latest_input = input;
        self.p_coefficient * self.latest_input
    }

    fn latest_input(&self) -> &f64 {
        &self.latest_input
    }
}

#[cfg(test)]
mod test {}
