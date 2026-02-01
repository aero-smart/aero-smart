use embassy_stm32::{
    exti::ExtiInput,
    peripherals::TIM3,
    timer::qei::{Direction, Qei},
};

pub type QeiChan = embassy_sync::channel::Channel<
    embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex,
    (u16, bool, bool),
    2,
>;

pub struct QeiOperations<'a> {
    pub qei: Qei<'a, TIM3>,
    pub press_btn: ExtiInput<'a>,

    pub cnt_cache: u16,
    pub dir_cache: bool,
}

impl<'a> QeiOperations<'a> {
    pub fn new(qei: Qei<'a, TIM3>, press_btn: ExtiInput<'a>) -> Self {
        QeiOperations {
            qei,
            press_btn,
            cnt_cache: 0,
            dir_cache: true,
        }
    }

    pub fn read_position(&mut self) -> (u16, bool, bool) {
        let mut changed = false;
        let position = self.qei.count();
        if position != self.cnt_cache {
            changed = true;
        }
        self.cnt_cache = position;
        let direction = match self.qei.read_direction() {
            Direction::Upcounting => true,
            Direction::Downcounting => false,
        };
        if direction != self.dir_cache {
            changed = true;
        }
        self.dir_cache = direction;
        (position, direction, changed)
    }
}
