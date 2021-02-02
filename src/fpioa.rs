//! Field Programmable IO Array (FPIOA)

/*
    new design: Split FPIOA into several IO pins (IO0, IO1, .., IO47)
    with ownership of IOx struct we may change their functions.
    For GPIOs and GPIOHS's we should split them into another GpioXx structs.

    note: other modules do not need to require for ownership of certain IOx struct
    for that pins and peripherals are considered separate modules.
    If this design results in inconvenience or violent of Rust's ownership role
    please fire an issue to tell us.
*/

use crate::{
    pac::FPIOA,
    sysctl::{self, APB0},
};
use core::marker::PhantomData;

/// FPIOA function
pub trait Function {
    const INDEX: u8;
}

/// Pull direction
#[derive(Copy, Clone)]
pub enum Pull {
    /// No pulls
    None,
    /// Pull down
    Down,
    /// Pull up
    Up,
}

/// FPIOA I/O Pin
pub trait IoPin {
    const INDEX: u8;

    fn set_io_pull(&mut self, pull: Pull) {
        unsafe {
            let fpioa = &*FPIOA::ptr();
            fpioa.io[Self::INDEX as usize].modify(|_, w| match pull {
                Pull::None => w.pu().bit(false).pd().bit(false),
                Pull::Down => w.pu().bit(false).pd().bit(true),
                Pull::Up => w.pu().bit(true).pd().bit(false),
            });
        }
    }
}

/// Marker trait for I/O pin function detection
pub trait Mode<FUNC> {}

/// Extension trait to split a FPIOA peripheral in independent pins and registers
pub trait FpioaExt {
    /// Splits the FPIOA block into independent pins and registers
    ///
    /// todo: split sysctl into two apb's, then use the APB0 to split Fpioa
    fn split(self, apb0: &mut APB0) -> Parts;
}

/// All I/O pins
macro_rules! def_io_pin {
    ($($IoX: ident: ($id: expr, $iox: ident, $func: ident);)+) => {
impl FpioaExt for FPIOA {
    fn split(self, apb0: &mut APB0) -> Parts {
        // enable APB0 bus
        apb0.enable();
        // enable sysctl peripheral
        sysctl::clk_en_peri().modify(|_r, w| w.fpioa_clk_en().set_bit());
        // return ownership
        Parts {
            $( $iox: $IoX { _function: PhantomData }, )+
        }
    }
}
/// FPIOA Parts
pub struct Parts {
    $(
        /// Programmable I/O pin
        pub $iox: $IoX<$func>,
    )+
    // todo: tie controller Tie (force set high or low as input)
}
pub use io_pins::*;
/// All I/O pins
pub mod io_pins {
    use core::marker::PhantomData;
    use super::{Function, IoPin, Mode, FUNCTION_DEFAULTS};
    use crate::pac::FPIOA;

$(
    /// Programmable I/O pin
    pub struct $IoX<FUNC> {
        pub(crate) _function: PhantomData<FUNC>
    }

    impl<FUNC> $IoX<FUNC> {
        /// Change the programmable I/O pin into given function
        ///
        /// You may refer to module [functions] for all functions.
        ///
        /// [functions]: ../functions/index.html
        pub fn into_function<F: Function>(self, func: F) -> $IoX<F> {
            let _ = func; // note(discard): Zero-sized typestate value
            unsafe { &(*FPIOA::ptr()).io[$id].write(|w|
                w.bits(FUNCTION_DEFAULTS[F::INDEX as usize])
            ) };
            $IoX { _function: PhantomData }
        }
    }
    impl<FUNC> IoPin for $IoX<FUNC> {
        const INDEX: u8 = $id;
    }
    impl<FUNC> Mode<FUNC> for $IoX<FUNC> {}
)+
}
    };
}

