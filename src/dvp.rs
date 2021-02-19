use crate::clock::Clocks;
use crate::sysctl;
use crate::time::Hertz;
use k210_pac as pac;

pub trait DvpExt: Sized {
    fn constrain(self) -> Dvp;
}

impl DvpExt for pac::DVP {
    fn constrain(self) -> Dvp {
        Dvp { dvp: self }
    }
}

pub struct Dvp {
    pub dvp: pac::DVP,
}

impl Dvp {
    pub fn sccb_clk_init(&self) {
        unsafe {
            self.dvp
                .sccb_cfg
                .modify(|_, w| w.scl_lcnt().bits(255).scl_hcnt().bits(255))
        }
    }

    pub fn sccb_clk_set_rate(&self, clk_rate: Hertz, clock: &Clocks) -> Hertz {
        let sccb_freq = clock.apb1();
        let period_clk_cnt = (sccb_freq.0 / clk_rate.0 / 2).max(0).min(255) as u8;
        unsafe {
            self.dvp.sccb_cfg.modify(|_, w| {
                w.scl_lcnt()
                    .bits(period_clk_cnt)
                    .scl_hcnt()
                    .bits(period_clk_cnt)
            })
        }
        return Hertz(clock.cpu().0 / period_clk_cnt as u32 / 2);
    }

    fn sccb_start_transfer(&self) {
        while self.dvp.sts.read().sccb_en().bit() {
            // IDLE
        }
        self.dvp
            .sts
            .write(|w| w.sccb_en().set_bit().sccb_en_we().set_bit());
        while self.dvp.sts.read().sccb_en().bit() {
            // IDLE
        }
    }

    pub fn sccb_send_data(&self, dev_addr: u8, reg_addr: u8, reg_data: u8) {
        use pac::dvp::sccb_cfg::BYTE_NUM_A::*;
        unsafe {
            self.dvp.sccb_cfg.modify(|_, w| w.byte_num().variant(NUM3));
            self.dvp.sccb_ctl.write(|w| {
                w.device_address()
                    .bits(dev_addr | 1)
                    .reg_address()
                    .bits(reg_addr)
                    .wdata_byte0()
                    .bits(reg_data)
            })
        }
        self.sccb_start_transfer();
    }

    pub fn sccb_receive_data(&self, dev_addr: u8, reg_addr: u8) -> u8 {
        use pac::dvp::sccb_cfg::BYTE_NUM_A::*;
        unsafe {
            self.dvp.sccb_cfg.modify(|_, w| w.byte_num().variant(NUM2));
            self.dvp.sccb_ctl.write(|w| {
                w.device_address()
                    .bits(dev_addr | 1)
                    .reg_address()
                    .bits(reg_addr as u8)
            });
        }
        self.sccb_start_transfer();
        unsafe {
            self.dvp
                .sccb_ctl
                .write(|w| w.device_address().bits(dev_addr));
        }
        self.sccb_start_transfer();
        self.dvp.sccb_cfg.read().rdata().bits()
    }

    pub fn reset(&self) {
        self.dvp.cmos_cfg.modify(|_, w| w.power_down().set_bit());
        self.dvp.cmos_cfg.modify(|_, w| w.power_down().clear_bit());
        self.dvp.cmos_cfg.modify(|_, w| w.reset().clear_bit());
        self.dvp.cmos_cfg.modify(|_, w| w.reset().set_bit());
    }

    pub fn init(&self) {
        // Consider borrowing i.s.o. using global instance of sysctl?
        sysctl::clk_en_peri().modify(|_, w| w.dvp_clk_en().set_bit());
        sysctl::peri_reset().modify(|_, w| w.dvp_reset().set_bit());
        sysctl::peri_reset().modify(|_, w| w.dvp_reset().clear_bit());

        unsafe {
            self.dvp
                .cmos_cfg
                .modify(|_, w| w.clk_div().bits(3).clk_enable().set_bit());
        }

        self.sccb_clk_init();
        self.reset();
    }
}
