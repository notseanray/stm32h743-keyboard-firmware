//! The equivalent of Keyberon's matrix.rs but for Hall Effect sensors
//! connected to analog multilpexers

use core::fmt::Write;
use core::ops::{Index, IndexMut};
use crate::config;
use crate::layers;
use keyberon::layout::{Event, Layout};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ChannelState {
    pub pressed: bool,
    pub value: u16,
    pub default: u16,
    // pub smoothed: ArrayDeque<[u16; SMOOTHING], Wrapping>,
    // These are so we can track/debug voltage wobble:
    pub low: u16,     // Keep track of the lowest value
    pub high: u16,    // Keep track of the highest value
    pub rising: bool, // Whether or not this channel is rising (only used by rotary encoders)
}

impl Default for ChannelState {
    fn default() -> ChannelState {
        ChannelState {
            pressed: false,
            value: 0,
            default: 0,
            // smoothed: ArrayDeque::new(),
            low: 3200,
            high: 0,
            rising: false,
        }
    }
}

impl ChannelState {
    /// Sets self.pressed to true
    pub fn press(&mut self) {
        self.pressed = true;
    }

    /// Sets self.pressed to false
    pub fn release(&mut self) {
        self.pressed = false;
    }

    /// Updates self.default with the given value
    pub fn update_default(&mut self, val: u16) {
        self.default = val;
    }

    /// Records the given value, making sure to record any lows or highs.
    /// If *default* is true then the value will be recorded as the default value.
    pub fn record_value(&mut self, val: u16) {
        self.value = val;
        if val > self.high {
            self.high = val;
        }
        if val < self.low {
            self.low = val;
        }
    }
}

/// Struct for storing the state of each channel and pretty-printing it via rprintln
#[derive(Debug, Default, Clone)]
pub struct ChannelStates {
    pub states: [ChannelState; config::KEYBOARD_MAX_CHANNELS],
    curr: usize,        // Iterator tracking
    next: usize,        // Ditto
    pub pressed: usize, // Records how many keys are currently pressed
}

impl ChannelStates {
    pub fn update_default_by_index(&mut self, chan: usize, val: u16) {
        self.states[chan].default = val;
    }
    pub fn update_rising_by_index(&mut self, chan: usize, val: bool) {
        self.states[chan].rising = val;
    }
    pub fn press(&mut self, chan: usize) {
        self.states[chan].press();
        self.pressed_add();
    }
    pub fn release(&mut self, chan: usize) {
        self.states[chan].release();
        self.pressed_sub();
    }
    pub fn pressed_add(&mut self) {
        self.pressed += 1;
    }
    pub fn pressed_sub(&mut self) {
        self.pressed -= 1;
    }
}

impl Iterator for ChannelStates {
    type Item = ChannelState;

    fn next(&mut self) -> Option<ChannelState> {
        if self.curr < config::KEYBOARD_MAX_CHANNELS {
            self.curr = self.curr.saturating_add(1);
        } else {
            return None;
        }
        Some(self.states[self.curr])
    }
}

impl Index<usize> for ChannelStates {
    type Output = ChannelState;

    fn index(&self, i: usize) -> &ChannelState {
        &self.states[i]
    }
}

impl IndexMut<usize> for ChannelStates {
    fn index_mut<'a>(&'a mut self, i: usize) -> &'a mut ChannelState {
        &mut self.states[i]
    }
}

// impl our super user-friendly terminal view into all channels
impl core::fmt::Display for ChannelStates {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        // let _ = f.write_str("\x1B[2J\x1B[0H"); // Clear the screen and move cursor to start
        let _ = f.write_str("Multiplexer Channel Values:\n");
        let _ = f.write_str("\nch0    ch1    ch2    ch3    ch4    ch5    ch6    ch7\n");
        for i in 0..8 {
            let _ = f
                .write_fmt(format_args!("{}   ", self.states[i].value))
                .unwrap();
        }
        let _ = f.write_str("\n");
        let _ = f.write_str("ch8    ch9    ch10   ch11   ch12   ch13   ch14   ch15\n");
        for i in 8..10 {
            let _ = f
                .write_fmt(format_args!("{}   ", self.states[i].value))
                .unwrap();
        }
        for i in 10..16 {
            let _ = f
                .write_fmt(format_args!("{}    ", self.states[i].value))
                .unwrap();
        }
        let _ = f.write_str("\n");
        Ok(())
    }
}

///! Updates the status of all the Keyberon stuff in *layout* and returns true if a NEW keypress or rotary encoder movement was detected
pub fn check_channel(
    multilpexer: usize,
    chan: usize,
    millivolts: u16,
    ch_states: &mut [ChannelStates],
    layout: &mut Layout<16, 5, 7, ()>,
    rotary_clockwise: &mut bool,
    actuation_threshold: u16,
    encoder_press_threshold: u16,
    release_threshold: u16,
) -> bool {
    let ch_state = ch_states[multilpexer][chan];
    if ch_state.value > config::KEYBOARD_IGNORE_BELOW {
        let voltage_difference = if millivolts < ch_state.default {
            if config::KEYBOARD_NORTH_DOWN > 0 {
                ch_state.default - millivolts // North side down switches result in a mV drop
            } else {
                0
            }
        } else if config::KEYBOARD_NORTH_DOWN > 0 {
                0
        } else {
            millivolts - ch_state.default // South side down switches result in a mV increase
        };
        // Handle normal keypresses
        if voltage_difference > actuation_threshold {
            if !ch_state.pressed {
                // Encoder press doesn't work very reliably (needs work--probably a change to the PCB):
                if multilpexer == config::ENCODER_MUX {
                    if chan != config::ENCODER_CHANNEL1 && chan != config::ENCODER_CHANNEL2 {
                        ch_states[multilpexer].press(chan);
                        let _ = layout.event(Event::Press(multilpexer as u8, chan as u8));
                    }
                } else {
                    ch_states[multilpexer].press(chan);
                    let _ = layout.event(Event::Press(multilpexer as u8, chan as u8));
                }
                return true;
            }
        } else if voltage_difference < release_threshold && ch_state.pressed {
            if multilpexer == config::ENCODER_MUX {
                if chan != config::ENCODER_CHANNEL1 && chan != config::ENCODER_CHANNEL2 {
                    ch_states[multilpexer].release(chan);
                    let _ = layout.event(Event::Release(multilpexer as u8, chan as u8));
                }
            } else {
                ch_states[multilpexer].release(chan);
                let _ = layout.event(Event::Release(multilpexer as u8, chan as u8));
            }
            return false;
        }
    }
    false
}
