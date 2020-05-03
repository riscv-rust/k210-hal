//! Core Local Interruptor (CLINT)
//!
//! TODO: Should this module designed in a somehow IP-core peripheral create?

use crate::pac::CLINT;

pub struct Clint {
    pub msip: [Msip; 2],
    pub mtime: Mtime,
    pub mtimecmp: [Mtimecmp; 2],
}

/// Opaque msip register
pub struct Msip {
    // false: 0, true: 1
    // for hals adapting more cores this should be an integer
    hart_id: bool,
}

impl Msip {
    /// Set msip register value
    pub fn set_value(&mut self, value: bool) {
        let hart_id = if self.hart_id { 1 } else { 0 };
        unsafe {
            (*CLINT::ptr()).msip[hart_id].write(|w| w.bits(if value { 1 } else { 0 }))
        }
    }
}

/// Opaque mtimecmp register
pub struct Mtimecmp {
    hart_id: bool,
}

impl Mtimecmp {
    /// Read mtimecmp register.
    pub fn mtimecmp(&self) -> u64 {
        let hart_id = if self.hart_id { 1 } else { 0 };
        unsafe { (*CLINT::ptr()).mtimecmp[hart_id].read().bits() }
    }

    /// Write mtimecmp register.
    pub fn set_mtimecmp(&mut self, value: u64) {
        let hart_id = if self.hart_id { 1 } else { 0 };
        // Volume II: RISC-V Privileged Architectures V1.10 p.31, figure 3.15
        unsafe { (*CLINT::ptr()).mtimecmp[hart_id].write(|w| w.bits(value)) };
    }
}

/// Opaque mtime register
pub struct Mtime {
    _ownership: ()
}

impl Mtime {
    /// Read mtime register.
    pub fn mtime(&self) -> u64 {
        unsafe { (*CLINT::ptr()).mtime.read().bits() }
    }
}
