use crate::utils;

pub struct ParserOptions {
    stripheader: usize, // Strip amount of bytes before actual leddata begins
    double: bool, // Fill up all the 480 leds when only 240 are inputted
    offset: i16, // Offset rotation of leds
    mirror: bool // Mirror the leds
}

impl ParserOptions {
    pub fn stripheader(mut self, val: usize) -> ParserOptions { self.stripheader = val; self }
    pub fn double(mut self, val: bool) -> ParserOptions { self.double = val; self }
    pub fn offset(mut self, val: i16) -> ParserOptions { self.offset = val; self }
    pub fn mirror(mut self, val: bool) -> ParserOptions { self.mirror = val; self }
}

impl Default for ParserOptions {
    fn default() -> Self {
        ParserOptions {
            stripheader: 0,
            double: false,
            offset: 0,
            mirror: false,    
        } 
    }
}

pub fn parseleds(mut data: Vec<u8>, options: ParserOptions) -> Vec<[f32; 4]> {
    let mut out: Vec<[f32; 4]> = Vec::new();
    if options.stripheader > 0 {
        data.drain(0..options.stripheader);
    }
    let expectedlen = if options.double { 720 } else { 1440 };
    if data.len() < expectedlen {
	data.append(&mut vec![5; expectedlen-out.len()]);
    }
    for x in data.array_chunks::<3>() {
        out.push([utils::scaleu8tof32(x[0]), utils::scaleu8tof32(x[1]), utils::scaleu8tof32(x[2]), 1.0]);
        if options.double && !options.mirror {
            out.push(out.last().unwrap().clone());
        }
    }
    out = if options.mirror {
        let mut shifted: Vec<[f32; 4]> = Vec::new();
        for e in 0..60 {
            for i in 0..4 {
                shifted.push(out[(3-i)*60+(59-utils::offset(e as i16, 59, options.offset) as usize)]);
                if options.double {
                    shifted.push(shifted.last().unwrap().clone());
                    // shifted.push(out[(3-i)*60+(59-offset(e as i16, 59, options.offset) as usize)]);
                }
            }
        }
        shifted
    } else { out };
    //if out.len() < 480 {
    //    out.append(&mut vec![[0.02, 0.02, 0.02, 1.0]; 480-out.len()]); //Ensure we don't get any out of bounds errors
    //}
    out
}
