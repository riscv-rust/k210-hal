//! Platform-Level Interrupt Controller (PLIC)

// I don't know if this part should be implemented in runtime
// keep by now, may be replaced on further designs
// #[doc(hidden)]
// #[link_name = "MachineExternal"]
// pub extern fn machine_external() {

// }

use crate::pac::PLIC;

/// Extension trait for PLIC interrupt controller peripheral
pub trait PlicExt {
    /// Is this M-Mode interrupt enabled on given hart?
    fn is_enabled<I: Nr>(hart_id: usize, interrupt: I) -> bool;
    /// Enable an interrupt for a given hart
    fn enable<I: Nr>(hart_id: usize, interrupt: I);
    /// Disable an interrupt for a given hart
    fn disable<I: Nr>(hart_id: usize, interrupt: I);
    /// Get global priority for one interrupt
    fn get_priority<I: Nr>(interrupt: I) -> Priority;
    /// Globally set priority for one interrupt
    unsafe fn set_priority<I: Nr>(interrupt: I, prio: Priority);
    /// Get priority threshold for a given hart
    fn get_threshold(hart_id: usize) -> Priority;
    /// Set the priority threshold for a given hart
    unsafe fn set_threshold(hart_id: usize, threshold: Priority);
    /// Mark that given hart have claimed to handle this interrupt
    fn claim<I: Nr>(hart_id: usize) -> I;
    /// Mark that given hart have completed handling this interrupt
    fn complete<I: Nr>(hart_id: usize, interrupt: I);
    /// Is this interrupt claimed and under procceeding? 
    fn is_pending<I: Nr>(interrupt: I) -> bool;
}

impl PlicExt for PLIC {
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
    fn is_pending<I: Nr>(interrupt: I) -> bool {
        let irq_number = interrupt.into_bits() as usize;
        unsafe {
            (*PLIC::ptr()).pending[irq_number / 32]
                .read().bits() & 1 << (irq_number % 32) != 0
        }
    }
}

/// Enum for all interrupts
#[allow(non_camel_case_types)]
#[repr(u8)]
#[derive(Clone, Copy)]
pub enum Interrupt {
    #[doc = "SPI0 interrupt"]
    SPI0 = 1,
    #[doc = "SPI1 interrupt"]
    SPI1 = 2,
    #[doc = "SPI_SLAVE interrupt"]
    SPI_SLAVE = 3,
    #[doc = "SPI3 interrupt"]
    SPI3 = 4,
    #[doc = "I2S0 interrupt"]
    I2S0 = 5,
    #[doc = "I2S1 interrupt"]
    I2S1 = 6,
    #[doc = "I2S2 interrupt"]
    I2S2 = 7,
    #[doc = "I2C0 interrupt"]
    I2C0 = 8,
    #[doc = "I2C1 interrupt"]
    I2C1 = 9,
    #[doc = "I2C2 interrupt"]
    I2C2 = 10,
    #[doc = "UART1 interrupt"]
    UART1 = 11,
    #[doc = "UART2 interrupt"]
    UART2 = 12,
    #[doc = "UART3 interrupt"]
    UART3 = 13,
    #[doc = "TIMER0 channel 0 or 1 interrupt"]
    TIMER0A = 14,
    #[doc = "TIMER0 channel 2 or 3 interrupt"]
    TIMER0B = 15,
    #[doc = "TIMER1 channel 0 or 1 interrupt"]
    TIMER1A = 16,
    #[doc = "TIMER1 channel 2 or 3 interrupt"]
    TIMER1B = 17,
    #[doc = "TIMER2 channel 0 or 1 interrupt"]
    TIMER2A = 18,
    #[doc = "TIMER2 channel 2 or 3 interrupt"]
    TIMER2B = 19,
    #[doc = "RTC tick and alarm interrupt"]
    RTC = 20,
    #[doc = "Watching dog timer0 interrupt"]
    WDT0 = 21,
    #[doc = "Watching dog timer1 interrupt"]
    WDT1 = 22,
    #[doc = "APB GPIO interrupt"]
    APB_GPIO = 23,
    #[doc = "Digital video port interrupt"]
    DVP = 24,
    #[doc = "AI accelerator interrupt"]
    AI = 25,
    #[doc = "FFT accelerator interrupt"]
    FFT = 26,
    #[doc = "DMA channel0 interrupt"]
    DMA0 = 27,
    #[doc = "DMA channel1 interrupt"]
    DMA1 = 28,
    #[doc = "DMA channel2 interrupt"]
    DMA2 = 29,
    #[doc = "DMA channel3 interrupt"]
    DMA3 = 30,
    #[doc = "DMA channel4 interrupt"]
    DMA4 = 31,
    #[doc = "DMA channel5 interrupt"]
    DMA5 = 32,
    #[doc = "Hi-speed UART0 interrupt"]
    UARTHS = 33,
    #[doc = "Hi-speed GPIO0 interrupt"]
    GPIOHS0 = 34,
    #[doc = "Hi-speed GPIO1 interrupt"]
    GPIOHS1 = 35,
    #[doc = "Hi-speed GPIO2 interrupt"]
    GPIOHS2 = 36,
    #[doc = "Hi-speed GPIO3 interrupt"]
    GPIOHS3 = 37,
    #[doc = "Hi-speed GPIO4 interrupt"]
    GPIOHS4 = 38,
    #[doc = "Hi-speed GPIO5 interrupt"]
    GPIOHS5 = 39,
    #[doc = "Hi-speed GPIO6 interrupt"]
    GPIOHS6 = 40,
    #[doc = "Hi-speed GPIO7 interrupt"]
    GPIOHS7 = 41,
    #[doc = "Hi-speed GPIO8 interrupt"]
    GPIOHS8 = 42,
    #[doc = "Hi-speed GPIO9 interrupt"]
    GPIOHS9 = 43,
    #[doc = "Hi-speed GPIO10 interrupt"]
    GPIOHS10 = 44,
    #[doc = "Hi-speed GPIO11 interrupt"]
    GPIOHS11 = 45,
    #[doc = "Hi-speed GPIO12 interrupt"]
    GPIOHS12 = 46,
    #[doc = "Hi-speed GPIO13 interrupt"]
    GPIOHS13 = 47,
    #[doc = "Hi-speed GPIO14 interrupt"]
    GPIOHS14 = 48,
    #[doc = "Hi-speed GPIO15 interrupt"]
    GPIOHS15 = 49,
    #[doc = "Hi-speed GPIO16 interrupt"]
    GPIOHS16 = 50,
    #[doc = "Hi-speed GPIO17 interrupt"]
    GPIOHS17 = 51,
    #[doc = "Hi-speed GPIO18 interrupt"]
    GPIOHS18 = 52,
    #[doc = "Hi-speed GPIO19 interrupt"]
    GPIOHS19 = 53,
    #[doc = "Hi-speed GPIO20 interrupt"]
    GPIOHS20 = 54,
    #[doc = "Hi-speed GPIO21 interrupt"]
    GPIOHS21 = 55,
    #[doc = "Hi-speed GPIO22 interrupt"]
    GPIOHS22 = 56,
    #[doc = "Hi-speed GPIO23 interrupt"]
    GPIOHS23 = 57,
    #[doc = "Hi-speed GPIO24 interrupt"]
    GPIOHS24 = 58,
    #[doc = "Hi-speed GPIO25 interrupt"]
    GPIOHS25 = 59,
    #[doc = "Hi-speed GPIO26 interrupt"]
    GPIOHS26 = 60,
    #[doc = "Hi-speed GPIO27 interrupt"]
    GPIOHS27 = 61,
    #[doc = "Hi-speed GPIO28 interrupt"]
    GPIOHS28 = 62,
    #[doc = "Hi-speed GPIO29 interrupt"]
    GPIOHS29 = 63,
    #[doc = "Hi-speed GPIO30 interrupt"]
    GPIOHS30 = 64,
    #[doc = "Hi-speed GPIO31 interrupt"]
    GPIOHS31 = 65,
}

