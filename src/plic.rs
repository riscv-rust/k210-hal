// I don't know if this part should be implemented in runtime
// keep by now, may be replaced on further designs
// #[doc(hidden)]
// #[link_name = "MachineExternal"]
// pub extern fn machine_external() {

// }

use crate::pac::PLIC;

pub enum Interrupt {
    //todo
}

pub trait Nr {
    fn into_bits(&self) -> u32;
    fn from_bits(bits: u32) -> Self;
}

pub enum Priority {
    /// Priority 0: Never interrupt
    P0,
    /// Priority 1: Lowest active priority
    P1,
    /// Priority 2
    P2,
    /// Priority 3
    P3,
    /// Priority 4
    P4,
    /// Priority 5
    P5,
    /// Priority 6
    P6,
    /// Priority 7: Highest priority
    P7,
}

impl Priority {
    fn into_bits(self) -> u32 {
        match self {
            Priority::P0 => 0,
            Priority::P1 => 1,
            Priority::P2 => 2,
            Priority::P3 => 3,
            Priority::P4 => 4,
            Priority::P5 => 5,
            Priority::P6 => 6,
            Priority::P7 => 7,
        }
    }     
    fn from_bits(prio: u32) -> Priority {
        match prio {
            0 => Priority::P0,
            1 => Priority::P1,
            2 => Priority::P2,
            3 => Priority::P3,
            4 => Priority::P4,
            5 => Priority::P5,
            6 => Priority::P6,
            7 => Priority::P7,
            _ => panic!("Invalid priority"),
        }
    }
}

pub trait PlicExt {
    type Priority;
    fn is_enabled<I: Nr>(hart_id: usize, interrupt: I) -> bool;
    fn enable<I: Nr>(hart_id: usize, interrupt: I); 
    fn disable<I: Nr>(hart_id: usize, interrupt: I);
    fn get_priority<I: Nr>(interrupt: I) -> Self::Priority;
    unsafe fn set_priority<I: Nr>(interrupt: I, prio: Self::Priority);
    fn get_threshold(hart_id: usize) -> Self::Priority;
    unsafe fn set_threshold(hart_id: usize, threshold: Self::Priority);
    fn claim<I: Nr>(hart_id: usize) -> I;
    fn complete<I: Nr>(hart_id: usize, interrupt: I);
}

impl PlicExt for PLIC {
    type Priority = Priority;
    fn is_enabled<I: Nr>(hart_id: usize, interrupt: I) -> bool {
        let irq_number = interrupt.into_bits() as usize;
        unsafe {
            (*PLIC::ptr()).target_enables[hart_id].enable[irq_number / 32]
                .read().bits() & 1 << (irq_number % 32) != 0
        }
    }
    fn enable<I: Nr>(hart_id: usize, interrupt: I) {
        let irq_number = interrupt.into_bits() as usize;
        unsafe {
            (*PLIC::ptr()).target_enables[hart_id].enable[irq_number / 32]
                .modify(|r, w| 
                    w.bits(r.bits() | 1 << (irq_number % 32)));
        }
    }
    fn disable<I: Nr>(hart_id: usize, interrupt: I) { 
        let irq_number = interrupt.into_bits() as usize;
        unsafe {
            (*PLIC::ptr()).target_enables[hart_id].enable[irq_number / 32]
                .modify(|r, w| 
                    w.bits(r.bits() & !(1 << (irq_number % 32))));
        }
    }
    fn get_priority<I: Nr>(interrupt: I) -> Priority { 
        let irq_number = interrupt.into_bits() as usize;
        let bits = unsafe {
            (*PLIC::ptr()).priority[irq_number].read().bits() 
        };
        Priority::from_bits(bits)
    }
    unsafe fn set_priority<I: Nr>(interrupt: I, prio: Priority) { 
        let irq_number = interrupt.into_bits() as usize;
        (*PLIC::ptr()).priority[irq_number].write(
            |w| 
                w.bits(prio.into_bits()));
    }
    fn get_threshold(hart_id: usize) -> Priority {
        let bits = unsafe {
            (*PLIC::ptr()).targets[hart_id].threshold.read().bits()
        };
        Priority::from_bits(bits)
    }
    unsafe fn set_threshold(hart_id: usize, threshold: Priority) {
        (*PLIC::ptr()).targets[hart_id].threshold.write(
            |w| 
                w.bits(threshold.into_bits()));
    }
    fn claim<I: Nr>(hart_id: usize) -> I {
        let bits = unsafe {
            (*PLIC::ptr()).targets[hart_id].claim.read().bits()
        };
        Nr::from_bits(bits)
    }
    fn complete<I: Nr>(hart_id: usize, interrupt: I) {
        unsafe {
            (*PLIC::ptr()).targets[hart_id].claim.write(
                |w| 
                    w.bits(interrupt.into_bits()));
        }
    }
}
