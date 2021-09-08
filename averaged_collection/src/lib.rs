pub struct AveragedCollection {
    data: Vec<i32>,
    average: f64,
}

impl AveragedCollection {
    pub fn new() -> AveragedCollection {
        AveragedCollection {
            data: Vec::new(),
            average: 0.0
        }
    }

    pub fn push(&mut self, value: i32) {
        self.data.push(value);
        self.update_average();
    }

    pub fn pop(&mut self) -> Option<i32> {
        match self.data.pop() {
            None => None,
            Some(value) => {
                self.update_average();
                Some(value)
            }
        }
    }

    fn update_average(&mut self) {
        let sum: i32 = self.data.iter().sum();
        self.average = sum as f64 / self.data.len() as f64;
    }

    pub fn average(&self) -> f64 {
        self.average
    }
}