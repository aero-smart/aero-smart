pub mod magnus;
pub mod mass_conservation;
pub mod pitot;

use core::mem::MaybeUninit;

use crate::SerialMessage;

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
    buffer[..len].copy_from_slice(&bytes);
    (buffer, len)
}
