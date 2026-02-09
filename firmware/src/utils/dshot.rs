/// Build proper DSHOT600 frame with correct 37.5%/75% duty cycles
pub fn build_dshot_frame(throttle: u16, telemetry: bool, max_duty: u16) -> [u16; 17] {
    let mut frame = [0u16; 17];

    // DSHOT600 spec: 37.5% for bit 0, 75% for bit 1
    let bit_0_duty = (max_duty as f32 * 0.375) as u16; // ~31 with max_duty=83
    let bit_1_duty = (max_duty as f32 * 0.75) as u16; // ~62 with max_duty=83

    // Build DSHOT packet: 11-bit throttle + 1-bit telemetry + 4-bit CRC
    let mut packet = ((throttle & 0x7FF) << 1) | (telemetry as u16);

    // Calculate CRC (XOR of three 4-bit nibbles)
    let crc = (packet ^ (packet >> 4) ^ (packet >> 8)) & 0x0F;
    packet = (packet << 4) | crc;

    // Convert to PWM duty cycles (MSB first)
    for i in 0..16 {
        let bit = (packet >> (15 - i)) & 1;
        frame[i] = if bit == 1 { bit_1_duty } else { bit_0_duty };
    }

    frame[16] = 0; // Reset period
    frame
}