impl Nr for Interrupt {
    fn into_bits(&self) -> u32 {
        *self as u8 as u32
    }
    fn from_bits(bits: u32) -> Self {
        use Interrupt::*;
        match bits {
            1 => SPI0,
            2 => SPI1,
            3 => SPI_SLAVE,
            4 => SPI3,
            5 => I2S0,
            6 => I2S1,
            7 => I2S2,
            8 => I2C0,
            9 => I2C1,
            10 => I2C2,
            11 => UART1,
            12 => UART2,
            13 => UART3,
            14 => TIMER0A,
            15 => TIMER0B,
            16 => TIMER1A,
            17 => TIMER1B,
            18 => TIMER2A,
            19 => TIMER2B,
            20 => RTC,
            21 => WDT0,
            22 => WDT1,
            23 => APB_GPIO,
            24 => DVP,
            25 => AI,
            26 => FFT,
            27 => DMA0,
            28 => DMA1,
            29 => DMA2,
            30 => DMA3,
            31 => DMA4,
            32 => DMA5,
            33 => UARTHS,
            34 => GPIOHS0,
            35 => GPIOHS1,
            36 => GPIOHS2,
            37 => GPIOHS3,
            38 => GPIOHS4,
            39 => GPIOHS5,
            40 => GPIOHS6,
            41 => GPIOHS7,
            42 => GPIOHS8,
            43 => GPIOHS9,
            44 => GPIOHS10,
            45 => GPIOHS11,
            46 => GPIOHS12,
            47 => GPIOHS13,
            48 => GPIOHS14,
            49 => GPIOHS15,
            50 => GPIOHS16,
            51 => GPIOHS17,
            52 => GPIOHS18,
            53 => GPIOHS19,
            54 => GPIOHS20,
            55 => GPIOHS21,
            56 => GPIOHS22,
            57 => GPIOHS23,
            58 => GPIOHS24,
            59 => GPIOHS25,
            60 => GPIOHS26,
            61 => GPIOHS27,
            62 => GPIOHS28,
            63 => GPIOHS29,
            64 => GPIOHS30,
            65 => GPIOHS31,
            _ => panic!("invalid interrupt bits")
        }
    }
}

#[doc(hidden)]
pub trait Nr {
    fn into_bits(&self) -> u32;
    fn from_bits(bits: u32) -> Self;
}

/// Priority of an interrupt
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
