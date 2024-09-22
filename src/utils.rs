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

pub fn get_host(url: url::Url) -> String {
    match url.host_str() {
        Some(addr) => {
            let port = match url.port() {
                Some(port) => port,
                None => 21324,
            };
            return format!("{}:{}",addr,port);
        },
        None => {
            println!("Invalid host");
            std::process::exit(1);
        },
    }

}