pub mod battery;
pub mod magnus;
pub mod mass_conservation;
pub mod pitot;
pub mod pressure_adc;
#[macro_use]
pub mod transfer;

use core::mem::MaybeUninit;

use crate::SerialMessage;

use defmt::info;
use rkyv::{
    api::low::to_bytes_in_with_alloc,
    rancor::Failure,
    ser::{allocator::SubAllocator, writer::Buffer},
    util::Align,
};

pub async fn send_message(message: SerialMessage) -> ([u8; 256], usize) {
    let mut output = Align([MaybeUninit::<u8>::uninit(); 256]);
    let mut alloc = [MaybeUninit::<u8>::uninit(); 256];

    let bytes = to_bytes_in_with_alloc::<_, _, Failure>(
        &message,
        Buffer::from(&mut *output),
        SubAllocator::new(&mut alloc),
    )
    .unwrap();

    let mut buffer = [0u8; 256];
    let len = bytes.len();

    // Add Length Prefix (u32 little endian)
    // First 4 bytes = length
    let len_bytes = (len as u32).to_le_bytes();
    buffer[0..4].copy_from_slice(&len_bytes);

    // Payload follows
    buffer[4..4 + len].copy_from_slice(&bytes);

    let data_to_display = &buffer[4..4 + len];

    info!(
        "Serialized message of length {}; its data {}",
        len, data_to_display
    );

    (buffer, 4 + len)
}
