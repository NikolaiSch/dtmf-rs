use ::sample::Signal;
use ::sample::signal::Delay;

use ::Message;
use super::SignalEncoder;

/// An encoder which encodes a DTMF message.
#[derive(Clone)]
pub struct MessageEncoder {
    signals: Vec<Delay<SignalEncoder>>,
    current_index: usize,
}

impl MessageEncoder {
    /// Creates a new encoder given a message and a sample rate
    pub fn new(message: &Message, sample_rate: f64) -> MessageEncoder {
        let signal_length = message.signal_duration().as_secs() as f64 * sample_rate;
        let silence_length = message.silence_duration().as_secs() as usize * sample_rate as usize;

        let mut signals = Vec::new();
        let mut signal_iterator = message.iter();

        // Add the first signal without delay, the others with it.
        if let Some(signal) = signal_iterator.next() {
            signals.push(SignalEncoder::new(*signal, signal_length).expect("Valid signal").delay(0));
            for signal in signal_iterator {
                signals.push(SignalEncoder::new(*signal, signal_length)
                    .expect("Valid signal")
                    .delay(silence_length));
            }
        }

        MessageEncoder {
            signals: signals,
            current_index: 0,
        }
    }
}

impl Iterator for MessageEncoder {
    type Item = [f64; 1];

    fn next(&mut self) -> Option<Self::Item> {
        match self.current_index < self.signals.len() {
            true => {
                self.signals[self.current_index].next().or_else(|| {
                    self.current_index += 1;
                    self.next()
                })
            }
            false => None,
        }
    }
}
