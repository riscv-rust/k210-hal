use core::marker::PhantomData;

pub struct ExternalPin0(PhantomData<*const ()>);
pub struct ExternalPin1(PhantomData<*const ()>);
pub struct ExternalPin2(PhantomData<*const ()>);
pub struct ExternalPin3(PhantomData<*const ()>);
pub struct ExternalPin4(PhantomData<*const ()>);
pub struct ExternalPin5(PhantomData<*const ()>);
pub struct ExternalPin6(PhantomData<*const ()>);
pub struct ExternalPin7(PhantomData<*const ()>);
pub struct ExternalPin8(PhantomData<*const ()>);
pub struct ExternalPin9(PhantomData<*const ()>);
pub struct ExternalPin10(PhantomData<*const ()>);
pub struct ExternalPin11(PhantomData<*const ()>);
pub struct ExternalPin12(PhantomData<*const ()>);
pub struct ExternalPin13(PhantomData<*const ()>);
pub struct ExternalPin14(PhantomData<*const ()>);
pub struct ExternalPin15(PhantomData<*const ()>);
pub struct ExternalPin16(PhantomData<*const ()>);
pub struct ExternalPin17(PhantomData<*const ()>);
pub struct ExternalPin18(PhantomData<*const ()>);
pub struct ExternalPin19(PhantomData<*const ()>);
pub struct ExternalPin20(PhantomData<*const ()>);
pub struct ExternalPin21(PhantomData<*const ()>);
pub struct ExternalPin22(PhantomData<*const ()>);
pub struct ExternalPin23(PhantomData<*const ()>);
pub struct ExternalPin24(PhantomData<*const ()>);
pub struct ExternalPin25(PhantomData<*const ()>);
pub struct ExternalPin26(PhantomData<*const ()>);
pub struct ExternalPin27(PhantomData<*const ()>);
pub struct ExternalPin28(PhantomData<*const ()>);
pub struct ExternalPin29(PhantomData<*const ()>);
pub struct ExternalPin30(PhantomData<*const ()>);
pub struct ExternalPin31(PhantomData<*const ()>);
pub struct ExternalPin32(PhantomData<*const ()>);
pub struct ExternalPin33(PhantomData<*const ()>);
pub struct ExternalPin34(PhantomData<*const ()>);
pub struct ExternalPin35(PhantomData<*const ()>);
pub struct ExternalPin36(PhantomData<*const ()>);
pub struct ExternalPin37(PhantomData<*const ()>);
pub struct ExternalPin38(PhantomData<*const ()>);
pub struct ExternalPin39(PhantomData<*const ()>);
pub struct ExternalPin40(PhantomData<*const ()>);
pub struct ExternalPin41(PhantomData<*const ()>);
pub struct ExternalPin42(PhantomData<*const ()>);
pub struct ExternalPin43(PhantomData<*const ()>);
pub struct ExternalPin44(PhantomData<*const ()>);
pub struct ExternalPin45(PhantomData<*const ()>);
pub struct ExternalPin46(PhantomData<*const ()>);
pub struct ExternalPin47(PhantomData<*const ()>);

mod closed_trait {
    pub trait ExternalPin {
        const INDEX: usize;
    }
}
pub(crate) use closed_trait::ExternalPin;

impl ExternalPin for ExternalPin0 { const INDEX: usize = 0; }
impl ExternalPin for ExternalPin1 { const INDEX: usize = 1; }
impl ExternalPin for ExternalPin2 { const INDEX: usize = 2; }
impl ExternalPin for ExternalPin3 { const INDEX: usize = 3; }
impl ExternalPin for ExternalPin4 { const INDEX: usize = 4; }
impl ExternalPin for ExternalPin5 { const INDEX: usize = 5; }
impl ExternalPin for ExternalPin6 { const INDEX: usize = 6; }
impl ExternalPin for ExternalPin7 { const INDEX: usize = 7; }
impl ExternalPin for ExternalPin8 { const INDEX: usize = 8; }
impl ExternalPin for ExternalPin9 { const INDEX: usize = 9; }
impl ExternalPin for ExternalPin10 { const INDEX: usize = 10; }
impl ExternalPin for ExternalPin11 { const INDEX: usize = 11; }
impl ExternalPin for ExternalPin12 { const INDEX: usize = 12; }
impl ExternalPin for ExternalPin13 { const INDEX: usize = 13; }
impl ExternalPin for ExternalPin14 { const INDEX: usize = 14; }
impl ExternalPin for ExternalPin15 { const INDEX: usize = 15; }
impl ExternalPin for ExternalPin16 { const INDEX: usize = 16; }
impl ExternalPin for ExternalPin17 { const INDEX: usize = 17; }
impl ExternalPin for ExternalPin18 { const INDEX: usize = 18; }
impl ExternalPin for ExternalPin19 { const INDEX: usize = 19; }
impl ExternalPin for ExternalPin20 { const INDEX: usize = 20; }
impl ExternalPin for ExternalPin21 { const INDEX: usize = 21; }
impl ExternalPin for ExternalPin22 { const INDEX: usize = 22; }
impl ExternalPin for ExternalPin23 { const INDEX: usize = 23; }
impl ExternalPin for ExternalPin24 { const INDEX: usize = 24; }
impl ExternalPin for ExternalPin25 { const INDEX: usize = 25; }
impl ExternalPin for ExternalPin26 { const INDEX: usize = 26; }
impl ExternalPin for ExternalPin27 { const INDEX: usize = 27; }
impl ExternalPin for ExternalPin28 { const INDEX: usize = 28; }
impl ExternalPin for ExternalPin29 { const INDEX: usize = 29; }
impl ExternalPin for ExternalPin30 { const INDEX: usize = 30; }
impl ExternalPin for ExternalPin31 { const INDEX: usize = 31; }
impl ExternalPin for ExternalPin32 { const INDEX: usize = 32; }
impl ExternalPin for ExternalPin33 { const INDEX: usize = 33; }
impl ExternalPin for ExternalPin34 { const INDEX: usize = 34; }
impl ExternalPin for ExternalPin35 { const INDEX: usize = 35; }
impl ExternalPin for ExternalPin36 { const INDEX: usize = 36; }
impl ExternalPin for ExternalPin37 { const INDEX: usize = 37; }
impl ExternalPin for ExternalPin38 { const INDEX: usize = 38; }
impl ExternalPin for ExternalPin39 { const INDEX: usize = 39; }
impl ExternalPin for ExternalPin40 { const INDEX: usize = 40; }
impl ExternalPin for ExternalPin41 { const INDEX: usize = 41; }
impl ExternalPin for ExternalPin42 { const INDEX: usize = 42; }
impl ExternalPin for ExternalPin43 { const INDEX: usize = 43; }
impl ExternalPin for ExternalPin44 { const INDEX: usize = 44; }
impl ExternalPin for ExternalPin45 { const INDEX: usize = 45; }
impl ExternalPin for ExternalPin46 { const INDEX: usize = 46; }
impl ExternalPin for ExternalPin47 { const INDEX: usize = 47; }