def_io_pin! {
    Io0: (0, io0, JTAG_TCLK);
    Io1: (1, io1, JTAG_TDI);
    Io2: (2, io2, JTAG_TMS);
    Io3: (3, io3, JTAG_TDO);
    Io4: (4, io4, UARTHS_RX);
    Io5: (5, io5, UARTHS_TX);
    Io6: (6, io6, RESV0); // (FLOAT*): no default function
    Io7: (7, io7, RESV0); // (FLOAT*): no default function
    Io8: (8, io8, GPIO0);
    Io9: (9, io9, GPIO1);
    Io10: (10, io10, GPIO2);
    Io11: (11, io11, GPIO3);
    Io12: (12, io12, GPIO4);
    Io13: (13, io13, GPIO5);
    Io14: (14, io14, GPIO6);
    Io15: (15, io15, GPIO7);
    Io16: (16, io16, GPIOHS0);
    Io17: (17, io17, GPIOHS1);
    Io18: (18, io18, GPIOHS2);
    Io19: (19, io19, GPIOHS3);
    Io20: (20, io20, GPIOHS4);
    Io21: (21, io21, GPIOHS5);
    Io22: (22, io22, GPIOHS6);
    Io23: (23, io23, GPIOHS7);
    Io24: (24, io24, GPIOHS8);
    Io25: (25, io25, GPIOHS9);
    Io26: (26, io26, GPIOHS10);
    Io27: (27, io27, GPIOHS11);
    Io28: (28, io28, GPIOHS12);
    Io29: (29, io29, GPIOHS13);
    Io30: (30, io30, GPIOHS14);
    Io31: (31, io31, GPIOHS15);
    Io32: (32, io32, GPIOHS16);
    Io33: (33, io33, GPIOHS17);
    Io34: (34, io34, GPIOHS18);
    Io35: (35, io35, GPIOHS19);
    Io36: (36, io36, GPIOHS20);
    Io37: (37, io37, GPIOHS21);
    Io38: (38, io38, GPIOHS22);
    Io39: (39, io39, GPIOHS23);
    Io40: (40, io40, GPIOHS24);
    Io41: (41, io41, GPIOHS25);
    Io42: (42, io42, GPIOHS26);
    Io43: (43, io43, GPIOHS27);
    Io44: (44, io44, GPIOHS28);
    Io45: (45, io45, GPIOHS29);
    Io46: (46, io46, GPIOHS30);
    Io47: (47, io47, GPIOHS31);
}

