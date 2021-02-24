//! (TODO) Direct Memory Access Controller (DMAC)
use crate::pac::DMAC;
use crate::{pac, sysctl};

pub fn dmac_id() -> u64 {
    unsafe { (*pac::DMAC::ptr()).id.read().bits() }
}

pub fn dmac_version() -> u64 {
    unsafe { (*pac::DMAC::ptr()).compver.read().bits() }
}

pub use crate::pac::dmac::channel::ctl::SMS_A as Sms;

pub use crate::pac::dmac::channel::ctl::SINC_A as Inc;

pub use crate::pac::dmac::channel::ctl::SRC_TR_WIDTH_A as TrWidth;

pub use crate::pac::dmac::channel::ctl::SRC_MSIZE_A as Msize;

pub use crate::pac::dmac::channel::cfg::TT_FC_A as FlowControl;

pub use crate::pac::dmac::channel::cfg::HS_SEL_SRC_A as HandshakeSrcSel;

pub use crate::pac::dmac::channel::cfg::HS_SEL_DST_A as HandshakeDstSel;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum MemType {
    Peripheral,
    Memory,
}

fn is_memory(addr: u64) -> MemType {
    let mem_len = 6 * 1024 * 1024;
    let mem_no_cache_len = 8 * 1024 * 1024;
    if ((addr >= 0x8000_0000) && (addr < 0x8000_0000 + mem_len))
        || ((addr >= 0x40000000) && (addr < 0x40000000 + mem_no_cache_len))
        || (addr == 0x50450040)
    {
        MemType::Memory
    } else {
        MemType::Peripheral
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum DmacSrcDstSelect {
    Source,
    Destination,
    SourceDestination,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum DmacError {
    ChannelBusy,
}
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum DmacChannel {
    Channel0 = 0,
    Channel1 = 1,
    Channel2 = 2,
    Channel3 = 3,
    Channel4 = 4,
    Channel5 = 5,
}

pub trait DmacExt {
    fn constrain(self) -> Dmac;
}

impl DmacExt for pac::DMAC {
    fn constrain(self) -> Dmac {
        Dmac { dmac: self }
    }
}

pub struct Dmac {
    dmac: DMAC,
}

impl Dmac {
    pub fn init(&mut self) {
        sysctl::clk_en_peri().modify(|_, w| w.dma_clk_en().set_bit());

        /* reset dmac */
        self.reset();

        /* clear common register interrupt */
        self.clear_common_interrupt();

        /* disable dmac and disable interrupt */
        self.disable();

        while self.dmac.cfg.read().bits() != 0x00 {
            // IDLE
        }

        self.dmac.chen.modify(|_, w| {
            w.ch1_en()
                .clear_bit()
                .ch2_en()
                .clear_bit()
                .ch3_en()
                .clear_bit()
                .ch4_en()
                .clear_bit()
                .ch5_en()
                .clear_bit()
        });

        for ch in &self.dmac.channel {
            unsafe { ch.intclear.write(|w| w.bits(0xffff_ffff)) }
        }

        self.enable();
    }

    /// Get DMAC ID
    pub fn id(&self) -> u64 {
        self.dmac.id.read().bits()
    }

    /// Get DMAC Version
    pub fn version(&self) -> u64 {
        self.dmac.compver.read().bits()
    }

    /// Enable DMAC peripheral
    fn enable(&mut self) {
        self.dmac
            .cfg
            .modify(|_, w| w.dmac_en().set_bit().int_en().set_bit())
    }

    /// Disable DMAC peripheral
    fn disable(&mut self) {
        self.dmac
            .cfg
            .modify(|_, w| w.dmac_en().clear_bit().int_en().clear_bit())
    }

    ///
    fn reset(&mut self) {
        self.dmac.reset.write(|w| w.rst().set_bit());
        while self.dmac.reset.read().rst().bit() {
            // IDLE
        }
    }

    fn clear_common_interrupt(&mut self) {
        /* clear common register interrupt (0b1_0000_1111 = 0x10f) */
        unsafe { self.dmac.com_intclear.write(|w| w.bits(0x10f)) }
    }

    /// Enable a DMA Channel
    pub fn channel_enable(&mut self, channel: DmacChannel) {
        use DmacChannel::*;
        match channel {
            Channel0 => self
                .dmac
                .chen
                .modify(|_, w| w.ch1_en().set_bit().ch1_en_we().set_bit()),
            Channel1 => self
                .dmac
                .chen
                .modify(|_, w| w.ch2_en().set_bit().ch2_en_we().set_bit()),
            Channel2 => self
                .dmac
                .chen
                .modify(|_, w| w.ch3_en().set_bit().ch3_en_we().set_bit()),
            Channel3 => self
                .dmac
                .chen
                .modify(|_, w| w.ch4_en().set_bit().ch4_en_we().set_bit()),
            Channel4 => self
                .dmac
                .chen
                .modify(|_, w| w.ch5_en().set_bit().ch5_en_we().set_bit()),
            Channel5 => self
                .dmac
                .chen
                .modify(|_, w| w.ch6_en().set_bit().ch6_en_we().set_bit()),
        }
    }

    /// Disable a DMA Channel
    pub fn channel_disable(&mut self, channel: DmacChannel) {
        use DmacChannel::*;
        match channel {
            Channel0 => self
                .dmac
                .chen
                .modify(|_, w| w.ch1_en().clear_bit().ch1_en_we().set_bit()),
            Channel1 => self
                .dmac
                .chen
                .modify(|_, w| w.ch2_en().clear_bit().ch2_en_we().set_bit()),
            Channel2 => self
                .dmac
                .chen
                .modify(|_, w| w.ch3_en().clear_bit().ch3_en_we().set_bit()),
            Channel3 => self
                .dmac
                .chen
                .modify(|_, w| w.ch4_en().clear_bit().ch4_en_we().set_bit()),
            Channel4 => self
                .dmac
                .chen
                .modify(|_, w| w.ch5_en().clear_bit().ch5_en_we().set_bit()),
            Channel5 => self
                .dmac
                .chen
                .modify(|_, w| w.ch6_en().clear_bit().ch6_en_we().set_bit()),
        }
    }

    pub fn is_channel_busy(&self, channel: DmacChannel) -> bool {
        use DmacChannel::*;
        match channel {
            Channel0 => self.dmac.chen.read().ch1_en().bit(),
            Channel1 => self.dmac.chen.read().ch2_en().bit(),
            Channel2 => self.dmac.chen.read().ch3_en().bit(),
            Channel3 => self.dmac.chen.read().ch4_en().bit(),
            Channel4 => self.dmac.chen.read().ch5_en().bit(),
            Channel5 => self.dmac.chen.read().ch6_en().bit(),
        }
    }

    pub fn set_list_master_select(
        &mut self,
        channel: DmacChannel,
        sd: DmacSrcDstSelect,
        sms: Sms,
    ) -> Result<(), DmacError> {
        if self.is_channel_busy(channel) {
            Err(DmacError::ChannelBusy)
        } else {
            use DmacSrcDstSelect::*;

            if sd != Destination {
                self.dmac.channel[channel as usize]
                    .ctl
                    .modify(|_, w| w.sms().variant(sms));
            }

            if sd != Source {
                self.dmac.channel[channel as usize]
                    .ctl
                    .modify(|_, w| w.dms().variant(sms));
            }

            Ok(())
        }
    }

    fn channel_interrupt_clear(&mut self, channel: DmacChannel) {
        unsafe {
            self.dmac.channel[channel as usize]
                .intclear
                .write(|w| w.bits(0xffff_ffff))
        }
    }

    fn is_channel_idle(&self, channel: DmacChannel) -> bool {
        (self.dmac.chen.read().bits() >> channel as u8) & 0x1 == 0
    }

    fn wait_idle(&mut self, channel: DmacChannel) {
        while !self.is_channel_idle(channel) {
            // IDLE
        }
        self.channel_interrupt_clear(channel);
    }

    fn set_channel_param(
        &mut self,
        channel: DmacChannel,
        src: u64,
        dst: u64,
        src_inc: Inc,
        dst_inc: Inc,
        trans_width: TrWidth,
        burst_size: Msize,
        block_size: u32,
    ) {
        let ch = &self.dmac.channel[channel as usize];
        let mem_type = (is_memory(src), is_memory(dst));
        let flow_control = {
            use FlowControl::*;
            use MemType::*;

            match mem_type {
                (Memory, Memory) => MEM2MEM_DMA,
                (Memory, Peripheral) => MEM2PRF_DMA,
                (Peripheral, Memory) => PRF2MEM_DMA,
                (Peripheral, Peripheral) => PRF2PRF_DMA,
            }
        };

        /*
         * cfg register must configure before ts_block and
         * sar dar register
         */
        ch.cfg.modify(|_, w| unsafe {
            w.tt_fc()
                .variant(flow_control)
                .hs_sel_src()
                .variant(match mem_type.0 {
                    MemType::Memory => HandshakeSrcSel::SOFTWARE,
                    MemType::Peripheral => HandshakeSrcSel::HARDWARE,
                })
                .hs_sel_dst()
                .variant(match mem_type.1 {
                    MemType::Memory => HandshakeDstSel::SOFTWARE,
                    MemType::Peripheral => HandshakeDstSel::HARDWARE,
                })
                .src_per()
                .bits(channel as u8)
                .dst_per()
                .bits(channel as u8)
                .src_multblk_type()
                .bits(0x00)
                .dst_multblk_type()
                .bits(0x00)
        });

        unsafe {
            ch.sar.write(|w| w.bits(src));
            ch.dar.write(|w| w.bits(dst));
        }

        ch.ctl.modify(|_, w| {
            w.sms()
                .variant(Sms::AXI_MASTER_1)
                .dms()
                .variant(Sms::AXI_MASTER_2)
                /* set address increment */
                .sinc()
                .variant(src_inc)
                .dinc()
                .variant(dst_inc)
                /* set transfer width */
                .src_tr_width()
                .variant(trans_width)
                .dst_tr_width()
                .variant(trans_width)
                /* set burst size */
                .src_msize()
                .variant(burst_size)
                .dst_msize()
                .variant(burst_size)
        });

        ch.block_ts
            .write(|w| unsafe { w.block_ts().bits(block_size - 1) })
    }

    pub fn wait_done(&mut self, channel: DmacChannel) {
        self.wait_idle(channel);
    }

    pub fn set_single_mode(
        &mut self,
        channel: DmacChannel,
        src: u64,
        dst: u64,
        src_inc: Inc,
        dst_inc: Inc,
        trans_width: TrWidth,
        burst_size: Msize,
        block_size: u32,
    ) {
        self.channel_interrupt_clear(channel);
        self.channel_disable(channel);
        self.wait_idle(channel);
        self.set_channel_param(
            channel,
            src,
            dst,
            src_inc,
            dst_inc,
            trans_width,
            burst_size,
            block_size,
        );
        self.enable();
        self.channel_enable(channel);
    }
}
