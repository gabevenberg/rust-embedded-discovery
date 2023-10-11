#![deny(unsafe_code)]
#![no_main]
#![no_std]

use cortex_m_rt::entry;
use microbit::board::Board;
use microbit::display::blocking::Display;
use microbit::hal::timer::Timer;
use panic_halt as _;

const DELAY: u32 = 32;

#[derive(Debug, Default)]
enum Direction {
    #[default]
    Left,
    Down,
    Right,
    Up,
}

#[derive(Debug, Default)]
struct State {
    direction: Direction,
    position: u8,
}

impl State {
    fn advance(&mut self) {
        if self.position >= 4 {
            match self.direction {
                Direction::Left => {
                    self.direction = Direction::Down;
                }
                Direction::Down => {
                    self.direction = Direction::Right;
                }
                Direction::Right => {
                    self.direction = Direction::Up;
                }
                Direction::Up => {
                    self.direction = Direction::Left;
                }
            }
            self.position = 0;
        }
        self.position += 1
    }

    fn x(&self) -> usize {
        match self.direction {
            Direction::Left => 0,
            Direction::Down => self.position as usize,
            Direction::Right => 4,
            Direction::Up => 4 - self.position as usize,
        }
    }

    fn y(&self) -> usize {
        match self.direction {
            Direction::Left => self.position as usize,
            Direction::Down => 4,
            Direction::Right => 4 - self.position as usize,
            Direction::Up => 0,
        }
    }
}

#[entry]
fn main() -> ! {
    let board = Board::take().unwrap();
    let mut timer = Timer::new(board.TIMER0);
    let mut display = Display::new(board.display_pins);
    let mut leds = [
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
    ];

    let mut state: State = Default::default();

    loop {
        leds[state.x()][state.y()] = 0;
        state.advance();
        leds[state.x()][state.y()] = 1;
        display.show(&mut timer, leds, DELAY)
    }
}
