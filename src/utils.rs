use clap::error::{ContextKind, ContextValue, ErrorKind};

pub fn compare(a: f64, b: f64) -> f64 {
    if a > b {
        b
    } else {
        a
    }
}

pub fn offset(n: i16, max: i16, offset: i16) -> i16 {
    let mut o = n+offset;
    if o > max {
        o=o-max;
    }
    o
}

pub fn scaleu8tof32(old_value: u8) -> f32 { // Convert a u8 between 0 and 255 to a f32 between 0.0 and 1.0 for piston colors
    (((old_value as f32 - 0.0) * (0.0 - 1.0)) / (0.0 - 255.0)) + 0.0
}

pub fn err_missing_args(args: Vec<String>) {
    let mut err = clap::Error::new(ErrorKind::MissingRequiredArgument);
    err.insert(ContextKind::InvalidArg, ContextValue::Strings(args));
    let _ = err.print();
    std::process::exit(1);
}