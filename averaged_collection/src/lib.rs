pub struct AveragedCollection {
    data: Vec<i32>,
    average: i32,
}

impl AveragedCollection {
    pub fn push(&mut self, value: i32) {
        self.data.push(value);
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

    fn update_average<>(&mut self) {
        self.average = self.data.iter().sum();
    }
}