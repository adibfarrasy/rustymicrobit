use fugit::{Duration, Instant};
use microbit::{hal::Rtc, pac::RTC0};

type TickInstant = Instant<u64, 1, 32768>;
type TickDuration = Duration<u64, 1, 32768>;

pub struct Timer<'a> {
    end_time: TickInstant,
    ticker: &'a Ticker,
}

impl<'a> Timer<'a> {
    pub fn new(duration: TickDuration, ticker: &'a Ticker) -> Self {
        Self {
            end_time: ticker.now() + duration,
            ticker,
        }
    }

    pub fn is_ready(&self) -> bool {
        self.ticker.now() >= self.end_time
    }
}

pub struct Ticker {
    rtc: Rtc<RTC0>,
}

impl Ticker {
    pub fn new(rtc0: RTC0) -> Self {
        let rtc = Rtc::new(rtc0, 0).unwrap();
        rtc.enable_counter();
        Self { rtc }
    }

    pub fn now(&self) -> TickInstant {
        TickInstant::from_ticks(self.rtc.get_counter() as u64)
    }
}
