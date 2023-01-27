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

pub type ImageFormat = pac::dvp::dvp_cfg::FORMAT_A;

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

    pub fn set_xclk_rate(&self, xclk_rate: Hertz, clock: &Clocks) -> Hertz {
        let apb1_clk = clock.apb1().0;
        let period = if apb1_clk > xclk_rate.0 * 2 {
            apb1_clk / xclk_rate.0 / 2 - 1 as u32
        } else {
            0
        };

        let period = period.min(255);
        unsafe {
            self.dvp
                .cmos_cfg
                .modify(|_, w| w.clk_div().bits(period as u8).clk_enable().set_bit())
        }

        self.reset();
        Hertz(apb1_clk / (period + 1) / 2)
    }

    pub fn set_image_size(&self, burst_mode: bool, width: u16, height: u16) {
        use pac::dvp::axi::GM_MLEN_A::*;
        let burst_num = if burst_mode {
            self.dvp
                .dvp_cfg
                .modify(|_, w| w.burst_size_4beats().set_bit());
            self.dvp.axi.modify(|_, w| w.gm_mlen().variant(BYTE4));
            width / 8 / 4
        } else {
            self.dvp
                .dvp_cfg
                .modify(|_, w| w.burst_size_4beats().clear_bit());
            self.dvp.axi.modify(|_, w| w.gm_mlen().variant(BYTE1));
            width / 8 / 1
        };

        let burst_num = burst_num.min(255).max(0) as u8;

        unsafe {
            self.dvp
                .dvp_cfg
                .modify(|_, w| w.href_burst_num().bits(burst_num).line_num().bits(height))
        }
    }

    pub fn set_image_format(&self, format: ImageFormat) {
        self.dvp.dvp_cfg.modify(|_, w| w.format().variant(format));
    }

    pub fn set_display_addr(&self, addr: Option<*mut u32>) {
        unsafe {
            if let Some(addr) = addr {
                self.dvp
                    .rgb_addr
                    .write(|w| w.bits((addr as usize & 0xffff_ffff) as u32));
                self.dvp
                    .dvp_cfg
                    .modify(|_, w| w.display_output_enable().set_bit());
            } else {
                self.dvp
                    .dvp_cfg
                    .modify(|_, w| w.display_output_enable().clear_bit());
            }
        }
    }

    pub fn set_auto(&self, status: bool) {
        self.dvp.dvp_cfg.modify(|_, w| w.auto_enable().bit(status));
    }

    pub fn get_image(&self) {
        while !self.dvp.sts.read().frame_start().bit() {
            // IDLE
        }
        self.dvp
            .sts
            .write(|w| w.frame_start().set_bit().frame_start_we().set_bit());
        while !self.dvp.sts.read().frame_start().bit() {
            // IDLE
        }
        self.dvp.sts.write(|w| {
            w.frame_finish()
                .set_bit()
                .frame_finish_we()
                .set_bit()
                .frame_start()
                .set_bit()
                .frame_start_we()
                .set_bit()
                .dvp_en()
                .set_bit()
                .dvp_en_we()
                .set_bit()
        });
        while !self.dvp.sts.read().frame_finish().bit() {
            // IDLE
        }
    }
}
