pub struct Timer {
    // internal values
    timer_counter: u16,
    timer_cycles: u16,
    divider_counter: u8,
    // memory mapped values
    r_DIV: u8,
    r_TIMA: u8,
    r_TMA: u8,
    r_TAC: u8,
}


// memory mapped registers
// FF04 - DIV: Divider Register
// FF05 - TIMA: Timer Counter
// FF06 - TMA: Timer Modulo
// FF07 - TAC: Timer Control

impl Timer {
    pub fn new() -> Timer {
        Timer {
            timer_counter: 0,
            divider_counter: 0,
            timer_cycles: 1024,
            r_DIV: 0,
            r_TIMA: 0,
            r_TMA: 0,
            r_TAC: 0,
        }
    }

    fn timer_control(&mut self) {
        let byte = self.r_TAC & 0x3; // Bits 0 and 1
        self.timer_cycles = match byte {
            // number of cycles to wait before updating timer
            0x0 => 1024, // frequency 0x1000hz
            0x1 => 16, // frequency 0x40000hz
            0x2 => 64, // frequency 0x10000hz
            0x3 => 256, // frequency 0x4000hz
            _ => 1024
        };
        //println!("Timer Threshold: {}", self.timer_cycles);
    }

    // returns True if Timer Interrupt is to be set, otherwise false
    pub fn update_timestep(&mut self, cycles: u8) -> bool {
        self.divider_inc(cycles);
        if self.timer_enabled() {
            self.timer_counter = self.timer_counter.wrapping_add(cycles as u16);
            //println!("Counter val: {}", self.timer_counter );
            if self.timer_counter >= self.timer_cycles {
                //println!("Incrementing");
                self.timer_counter = 0;
                return self.timer_inc();
            }
            
        }
        false
    }

    fn timer_inc(&mut self) -> bool {
        if self.r_TIMA == 0xFF {
            // Overflow, so reset to modulo and raise Timer Interrupt
            // println!("Overflow");
            self.r_TIMA = self.r_TMA;
            return true;
        }
        else {
            self.r_TIMA = self.r_TIMA.wrapping_add(1);
            //println!("Timer Value: {}", self.r_TIMA);
        }
        false
    }

    fn divider_inc(&mut self, cycles: u8) {
        // DIV incremented at 0x4000hz always
        self.divider_counter = self.divider_counter.wrapping_add(cycles);
        if self.divider_counter >= 255 {
            self.divider_counter = 0;
            self.r_DIV = self.r_DIV.wrapping_add(1); // resets to 0 when overflowing from 0xFF, no interrupt raised
        }
    }

    pub fn timer_enabled(&self) -> bool {
        self.r_TAC & 0x4 > 0 // bit 2 of TAC set
    }

    pub fn reset_DIV(&mut self) {
        // any write to r_DIV resets it
        self.r_DIV = 0;
    }

    pub fn get_DIV(&self) -> u8 {
        self.r_DIV
    }

    pub fn set_TIMA(&mut self, val: u8) {
        self.r_TIMA = val;
    }

    pub fn get_TIMA(&self) -> u8 {
        self.r_TIMA
    }

    pub fn set_TMA(&mut self, val: u8) {
        self.r_TMA = val;
    }

    pub fn get_TMA(&self) -> u8 {
        self.r_TMA
    }

    pub fn get_TAC(&self) -> u8 {
        return 0x7 & self.r_TAC;
    }

    pub fn set_TAC(&mut self, val: u8) {
        self.r_TAC = val & 0x7;
        self.timer_control(); // update timer counter threshold
    }
}