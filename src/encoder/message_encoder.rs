use dasp_signal::from_iter;
use dasp_signal::{Delay, Signal};

use std::iter::Take;

use super::SignalEncoder;
use crate::Message;

/// An encoder which encodes a DTMF message.
#[derive(Clone)]
pub struct MessageEncoder {
    signals: Vec<Delay<dasp_signal::FromIterator<Take<SignalEncoder>>>>,
    current_index: usize,
    size: usize,
}

impl MessageEncoder {
    /// Creates a new encoder given a message and a sample rate
    /// # Example
    /// ```
    /// use dtmf::{Message, Signal};
    /// use dtmf::encoder::MessageEncoder;
    ///
    /// let mut message = Message::default();
    /// message.enqueue(Signal::A);
    /// message.enqueue(Signal::B);
    ///
    /// let encoder = MessageEncoder::new(&message, 48000.);
    ///
    /// // The MessageEncoder implements ExactSizeIterator
    /// let sample_num = encoder.len();
    /// assert_eq!(encoder.count(), sample_num);
    /// ```
    pub fn new(message: &Message, sample_rate: f64) -> MessageEncoder {
        let signal_length = (message.signal_duration() * sample_rate) as usize;
        let silence_length = (message.silence_duration() * sample_rate) as usize;

        let mut signals = Vec::new();
        let mut signal_iterator = message.iter();

        // Add the first signal without delay, the others with it.
        let size = match signal_iterator.next() {
            Some(signal) => {
                signals.push(
                    from_iter(
                        SignalEncoder::new(*signal, sample_rate)
                            .expect("Valid signal")
                            .take(signal_length),
                    )
                    .delay(0),
                );

                for signal in signal_iterator {
                    signals.push(
                        from_iter(
                            SignalEncoder::new(*signal, sample_rate)
                                .expect("Valid signal")
                                .take(signal_length),
                        )
                        .delay(silence_length),
                    );
                }

                (message.len() - 1) * (signal_length + silence_length) + signal_length
            }
            None => 0,
        };

        MessageEncoder {
            signals: signals,
            current_index: 0,
            size: size,
        }
    }
}

impl Iterator for MessageEncoder {
    type Item = [f64; 1];

    fn next(&mut self) -> Option<Self::Item> {
        match self.current_index < self.signals.len() {
            true => {
                self.size = self.size.saturating_sub(1);
                self.current_index += 1;
                return Some(self.signals[self.current_index].next());
            }
            false => None,
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.size, Some(self.size))
    }
}

impl ExactSizeIterator for MessageEncoder {
    fn len(&self) -> usize {
        self.size
    }
}
