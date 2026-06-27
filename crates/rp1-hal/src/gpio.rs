use core::marker::PhantomData;

pub struct Gpio {
    _private: (),
}

pub struct Pin<const N: u8> {
    _private: (),
}

pub struct Input;
pub struct Output;
pub struct Function<const F: u8>;

pub struct ConfiguredPin<const N: u8, Mode> {
    _mode: PhantomData<Mode>,
}

impl Gpio {
    pub(crate) const unsafe fn new() -> Self {
        Self { _private: () }
    }

    pub fn pin<const N: u8>(&mut self) -> Pin<N> {
        Pin { _private: () }
    }
}

impl<const N: u8> Pin<N> {
    pub fn into_input(self) -> ConfiguredPin<N, Input> {
        // TODO: write actual RP1 GPIO function/select register once offsets are verified.
        ConfiguredPin { _mode: PhantomData }
    }

    pub fn into_output(self) -> ConfiguredPin<N, Output> {
        // TODO: write actual RP1 GPIO function/select register once offsets are verified.
        ConfiguredPin { _mode: PhantomData }
    }

    pub fn into_function<const F: u8>(self) -> ConfiguredPin<N, Function<F>> {
        // TODO: write actual function select once RP1 GPIO register offsets are verified.
        ConfiguredPin { _mode: PhantomData }
    }
}

impl<const N: u8> ConfiguredPin<N, Output> {
    pub fn set_high(&mut self) {
        // TODO: actual MMIO set.
    }

    pub fn set_low(&mut self) {
        // TODO: actual MMIO clear.
    }

    pub fn toggle(&mut self) {
        // TODO: read/write actual state.
    }
}
