pub struct Sequencer {
    prefix: &'static str,
    current: u64,
}

impl Sequencer {
    pub fn new(prefix: &'static str) -> Sequencer {
        Sequencer { prefix, current: 0 }
    }

    pub fn get(&mut self) -> String {
        let ret = self.current;
        self.current += 1;
        format!("prefix_{ret}")
    }
}
