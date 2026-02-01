use std::f32::consts::PI;
use std::fs::File;
use std::io::Write;

fn main() {
    println!("cargo:rustc-link-arg-bins=--nmagic");
    println!("cargo:rustc-link-arg-bins=-Tlink.x");
    println!("cargo:rustc-link-arg-bins=-Tdefmt.x");

    {
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

        println!("Generated Hann window coefficients.");
    }

    {
        let n = 1024;
        let mut file = File::create("src/algorithms/blackman_harris_window.rs").unwrap();

        writeln!(file, "pub const BLACKMAN_HARRIS_WINDOW: [f32; {}] = [", n).unwrap();

        let parameters: [f32; 4] = [0.35875, 0.48829, 0.14128, 0.01168];

        for i in 0..n {
            let mut val = parameters[0];
            for (k, &a_k) in parameters.iter().enumerate().skip(1) {
                val -= a_k * (2.0 * PI * k as f32 * i as f32 / (n - 1) as f32).cos();
            }
            write!(file, "    {}_f32,", val).unwrap();
            if (i + 1) % 4 == 0 {
                writeln!(file).unwrap();
            }
        }

        writeln!(file, "];").unwrap();

        println!("Generated Blackman-Harris window coefficients.");
    }
}