pub struct ExternalPins {
    pub pin0: ExternalPin0,
    pub pin1: ExternalPin1,
    pub pin2: ExternalPin2,
    pub pin3: ExternalPin3,
    pub pin4: ExternalPin4,
    pub pin5: ExternalPin5,
    pub pin6: ExternalPin6,
    pub pin7: ExternalPin7,
    pub pin8: ExternalPin8,
    pub pin9: ExternalPin9,
    pub pin10: ExternalPin10,
    pub pin11: ExternalPin11,
    pub pin12: ExternalPin12,
    pub pin13: ExternalPin13,
    pub pin14: ExternalPin14,
    pub pin15: ExternalPin15,
    pub pin16: ExternalPin16,
    pub pin17: ExternalPin17,
    pub pin18: ExternalPin18,
    pub pin19: ExternalPin19,
    pub pin20: ExternalPin20,
    pub pin21: ExternalPin21,
    pub pin22: ExternalPin22,
    pub pin23: ExternalPin23,
    pub pin24: ExternalPin24,
    pub pin25: ExternalPin25,
    pub pin26: ExternalPin26,
    pub pin27: ExternalPin27,
    pub pin28: ExternalPin28,
    pub pin29: ExternalPin29,
    pub pin30: ExternalPin30,
    pub pin31: ExternalPin31,
    pub pin32: ExternalPin32,
    pub pin33: ExternalPin33,
    pub pin34: ExternalPin34,
    pub pin35: ExternalPin35,
    pub pin36: ExternalPin36,
    pub pin37: ExternalPin37,
    pub pin38: ExternalPin38,
    pub pin39: ExternalPin39,
    pub pin40: ExternalPin40,
    pub pin41: ExternalPin41,
    pub pin42: ExternalPin42,
    pub pin43: ExternalPin43,
    pub pin44: ExternalPin44,
    pub pin45: ExternalPin45,
    pub pin46: ExternalPin46,
    pub pin47: ExternalPin47,
}

impl ExternalPins {
    pub(crate) fn new() -> Self {
        Self {
            pin0: ExternalPin0(PhantomData),
            pin1: ExternalPin1(PhantomData),
            pin2: ExternalPin2(PhantomData),
            pin3: ExternalPin3(PhantomData),
            pin4: ExternalPin4(PhantomData),
            pin5: ExternalPin5(PhantomData),
            pin6: ExternalPin6(PhantomData),
            pin7: ExternalPin7(PhantomData),
            pin8: ExternalPin8(PhantomData),
            pin9: ExternalPin9(PhantomData),
            pin10: ExternalPin10(PhantomData),
            pin11: ExternalPin11(PhantomData),
            pin12: ExternalPin12(PhantomData),
            pin13: ExternalPin13(PhantomData),
            pin14: ExternalPin14(PhantomData),
            pin15: ExternalPin15(PhantomData),
            pin16: ExternalPin16(PhantomData),
            pin17: ExternalPin17(PhantomData),
            pin18: ExternalPin18(PhantomData),
            pin19: ExternalPin19(PhantomData),
            pin20: ExternalPin20(PhantomData),
            pin21: ExternalPin21(PhantomData),
            pin22: ExternalPin22(PhantomData),
            pin23: ExternalPin23(PhantomData),
            pin24: ExternalPin24(PhantomData),
            pin25: ExternalPin25(PhantomData),
            pin26: ExternalPin26(PhantomData),
            pin27: ExternalPin27(PhantomData),
            pin28: ExternalPin28(PhantomData),
            pin29: ExternalPin29(PhantomData),
            pin30: ExternalPin30(PhantomData),
            pin31: ExternalPin31(PhantomData),
            pin32: ExternalPin32(PhantomData),
            pin33: ExternalPin33(PhantomData),
            pin34: ExternalPin34(PhantomData),
            pin35: ExternalPin35(PhantomData),
            pin36: ExternalPin36(PhantomData),
            pin37: ExternalPin37(PhantomData),
            pin38: ExternalPin38(PhantomData),
            pin39: ExternalPin39(PhantomData),
            pin40: ExternalPin40(PhantomData),
            pin41: ExternalPin41(PhantomData),
            pin42: ExternalPin42(PhantomData),
            pin43: ExternalPin43(PhantomData),
            pin44: ExternalPin44(PhantomData),
            pin45: ExternalPin45(PhantomData),
            pin46: ExternalPin46(PhantomData),
            pin47: ExternalPin47(PhantomData)
        }
    }
}
