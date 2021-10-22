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
pub struct Sequence<T>
where
    T: Debug + PartialEq + Eq,
{
    states: Vec<T>,
    reset_signal: T,
    last_input: Option<T>,
    next_state: usize,
}

impl<T> Sequence<T>
where
    T: Debug + PartialEq + Eq,
{
    pub fn new(states: Vec<T>, reset_signal: T) -> Self
    where
        T: Debug + PartialEq + Eq,
    {
        Sequence {
            states,
            reset_signal,
            last_input: None,
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
        match (self.last_input.as_ref(), self.next_state > 0) {
            (Some(_), true) => true,
            (None, false) => true,
            _ => false,
        }
    }

    pub fn go_to(&mut self, input: T) -> bool
    where
        T: Debug + PartialEq + Eq,
    {
        let current_next_state = self.next_state;

        if self.is_complete() {
            self.next_state = 0;
        }

        if self.can_advance() {
            if let Some(state) = self.states.get(self.next_state) {
                if *state == input {
                    self.next_state += 1;
                } else {
                    self.next_state = 0;
                }
            }
        }

        self.last_input = match input == self.reset_signal {
            true => None,
            false => Some(input),
        };

        self.next_state > current_next_state
    }
}
