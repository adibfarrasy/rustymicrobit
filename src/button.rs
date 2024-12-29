use embedded_hal::digital::InputPin;
use fugit::ExtU64;
use microbit::hal::gpio::{Floating, Input, Pin};

use crate::{
    channel::Sender,
    time::{Ticker, Timer},
};

pub struct ButtonTask<'a> {
    pin: Pin<Input<Floating>>,
    ticker: &'a Ticker,
    direction: ButtonDirection,
    state: ButtonState<'a>,
    sender: Sender<'a, ButtonDirection>,
}

#[derive(Clone, Copy)]
pub enum ButtonDirection {
    Left,
    Right,
}

enum ButtonState<'a> {
    WairForPress,
    Debounce(Timer<'a>),
}

impl<'a> ButtonTask<'a> {
    pub fn new(
        pin: Pin<Input<Floating>>,
        ticker: &'a Ticker,
        direction: ButtonDirection,
        sender: Sender<'a, ButtonDirection>,
    ) -> Self {
        Self {
            pin,
            ticker,
            direction,
            state: ButtonState::WairForPress,
            sender,
        }
    }

    pub fn poll(&mut self) {
        match self.state {
            ButtonState::WairForPress => {
                if self.pin.is_low().unwrap() {
                    self.sender.send(self.direction);
                    self.state = ButtonState::Debounce(Timer::new(100.millis(), &self.ticker))
                }
            }
            ButtonState::Debounce(ref timer) => {
                if timer.is_ready() && self.pin.is_high().unwrap() {
                    self.state = ButtonState::WairForPress;
                }
            }
        }
    }
}
