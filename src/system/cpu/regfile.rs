/// Eight 8-Bit Registers
/// A is accumulator, F is Flag Register
/// Can also be accessed as four 16-bit combined registers,
/// AF, BC, DE, HL
#[derive(Eq, PartialEq, Debug)]
pub struct Regfile {
    pub r_a: u8,
    pub r_b: u8,
    pub r_c: u8,
    pub r_d: u8,
    pub r_e: u8,
    pub r_f: u8,
    pub r_h: u8,
    pub r_l: u8,
}

macro_rules! combine_two_u8 {
    ($first:expr, $second:expr) => {
        (($first as u16) << 8) | ($second as u16)
    };
}

impl Regfile {

    pub fn new() -> Regfile {
        let r_a: u8 = 0;
        let r_b: u8 = 0;
        let r_c: u8 = 0;
        let r_d: u8 = 0;
        let r_e: u8 = 0;
        let r_f: u8 = 0;
        let r_h: u8 = 0;
        let r_l: u8 = 0;
        Regfile {r_a, r_b, r_c, r_d, r_e, r_f, r_h, r_l}
    }

    /// Following are getters and setters for the four 16-bit pseudo-registers
    /// used by instructions
    pub fn get_af(&self) -> u16 {
        combine_two_u8!(self.r_a, self.r_f)
    } 
    pub fn get_bc(&self) -> u16 {
        combine_two_u8!(self.r_b, self.r_c)
    }
    pub fn get_de(&self) -> u16 {
        combine_two_u8!(self.r_d, self.r_e)
    }
    pub fn get_hl(&self) -> u16 {
        combine_two_u8!(self.r_h, self.r_l)
    }

    fn set_af(&mut self, val: u16) {
        self.r_a = ((val & 0xFF00) >> 8) as u8;
        self.r_f = (val & 0xF0) as u8; //lowest 4 bits discarded
    }

    fn set_bc(&mut self, val: u16) {
        self.r_b = ((val & 0xFF00) >> 8) as u8;
        self.r_c = (val & 0xFF) as u8;
    }

    fn set_de(&mut self, val: u16) {
        // println!("Writing: {0}", val);
        self.r_d = ((val & 0xFF00) >> 8) as u8;
        self.r_e = (val & 0xFF) as u8;
    }

    fn set_hl(&mut self, val: u16) {
        self.r_h = ((val & 0xFF00) >> 8) as u8;
        self.r_l = (val & 0xFF) as u8;
    }

    // Getters and setters for the flag register r_f, with each of the 
    // four upper bits corresponding to a different flag

    pub fn get_zero(&self) -> bool {
        if (self.r_f & 0x80) > 0 { true } else { false }
    }

    fn set_zero(&mut self, val: bool) {
        if val { self.r_f = self.r_f | 0x80 }
        else { self.r_f = self.r_f & 0x7F }
    }

    pub fn get_sub(&self) -> bool {
        if (self.r_f & 0x40) > 0 { true } else { false }
    }

    fn set_sub(&mut self, val: bool) {
        if val { self.r_f = self.r_f | 0x40 }
        else { self.r_f = self.r_f & 0xBF }
    }

    pub fn get_half_carry(&self) -> bool {
        if (self.r_f & 0x20) > 0 { true } else { false }
    }

    fn set_half_carry(&mut self, val: bool) {
        if val { self.r_f = self.r_f | 0x20 }
        else { self.r_f = self.r_f & 0xDF }
    }

    pub fn get_carry(&self) -> bool {
        if (self.r_f & 0x10) > 0 { true } else { false }
    }

    fn set_carry(&mut self, val: bool) {
        if val { self.r_f = self.r_f | 0x10 }
        else { self.r_f = self.r_f & 0xEF }
    }

    fn toggle_carry(&mut self) {
        let val = !self.get_carry();
        self.set_carry(val);
    }
}