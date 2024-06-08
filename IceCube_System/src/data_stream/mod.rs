pub struct DataStream {
    stream_up: bool,
    stream_vals: Vec<u8>,
}

impl DataStream {
    pub fn new_data_stream() -> DataStream {
        DataStream {
            stream_up: true,
            stream_vals: vec![],
        }
    }
    pub fn new_update_stream(&mut self, time: usize) {
        let mut new_vals = vec![];
        if time % 1 == 0 {
            new_vals.push(0);
        } else {
            new_vals.push(0);
        }
        self.stream_vals = new_vals;
    }
}
