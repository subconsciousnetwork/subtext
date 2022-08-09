use core::fmt::Debug;

/**
 * A basic state machine to track interesting sequences of values.
 *
 * The sequence is considered complete when its states have been traversed in
 * order.
 * The sequence becomes stuck when sent to an out-of-order state and won't be
 * able to continue until it encounters the configured "reset" signal.
 */
#[derive(Debug)]
pub struct Sequence<'a> {
    states: &'a [char],
    sent_to_state: bool,
    reset_signal: Option<char>,
    next_state: usize,
}

impl<'a> Sequence<'a> {
    pub fn new(states: &'a [char], reset_signal: Option<char>) -> Self {
        Sequence {
            states,
            sent_to_state: false,
            reset_signal,
            next_state: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.states.len()
    }

    pub fn is_complete(&self) -> bool {
        self.next_state == self.states.len()
    }

    pub fn can_advance(&self) -> bool {
        match (self.sent_to_state, self.next_state > 0) {
            (true, true) => true,
            (false, false) => true,
            _ => false,
        }
    }

    pub fn go_to(&mut self, input: &char) -> bool {
        let current_next_state = self.next_state;

        if self.is_complete() {
            self.next_state = 0;
        }

        if self.can_advance() {
            if let Some(state) = self.states.get(self.next_state) {
                if *state == *input {
                    self.next_state += 1;
                } else {
                    self.next_state = 0;
                }
            }
        }

        let state_advanced = self.next_state > current_next_state;

        let reset_signal_sent = match self.reset_signal.as_ref() {
            Some(reset_signal) => *reset_signal == *input,
            None => true,
        };

        self.sent_to_state = state_advanced || !reset_signal_sent;

        state_advanced
    }
}
