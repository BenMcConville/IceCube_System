pub struct Graph {
    data_points: [(f64, f64); 200],
    current_index: usize,
}
impl Graph {
    pub fn new() -> Graph {
        Graph {
            data_points: [(0.0, 0.0); 200],
            current_index: 0,
        }
    }

    pub fn get_data_points(&self) -> &[(f64, f64); 200] {
        &self.data_points
    }
    pub fn update_data(&mut self, data_point: f64) {
        for val in self.data_points.iter_mut() {
            *val = (val.0 + 0.05, val.1);
        }

        self.current_index = if self.current_index == 0 {
            199
        } else {
            self.current_index - 1
        };
        self.data_points[self.current_index] = (0.0, data_point);
    }
}