/** Defaults per function (from Kendryte fpioa.c) */
#[rustfmt::skip]
static FUNCTION_DEFAULTS: &[u32] = &[
    0x00900000, 0x00900001, 0x00900002, 0x00001f03, 0x00b03f04, 0x00b03f05, 0x00b03f06, 0x00b03f07,
    0x00b03f08, 0x00b03f09, 0x00b03f0a, 0x00b03f0b, 0x00001f0c, 0x00001f0d, 0x00001f0e, 0x00001f0f,
    0x03900010, 0x00001f11, 0x00900012, 0x00001f13, 0x00900014, 0x00900015, 0x00001f16, 0x00001f17,
    0x00901f18, 0x00901f19, 0x00901f1a, 0x00901f1b, 0x00901f1c, 0x00901f1d, 0x00901f1e, 0x00901f1f,
    0x00901f20, 0x00901f21, 0x00901f22, 0x00901f23, 0x00901f24, 0x00901f25, 0x00901f26, 0x00901f27,
    0x00901f28, 0x00901f29, 0x00901f2a, 0x00901f2b, 0x00901f2c, 0x00901f2d, 0x00901f2e, 0x00901f2f,
    0x00901f30, 0x00901f31, 0x00901f32, 0x00901f33, 0x00901f34, 0x00901f35, 0x00901f36, 0x00901f37,
    0x00901f38, 0x00901f39, 0x00901f3a, 0x00901f3b, 0x00901f3c, 0x00901f3d, 0x00901f3e, 0x00901f3f,
    0x00900040, 0x00001f41, 0x00900042, 0x00001f43, 0x00900044, 0x00001f45, 0x00b03f46, 0x00b03f47,
    0x00b03f48, 0x00b03f49, 0x00b03f4a, 0x00b03f4b, 0x00b03f4c, 0x00b03f4d, 0x00001f4e, 0x00001f4f,
    0x00001f50, 0x00001f51, 0x03900052, 0x00001f53, 0x00b03f54, 0x00900055, 0x00900056, 0x00001f57,
    0x00001f58, 0x00001f59, 0x0090005a, 0x0090005b, 0x0090005c, 0x0090005d, 0x00001f5e, 0x00001f5f,
    0x00001f60, 0x00001f61, 0x00001f62, 0x00001f63, 0x00001f64, 0x00900065, 0x00900066, 0x00900067,
    0x00900068, 0x00001f69, 0x00001f6a, 0x00001f6b, 0x00001f6c, 0x00001f6d, 0x00001f6e, 0x00001f6f,
    0x00900070, 0x00900071, 0x00900072, 0x00900073, 0x00001f74, 0x00001f75, 0x00001f76, 0x00001f77,
    0x00000078, 0x00000079, 0x0000007a, 0x0000007b, 0x0000007c, 0x0000007d, 0x0099107e, 0x0099107f,
    0x00991080, 0x00991081, 0x00991082, 0x00991083, 0x00001f84, 0x00001f85, 0x00001f86, 0x00900087,
    0x00900088, 0x00900089, 0x0090008a, 0x0090008b, 0x0090008c, 0x0090008d, 0x0090008e, 0x0090008f,
    0x00900090, 0x00900091, 0x00993092, 0x00993093, 0x00900094, 0x00900095, 0x00900096, 0x00900097,
    0x00900098, 0x00001f99, 0x00001f9a, 0x00001f9b, 0x00001f9c, 0x00001f9d, 0x00001f9e, 0x00001f9f,
    0x00001fa0, 0x00001fa1, 0x009000a2, 0x009000a3, 0x009000a4, 0x009000a5, 0x009000a6, 0x00001fa7,
    0x00001fa8, 0x00001fa9, 0x00001faa, 0x00001fab, 0x00001fac, 0x00001fad, 0x00001fae, 0x00001faf,
    0x009000b0, 0x009000b1, 0x009000b2, 0x009000b3, 0x009000b4, 0x00001fb5, 0x00001fb6, 0x00001fb7,
    0x00001fb8, 0x00001fb9, 0x00001fba, 0x00001fbb, 0x00001fbc, 0x00001fbd, 0x00001fbe, 0x00001fbf,
    0x00001fc0, 0x00001fc1, 0x00001fc2, 0x00001fc3, 0x00001fc4, 0x00001fc5, 0x00001fc6, 0x00001fc7,
    0x00001fc8, 0x00001fc9, 0x00001fca, 0x00001fcb, 0x00001fcc, 0x00001fcd, 0x00001fce, 0x00001fcf,
    0x00001fd0, 0x00001fd1, 0x00001fd2, 0x00001fd3, 0x00001fd4, 0x009000d5, 0x009000d6, 0x009000d7,
    0x009000d8, 0x009100d9, 0x00991fda, 0x009000db, 0x009000dc, 0x009000dd, 0x000000de, 0x009000df,
    0x00001fe0, 0x00001fe1, 0x00001fe2, 0x00001fe3, 0x00001fe4, 0x00001fe5, 0x00001fe6, 0x00001fe7,
    0x00001fe8, 0x00001fe9, 0x00001fea, 0x00001feb, 0x00001fec, 0x00001fed, 0x00001fee, 0x00001fef,
    0x00001ff0, 0x00001ff1, 0x00001ff2, 0x00001ff3, 0x00001ff4, 0x00001ff5, 0x00001ff6, 0x00001ff7,
    0x00001ff8, 0x00001ff9, 0x00001ffa, 0x00001ffb, 0x00001ffc, 0x00001ffd, 0x00001ffe, 0x00001fff,
];

pub use functions::*;

/// All programmable functions
pub mod functions {
    use super::Function;

    macro_rules! def_function {
        ($($name: ident: ($index: expr, $doc: expr);)+) => {
            $(
    #[doc = $doc]
    #[allow(non_camel_case_types)]
    pub struct $name;

    impl Function for $name {
        const INDEX: u8 = $index;
    }
            )+
        };
    }

