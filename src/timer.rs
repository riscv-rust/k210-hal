use crate::pac::TIMER0;
use crate::sysctl::{self, APB0};
use crate::time::Hertz;

macro_rules! timer {
    ($Timer: ident, $timer: ident, $TIMER: ident) => {
        pub struct $Timer<'a> {
            timer: &'a $TIMER,
            channel_id: usize,
            resolution: u32,
        }

        impl<'a> $Timer<'a> {
            pub fn new(
                timer: &'a $TIMER,
                channel_id: usize,
                $timer: &mut sysctl::$Timer,
                apb0: &mut APB0,
            ) -> Self {
                $timer.enable(apb0);
                timer.channel[channel_id]
                    .control
                    .modify(|_, w| w.enable().set_bit().interrupt().set_bit().mode().user());
                Self {
                    timer,
                    channel_id,
                    resolution: $timer.get_frequency().0,
                }
            }
        }

        impl<'a> embedded_hal::timer::Periodic for $Timer<'a> {}

        impl<'a> embedded_hal::timer::CountDown for $Timer<'a> {
            type Error = ();
            type Time = Hertz;

            fn try_start<T>(&mut self, count: T) -> Result<(), Self::Error>
            where
                T: Into<Self::Time>,
            {
                self.timer.channel[self.channel_id]
                    .control
                    .modify(|_, w| w.enable().clear_bit());
                let count = count.into().0;
                let ticks = self.resolution / count;
                unsafe {
                    self.timer.channel[self.channel_id]
                        .current_value
                        .write(|w| w.bits(0));
                    self.timer.channel[self.channel_id]
                        .load_count
                        .write(|w| w.bits(ticks));
                }
                self.timer.channel[self.channel_id]
                    .control
                    .modify(|_, w| w.enable().set_bit());
                Ok(())
            }

            fn try_wait(&mut self) -> nb::Result<(), Self::Error> {
                if self.timer.raw_intr_stat.read().bits() & (1 << self.channel_id) == 0 {
                    Err(nb::Error::WouldBlock)
                } else {
                    let _ = self.timer.channel[self.channel_id].eoi.read().bits();
                    Ok(())
                }
            }
        }
    };
}

timer!(Timer0, timer0, TIMER0);
// todo: timer!(Timer1, timer1, TIMER1);
// todo: timer!(Timer2, timer2, TIMER2);
