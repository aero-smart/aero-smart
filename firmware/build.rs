use std::f32::consts::PI;
use std::fs::File;
use std::io::Write;

fn main() {
    println!("cargo:rustc-link-arg-bins=--nmagic");
    println!("cargo:rustc-link-arg-bins=-Tlink.x");
    println!("cargo:rustc-link-arg-bins=-Tdefmt.x");

    let n = 1024;
    let mut file = File::create("src/algorithms/hann_window.rs").unwrap();
    writeln!(file, "pub const HANN_WINDOW: [f32; {}] = [", n).unwrap();

    for i in 0..n {
        let val = 0.5 * (1.0 - (2.0 * PI * i as f32 / (n - 1) as f32).cos());
        write!(file, "    {}_f32,", val).unwrap();
        if (i + 1) % 4 == 0 {
            writeln!(file).unwrap();
        }
    }

    writeln!(file, "];").unwrap();
}
