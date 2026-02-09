use crate::linear_transfer_function;

const PSI_TO_PASCAL: f32 = 6894.76;

linear_transfer_function!(
    ms4525do,
    pressure,
    14,
    0.8f32,
    (
        -1.0f32 * PSI_TO_PASCAL as f32,
        1.0f32 * PSI_TO_PASCAL as f32
    )
);
linear_transfer_function!(ms4525do, temperature, 11, 1.0f32, (-50.0f32, 150.0f32));
