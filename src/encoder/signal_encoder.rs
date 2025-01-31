use dasp_signal::Signal;
use dasp_signal::{rate, AddAmp, ConstHz, ScaleAmp, Sine};

use crate::Signal as DtmfSignal;

/// An encoder which encodes a specific DTMF signal.
#[derive(Clone)]
pub struct SignalEncoder(AddAmp<ScaleAmp<Sine<ConstHz>>, ScaleAmp<Sine<ConstHz>>>);

impl SignalEncoder {
    /// Creates a new encoder given an specific DTMF signal and a sample rate.
    /// # Example
    /// ```
    /// use dtmf::encoder::SignalEncoder;
    /// use dtmf::Signal;
    ///
    /// assert!(SignalEncoder::new(Signal::Hash, 44_100.0).is_some(), "Encoder was none.");
    /// assert!(SignalEncoder::new(Signal::Digit(66), 44_100.0).is_none(), "Invalid encoder");
    /// ```
    pub fn new(signal: DtmfSignal, sample_rate: f64) -> Option<SignalEncoder> {
        signal.frequencies().map(|(f1, f2)| {
            let sine1 = rate(sample_rate).const_hz(f1 as f64).sine().scale_amp(0.4);
            let sine2 = rate(sample_rate).const_hz(f2 as f64).sine().scale_amp(0.5);
            SignalEncoder(sine1.add_amp(sine2))
        })
    }
}

impl Iterator for SignalEncoder {
    type Item = [f64; 1];

    fn next(&mut self) -> Option<Self::Item> {
        self.next()
    }
}
