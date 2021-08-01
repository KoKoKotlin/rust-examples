#[derive(Debug)]
pub struct VMem {
    bytes: Vec<i8>,
    middle_index: i64,
    pub pointer: i64,
}

const INITIAL_SIZE: i64 = 2000;

impl VMem {
    pub fn new() -> Self {
        VMem {
            bytes: vec![0; INITIAL_SIZE as usize],
            middle_index: INITIAL_SIZE / 2,
            pointer: 0
        }
    }

    pub fn read(& mut self) -> i8 {
        if !(0..self.bytes.len()).contains(&((self.pointer + self.middle_index) as usize)) {
            return 0;
        }

        self.bytes[(self.pointer + self.middle_index) as usize]
    }

    pub fn write(& mut self, byte: i8) {
        if !(0..self.bytes.len()).contains(&((self.pointer + self.middle_index) as usize)) {
            self.resize();
        }

        self.bytes[(self.pointer + self.middle_index) as usize] = byte;
    }

    fn resize(& mut self) {
        let mut new_vec = vec![0i8; self.bytes.len() * 2];
        self.middle_index = self.middle_index * 2;
        
        for (i, byte) in self.bytes.iter().enumerate() {
            new_vec[(self.middle_index - self.middle_index / 2 + i as i64) as usize] = *byte;
        }
        
        self.bytes = new_vec;
    }

    pub fn inc(& mut self) {
        let byte = self.read();
        self.write(byte.overflowing_add(1).0);
    }

    pub fn dec(& mut self) {
        let byte = self.read();
        self.write(byte.overflowing_sub(1).0);
    }

    pub fn move_left(& mut self) {
        self.pointer -= 1;
    }

    pub fn move_right(& mut self) {
        self.pointer += 1;
    } 
}