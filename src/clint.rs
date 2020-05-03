//! Core Local Interruptor (CLINT)
//!
//! TODO: Should this module designed in a somehow IP-core peripheral create?

/// mtime register
pub mod mtime {
    use crate::pac;
    /// Read mtime register.
    pub fn read() -> u64 {
        unsafe { (*pac::CLINT::ptr()).mtime.read().bits() }
    }
}

/// msip register
///
/// TODO: not final API design; wait for const generics
pub mod msip {
    use crate::pac;
    // pub fn set_value<const HART_ID: usize>(value: bool) {
    //     unsafe {
    //         (*pac::CLINT::ptr()).msip[HART_ID].write(|w| w.bits(if value { 1 } else { 0 }))
    //     }
    // }

    pub fn set_value(hart_id: usize, value: bool) {
        unsafe {
            (*pac::CLINT::ptr()).msip[hart_id].write(|w| 
                w.bits(if value { 1 } else { 0 }))
        }
    }
}

/// mtimecmp register
///
/// TODO: not final API design; wait for const generics
pub mod mtimecmp {
    use crate::pac;
    // pub fn read<const HART_ID: usize>() -> u64 {
    //     unsafe { (*pac::CLINT::ptr()).mtimecmp[HART_ID].read().bits() }
    // }
    // pub fn write<const HART_ID: usize>(value: u64) {
    //     // Volume II: RISC-V Privileged Architectures V1.10 p.31, figure 3.15
    //     unsafe { (*pac::CLINT::ptr()).mtimecmp[HART_ID].write(|w| w.bits(value)) };
    // }
    pub fn read(hart_id: usize) -> u64 {
        unsafe { (*pac::CLINT::ptr()).mtimecmp[hart_id].read().bits() }
    }

    pub fn write(hart_id: usize, bits: u64) {
        // Volume II: RISC-V Privileged Architectures V1.10 p.31, figure 3.15
        unsafe { (*pac::CLINT::ptr()).mtimecmp[hart_id].write(|w| 
            w.bits(bits)) };
    }
}
