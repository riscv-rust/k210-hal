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
    /// Interrupt wrapper type
    type Interrupt;
    /// Is this M-Mode interrupt enabled on given hart?
    fn is_enabled(hart_id: usize, interrupt: Interrupt) -> bool;
    /// Enable an interrupt for a given hart
    fn enable(hart_id: usize, interrupt: Interrupt);
    /// Disable an interrupt for a given hart
    fn disable(hart_id: usize, interrupt: Interrupt);
    /// Get global priority for one interrupt
    fn get_priority(interrupt: Interrupt) -> Priority;
    /// Globally set priority for one interrupt
    unsafe fn set_priority(interrupt: Interrupt, prio: Priority);
    /// Get priority threshold for a given hart
    fn get_threshold(hart_id: usize) -> Priority;
    /// Set the priority threshold for a given hart
    unsafe fn set_threshold(hart_id: usize, threshold: Priority);
    /// Mark that given hart have claimed to handle this interrupt
    fn claim(hart_id: usize) -> Option<Interrupt>;
    /// Mark that given hart have completed handling this interrupt
    fn complete(hart_id: usize, interrupt: Interrupt);
    /// Is this interrupt claimed and under procceeding? 
    fn is_pending(interrupt: Interrupt) -> bool;
}

impl PlicExt for PLIC {
    type Interrupt = Interrupt;
    fn is_enabled(hart_id: usize, interrupt: Interrupt) -> bool {
        let irq_number = interrupt.into_bits() as usize;
        unsafe {
            (*PLIC::ptr()).target_enables[hart_id].enable[irq_number / 32]
                .read().bits() & 1 << (irq_number % 32) != 0
        }
    }
    fn enable(hart_id: usize, interrupt: Interrupt) {
        let irq_number = interrupt.into_bits() as usize;
        unsafe {
            (*PLIC::ptr()).target_enables[hart_id].enable[irq_number / 32]
                .modify(|r, w| 
                    w.bits(r.bits() | 1 << (irq_number % 32)));
        }
    }
    fn disable(hart_id: usize, interrupt: Interrupt) { 
        let irq_number = interrupt.into_bits() as usize;
        unsafe {
            (*PLIC::ptr()).target_enables[hart_id].enable[irq_number / 32]
                .modify(|r, w| 
                    w.bits(r.bits() & !(1 << (irq_number % 32))));
        }
    }
    fn get_priority(interrupt: Interrupt) -> Priority { 
        let irq_number = interrupt.into_bits() as usize;
        let bits = unsafe {
            (*PLIC::ptr()).priority[irq_number].read().bits() 
        };
        Priority::from_bits(bits)
    }
    unsafe fn set_priority(interrupt: Interrupt, prio: Priority) { 
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
    fn claim(hart_id: usize) -> Option<Interrupt> {
        let bits = unsafe {
            (*PLIC::ptr()).targets[hart_id].claim.read().bits()
        };
        Nr::from_bits(bits)
    }
    fn complete(hart_id: usize, interrupt: Interrupt) {
        unsafe {
            (*PLIC::ptr()).targets[hart_id].claim.write(
                |w| 
                    w.bits(interrupt.into_bits()));
        }
    }
    fn is_pending(interrupt: Interrupt) -> bool {
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
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
    fn from_bits(bits: u32) -> Option<Self> {
        use Interrupt::*;
        match bits {
            0 => None,
            1 => Some(SPI0),
            2 => Some(SPI1),
            3 => Some(SPI_SLAVE),
            4 => Some(SPI3),
            5 => Some(I2S0),
            6 => Some(I2S1),
            7 => Some(I2S2),
            8 => Some(I2C0),
            9 => Some(I2C1),
            10 => Some(I2C2),
            11 => Some(UART1),
            12 => Some(UART2),
            13 => Some(UART3),
            14 => Some(TIMER0A),
            15 => Some(TIMER0B),
            16 => Some(TIMER1A),
            17 => Some(TIMER1B),
            18 => Some(TIMER2A),
            19 => Some(TIMER2B),
            20 => Some(RTC),
            21 => Some(WDT0),
            22 => Some(WDT1),
            23 => Some(APB_GPIO),
            24 => Some(DVP),
            25 => Some(AI),
            26 => Some(FFT),
            27 => Some(DMA0),
            28 => Some(DMA1),
            29 => Some(DMA2),
            30 => Some(DMA3),
            31 => Some(DMA4),
            32 => Some(DMA5),
            33 => Some(UARTHS),
            34 => Some(GPIOHS0),
            35 => Some(GPIOHS1),
            36 => Some(GPIOHS2),
            37 => Some(GPIOHS3),
            38 => Some(GPIOHS4),
            39 => Some(GPIOHS5),
            40 => Some(GPIOHS6),
            41 => Some(GPIOHS7),
            42 => Some(GPIOHS8),
            43 => Some(GPIOHS9),
            44 => Some(GPIOHS10),
            45 => Some(GPIOHS11),
            46 => Some(GPIOHS12),
            47 => Some(GPIOHS13),
            48 => Some(GPIOHS14),
            49 => Some(GPIOHS15),
            50 => Some(GPIOHS16),
            51 => Some(GPIOHS17),
            52 => Some(GPIOHS18),
            53 => Some(GPIOHS19),
            54 => Some(GPIOHS20),
            55 => Some(GPIOHS21),
            56 => Some(GPIOHS22),
            57 => Some(GPIOHS23),
            58 => Some(GPIOHS24),
            59 => Some(GPIOHS25),
            60 => Some(GPIOHS26),
            61 => Some(GPIOHS27),
            62 => Some(GPIOHS28),
            63 => Some(GPIOHS29),
            64 => Some(GPIOHS30),
            65 => Some(GPIOHS31),
            _ => panic!("invalid interrupt bits")
        }
    }
}

#[doc(hidden)]
pub trait Nr: Sized {
    fn into_bits(&self) -> u32;
    fn from_bits(bits: u32) -> Option<Self>;
}

/// Priority of an interrupt
#[derive(Clone, Copy, Debug)]
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
