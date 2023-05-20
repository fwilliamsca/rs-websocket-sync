//! WebSocket Frame Parser optimized for throughput.
//! Uses explicit SIMD instructions (AVX2) to unmask payloads.

use bytes::{Buf, BytesMut};
use std::mem;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum OpCode {
    Continuation = 0x0,
    Text = 0x1,
    Binary = 0x2,
    Close = 0x8,
    Ping = 0x9,
    Pong = 0xA,
}

/// Represents a raw WebSocket frame header.
pub struct FrameHeader {
    pub fin: bool,
    pub opcode: OpCode,
    pub payload_len: u64,
    pub masking_key: Option<[u8; 4]>,
}

/// Applies the masking key to the payload using SIMD if available.
/// XORs 32 bytes at a time instead of byte-by-byte.
#[inline(always)]
pub unsafe fn apply_mask_fast(buf: &mut [u8], mask: [u8; 4]) {
    let len = buf.len();
    let mut ptr = buf.as_mut_ptr();
    
    // Check for AVX2 support at runtime
    if is_x86_feature_detected!("avx2") {
        use std::arch::x86_64::*;
        
        // Broadcast the 4-byte mask to fill a 256-bit register
        let mask_repeated = [
            mask[0], mask[1], mask[2], mask[3],
            mask[0], mask[1], mask[2], mask[3],
            mask[0], mask[1], mask[2], mask[3],
            mask[0], mask[1], mask[2], mask[3],
            mask[0], mask[1], mask[2], mask[3],
            mask[0], mask[1], mask[2], mask[3],
            mask[0], mask[1], mask[2], mask[3],
            mask[0], mask[1], mask[2], mask[3],
        ];
        
        let avx_mask = _mm256_loadu_si256(mask_repeated.as_ptr() as *const _);
        
        // Process 32 bytes per cycle
        while len >= 32 {
            let chunk = _mm256_loadu_si256(ptr as *const _);
            let xored = _mm256_xor_si256(chunk, avx_mask);
            _mm256_storeu_si256(ptr as *mut _, xored);
            
            ptr = ptr.add(32);
        }
    }
    
    // Fallback for remaining bytes
    // ... (Scalar implementation omitted for brevity)
}