    def_function! {
        JTAG_TCLK: (0, "JTAG Test Clock");
        JTAG_TDI: (1, "JTAG Test Data In");
        JTAG_TMS: (2, "JTAG Test Mode Select");
        JTAG_TDO: (3, "JTAG Test Data Out");
        SPI0_D0: (4, "SPI0 Data 0");
        SPI0_D1: (5, "SPI0 Data 1");
        SPI0_D2: (6, "SPI0 Data 2");
        SPI0_D3: (7, "SPI0 Data 3");
        SPI0_D4: (8, "SPI0 Data 4");
        SPI0_D5: (9, "SPI0 Data 5");
        SPI0_D6: (10, "SPI0 Data 6");
        SPI0_D7: (11, "SPI0 Data 7");
        SPI0_SS0: (12, "SPI0 Chip Select 0");
        SPI0_SS1: (13, "SPI0 Chip Select 1");
        SPI0_SS2: (14, "SPI0 Chip Select 2");
        SPI0_SS3: (15, "SPI0 Chip Select 3");
        SPI0_ARB: (16, "SPI0 Arbitration");
        SPI0_SCLK: (17, "SPI0 Serial Clock");
        UARTHS_RX: (18, "UART High speed Receiver");
        UARTHS_TX: (19, "UART High speed Transmitter");
        RESV6: (20, "Reserved function");
        RESV7: (21, "Reserved function");
        CLK_SPI1: (22, "Clock SPI1");
        CLK_I2C1: (23, "Clock I2C1");
        GPIOHS0: (24, "GPIO High speed 0");
        GPIOHS1: (25, "GPIO High speed 1");
        GPIOHS2: (26, "GPIO High speed 2");
        GPIOHS3: (27, "GPIO High speed 3");
        GPIOHS4: (28, "GPIO High speed 4");
        GPIOHS5: (29, "GPIO High speed 5");
        GPIOHS6: (30, "GPIO High speed 6");
        GPIOHS7: (31, "GPIO High speed 7");
        GPIOHS8: (32, "GPIO High speed 8");
        GPIOHS9: (33, "GPIO High speed 9");
        GPIOHS10: (34, "GPIO High speed 10");
        GPIOHS11: (35, "GPIO High speed 11");
        GPIOHS12: (36, "GPIO High speed 12");
        GPIOHS13: (37, "GPIO High speed 13");
        GPIOHS14: (38, "GPIO High speed 14");
        GPIOHS15: (39, "GPIO High speed 15");
        GPIOHS16: (40, "GPIO High speed 16");
        GPIOHS17: (41, "GPIO High speed 17");
        GPIOHS18: (42, "GPIO High speed 18");
        GPIOHS19: (43, "GPIO High speed 19");
        GPIOHS20: (44, "GPIO High speed 20");
        GPIOHS21: (45, "GPIO High speed 21");
        GPIOHS22: (46, "GPIO High speed 22");
        GPIOHS23: (47, "GPIO High speed 23");
        GPIOHS24: (48, "GPIO High speed 24");
        GPIOHS25: (49, "GPIO High speed 25");
        GPIOHS26: (50, "GPIO High speed 26");
        GPIOHS27: (51, "GPIO High speed 27");
        GPIOHS28: (52, "GPIO High speed 28");
        GPIOHS29: (53, "GPIO High speed 29");
        GPIOHS30: (54, "GPIO High speed 30");
        GPIOHS31: (55, "GPIO High speed 31");
        GPIO0: (56, "GPIO pin 0");
        GPIO1: (57, "GPIO pin 1");
        GPIO2: (58, "GPIO pin 2");
        GPIO3: (59, "GPIO pin 3");
        GPIO4: (60, "GPIO pin 4");
        GPIO5: (61, "GPIO pin 5");
        GPIO6: (62, "GPIO pin 6");
        GPIO7: (63, "GPIO pin 7");
        UART1_RX: (64, "UART1 Receiver");
        UART1_TX: (65, "UART1 Transmitter");
        UART2_RX: (66, "UART2 Receiver");
        UART2_TX: (67, "UART2 Transmitter");
        UART3_RX: (68, "UART3 Receiver");
        UART3_TX: (69, "UART3 Transmitter");
        SPI1_D0: (70, "SPI1 Data 0");
        SPI1_D1: (71, "SPI1 Data 1");
        SPI1_D2: (72, "SPI1 Data 2");
        SPI1_D3: (73, "SPI1 Data 3");
        SPI1_D4: (74, "SPI1 Data 4");
        SPI1_D5: (75, "SPI1 Data 5");
        SPI1_D6: (76, "SPI1 Data 6");
        SPI1_D7: (77, "SPI1 Data 7");
        SPI1_SS0: (78, "SPI1 Chip Select 0");
        SPI1_SS1: (79, "SPI1 Chip Select 1");
        SPI1_SS2: (80, "SPI1 Chip Select 2");
        SPI1_SS3: (81, "SPI1 Chip Select 3");
        SPI1_ARB: (82, "SPI1 Arbitration");
        SPI1_SCLK: (83, "SPI1 Serial Clock");
        SPI_SLAVE_D0: (84, "SPI Slave Data 0");
        SPI_SLAVE_SS: (85, "SPI Slave Select");
        SPI_SLAVE_SCLK: (86, "SPI Slave Serial Clock");
        I2S0_MCLK: (87, "I2S0 Master Clock");
        I2S0_SCLK: (88, "I2S0 Serial Clock(BCLK)");
        I2S0_WS: (89, "I2S0 Word Select(LRCLK)");
        I2S0_IN_D0: (90, "I2S0 Serial Data Input 0");
        I2S0_IN_D1: (91, "I2S0 Serial Data Input 1");
        I2S0_IN_D2: (92, "I2S0 Serial Data Input 2");
        I2S0_IN_D3: (93, "I2S0 Serial Data Input 3");
        I2S0_OUT_D0: (94, "I2S0 Serial Data Output 0");
        I2S0_OUT_D1: (95, "I2S0 Serial Data Output 1");
        I2S0_OUT_D2: (96, "I2S0 Serial Data Output 2");
        I2S0_OUT_D3: (97, "I2S0 Serial Data Output 3");
        I2S1_MCLK: (98, "I2S1 Master Clock");
        I2S1_SCLK: (99, "I2S1 Serial Clock(BCLK)");
        I2S1_WS: (100, "I2S1 Word Select(LRCLK)");
        I2S1_IN_D0: (101, "I2S1 Serial Data Input 0");
        I2S1_IN_D1: (102, "I2S1 Serial Data Input 1");
        I2S1_IN_D2: (103, "I2S1 Serial Data Input 2");
        I2S1_IN_D3: (104, "I2S1 Serial Data Input 3");
        I2S1_OUT_D0: (105, "I2S1 Serial Data Output 0");
        I2S1_OUT_D1: (106, "I2S1 Serial Data Output 1");
        I2S1_OUT_D2: (107, "I2S1 Serial Data Output 2");
        I2S1_OUT_D3: (108, "I2S1 Serial Data Output 3");
        I2S2_MCLK: (109, "I2S2 Master Clock");
        I2S2_SCLK: (110, "I2S2 Serial Clock(BCLK)");
        I2S2_WS: (111, "I2S2 Word Select(LRCLK)");
        I2S2_IN_D0: (112, "I2S2 Serial Data Input 0");
        I2S2_IN_D1: (113, "I2S2 Serial Data Input 1");
        I2S2_IN_D2: (114, "I2S2 Serial Data Input 2");
        I2S2_IN_D3: (115, "I2S2 Serial Data Input 3");
        I2S2_OUT_D0: (116, "I2S2 Serial Data Output 0");
        I2S2_OUT_D1: (117, "I2S2 Serial Data Output 1");
        I2S2_OUT_D2: (118, "I2S2 Serial Data Output 2");
        I2S2_OUT_D3: (119, "I2S2 Serial Data Output 3");
        RESV0: (120, "Reserved function");
        RESV1: (121, "Reserved function");
        RESV2: (122, "Reserved function");
        RESV3: (123, "Reserved function");
        RESV4: (124, "Reserved function");
        RESV5: (125, "Reserved function");
        I2C0_SCLK: (126, "I2C0 Serial Clock");
        I2C0_SDA: (127, "I2C0 Serial Data");
        I2C1_SCLK: (128, "I2C1 Serial Clock");
        I2C1_SDA: (129, "I2C1 Serial Data");
        I2C2_SCLK: (130, "I2C2 Serial Clock");
        I2C2_SDA: (131, "I2C2 Serial Data");
        CMOS_XCLK: (132, "DVP System Clock");
        CMOS_RST: (133, "DVP System Reset");
        CMOS_PWDN: (134, "DVP Power Down Mode");
        CMOS_VSYNC: (135, "DVP Vertical Sync");
        CMOS_HREF: (136, "DVP Horizontal Reference output");
        CMOS_PCLK: (137, "Pixel Clock");
        CMOS_D0: (138, "Data Bit 0");
        CMOS_D1: (139, "Data Bit 1");
        CMOS_D2: (140, "Data Bit 2");
        CMOS_D3: (141, "Data Bit 3");
        CMOS_D4: (142, "Data Bit 4");
        CMOS_D5: (143, "Data Bit 5");
        CMOS_D6: (144, "Data Bit 6");
        CMOS_D7: (145, "Data Bit 7");
        SCCB_SCLK: (146, "SCCB Serial Clock");
        SCCB_SDA: (147, "SCCB Serial Data");
        UART1_CTS: (148, "UART1 Clear To Send");
        UART1_DSR: (149, "UART1 Data Set Ready");
        UART1_DCD: (150, "UART1 Data Carrier Detect");
        UART1_RI: (151, "UART1 Ring Indicator");
        UART1_SIR_IN: (152, "UART1 Serial Infrared Input");
        UART1_DTR: (153, "UART1 Data Terminal Ready");
        UART1_RTS: (154, "UART1 Request To Send");
        UART1_OUT2: (155, "UART1 User-designated Output 2");
        UART1_OUT1: (156, "UART1 User-designated Output 1");
        UART1_SIR_OUT: (157, "UART1 Serial Infrared Output");
        UART1_BAUD: (158, "UART1 Transmit Clock Output");
        UART1_RE: (159, "UART1 Receiver Output Enable");
        UART1_DE: (160, "UART1 Driver Output Enable");
        UART1_RS485_EN: (161, "UART1 RS485 Enable");
        UART2_CTS: (162, "UART2 Clear To Send");
        UART2_DSR: (163, "UART2 Data Set Ready");
        UART2_DCD: (164, "UART2 Data Carrier Detect");
        UART2_RI: (165, "UART2 Ring Indicator");
        UART2_SIR_IN: (166, "UART2 Serial Infrared Input");
        UART2_DTR: (167, "UART2 Data Terminal Ready");
        UART2_RTS: (168, "UART2 Request To Send");
        UART2_OUT2: (169, "UART2 User-designated Output 2");
        UART2_OUT1: (170, "UART2 User-designated Output 1");
        UART2_SIR_OUT: (171, "UART2 Serial Infrared Output");
        UART2_BAUD: (172, "UART2 Transmit Clock Output");
        UART2_RE: (173, "UART2 Receiver Output Enable");
        UART2_DE: (174, "UART2 Driver Output Enable");
        UART2_RS485_EN: (175, "UART2 RS485 Enable");
        UART3_CTS: (176, "UART3 Clear To Send");
        UART3_DSR: (177, "UART3 Data Set Ready");
        UART3_DCD: (178, "UART3 Data Carrier Detect");
        UART3_RI: (179, "UART3 Ring Indicator");
        UART3_SIR_IN: (180, "UART3 Serial Infrared Input");
        UART3_DTR: (181, "UART3 Data Terminal Ready");
        UART3_RTS: (182, "UART3 Request To Send");
        UART3_OUT2: (183, "UART3 User-designated Output 2");
        UART3_OUT1: (184, "UART3 User-designated Output 1");
        UART3_SIR_OUT: (185, "UART3 Serial Infrared Output");
        UART3_BAUD: (186, "UART3 Transmit Clock Output");
        UART3_RE: (187, "UART3 Receiver Output Enable");
        UART3_DE: (188, "UART3 Driver Output Enable");
        UART3_RS485_EN: (189, "UART3 RS485 Enable");
        TIMER0_TOGGLE1: (190, "TIMER0 Toggle Output 1");
        TIMER0_TOGGLE2: (191, "TIMER0 Toggle Output 2");
        TIMER0_TOGGLE3: (192, "TIMER0 Toggle Output 3");
        TIMER0_TOGGLE4: (193, "TIMER0 Toggle Output 4");
        TIMER1_TOGGLE1: (194, "TIMER1 Toggle Output 1");
        TIMER1_TOGGLE2: (195, "TIMER1 Toggle Output 2");
        TIMER1_TOGGLE3: (196, "TIMER1 Toggle Output 3");
        TIMER1_TOGGLE4: (197, "TIMER1 Toggle Output 4");
        TIMER2_TOGGLE1: (198, "TIMER2 Toggle Output 1");
        TIMER2_TOGGLE2: (199, "TIMER2 Toggle Output 2");
        TIMER2_TOGGLE3: (200, "TIMER2 Toggle Output 3");
        TIMER2_TOGGLE4: (201, "TIMER2 Toggle Output 4");
        CLK_SPI2: (202, "Clock SPI2");
        CLK_I2C2: (203, "Clock I2C2");
        INTERNAL0: (204, "Internal function signal 0");
        INTERNAL1: (205, "Internal function signal 1");
        INTERNAL2: (206, "Internal function signal 2");
        INTERNAL3: (207, "Internal function signal 3");
        INTERNAL4: (208, "Internal function signal 4");
        INTERNAL5: (209, "Internal function signal 5");
        INTERNAL6: (210, "Internal function signal 6");
        INTERNAL7: (211, "Internal function signal 7");
        INTERNAL8: (212, "Internal function signal 8");
        INTERNAL9: (213, "Internal function signal 9");
        INTERNAL10: (214, "Internal function signal 10");
        INTERNAL11: (215, "Internal function signal 11");
        INTERNAL12: (216, "Internal function signal 12");
        INTERNAL13: (217, "Internal function signal 13");
        INTERNAL14: (218, "Internal function signal 14");
        INTERNAL15: (219, "Internal function signal 15");
        INTERNAL16: (220, "Internal function signal 16");
        INTERNAL17: (221, "Internal function signal 17");
        CONSTANT: (222, "Constant function");
        INTERNAL18: (223, "Internal function signal 18");
        DEBUG0: (224, "Debug function 0");
        DEBUG1: (225, "Debug function 1");
        DEBUG2: (226, "Debug function 2");
        DEBUG3: (227, "Debug function 3");
        DEBUG4: (228, "Debug function 4");
        DEBUG5: (229, "Debug function 5");
        DEBUG6: (230, "Debug function 6");
        DEBUG7: (231, "Debug function 7");
        DEBUG8: (232, "Debug function 8");
        DEBUG9: (233, "Debug function 9");
        DEBUG10: (234, "Debug function 10");
        DEBUG11: (235, "Debug function 11");
        DEBUG12: (236, "Debug function 12");
        DEBUG13: (237, "Debug function 13");
        DEBUG14: (238, "Debug function 14");
        DEBUG15: (239, "Debug function 15");
        DEBUG16: (240, "Debug function 16");
        DEBUG17: (241, "Debug function 17");
        DEBUG18: (242, "Debug function 18");
        DEBUG19: (243, "Debug function 19");
        DEBUG20: (244, "Debug function 20");
        DEBUG21: (245, "Debug function 21");
        DEBUG22: (246, "Debug function 22");
        DEBUG23: (247, "Debug function 23");
        DEBUG24: (248, "Debug function 24");
        DEBUG25: (249, "Debug function 25");
        DEBUG26: (250, "Debug function 26");
        DEBUG27: (251, "Debug function 27");
        DEBUG28: (252, "Debug function 28");
        DEBUG29: (253, "Debug function 29");
        DEBUG30: (254, "Debug function 30");
        DEBUG31: (255, "Debug function 31");
    }
}
