#[macro_export]
macro_rules! linear_transfer_function {
    ($sensor:ident, $var:ident, $res:expr, $percent:expr, $range:expr) => {
        paste::paste! {
            #[inline]
            pub fn [<$sensor _ $var>](output: u16) -> f32 {
                let resolution = (1u32 << $res) - 1; // 2^res - 1
                let lower_margin = (1.0 - $percent) / 2.0;
                ((output as f32 - (lower_margin * resolution as f32)) / ($percent * resolution as f32))
                    * ($range.1 - $range.0) + $range.0
            }
        }
    };
}
