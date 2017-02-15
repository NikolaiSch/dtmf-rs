use std;

use ::Signal;

/// Decodes a signal from a stream of samples.
/// # Example
/// ```
/// use ::dtmf::encoder::SignalEncoder;
/// use ::dtmf::decoder::decode_signal;
/// use ::dtmf::Signal;
///
/// for &signal in Signal::iter() {
///     let data = SignalEncoder::new(signal, 48000.).unwrap().take(12000).map(|x| x[0]).collect::<Vec<f64>>();
///     assert_eq!(decode_signal(&data, 48000.), signal);
/// }
/// ```
pub fn decode_signal(samples: &Vec<f64>, sample_rate: f64) -> Signal {
    let low_freq = goertzel_filter(samples, sample_rate, &[697, 770, 852, 941]);
    let high_freq = goertzel_filter(samples, sample_rate, &[1209, 1336, 1477, 1633]);

    Signal::from_frequencies((low_freq, high_freq)).expect("Valid frequencies")
}

/// Examines frequency which has most power in samples
fn goertzel_filter(samples: &Vec<f64>, sample_rate: f64, dtmf_freq: &[i32]) -> u16 {
    let len = samples.len() as i64;
    let step = sample_rate / (len as f64);
    let step_normalized = 1.0 / (len as f64);

    // make bins
    let mut bins = Vec::new();
    for i in dtmf_freq.iter() {
        let freq = (*i as f64) / step;
        // if freq > (len as f64) - 1f64 {
        //    return None;
        // }
        bins.push(freq.clone());
    }

    let n_range: Vec<i64> = (0..len).collect();
    let mut freqs = Vec::new();
    let mut results = Vec::new();

    for k in bins {
        // bin frequency and coefficients for computation
        let f = k * step_normalized;
        let real = 2.0 * (2.0 * std::f64::consts::PI * f).cos();

        let mut coeff1 = 0.0;
        let mut coeff2 = 0.0;
        // doing calculation on all samples
        for n in &n_range {
            let y = samples[*n as usize] + real * coeff1 - coeff2;
            coeff2 = coeff1;
            coeff1 = y;
        }
        // storing results
        results.push(coeff2.powi(2) + coeff1.powi(2) - real * coeff1 * coeff2);
        freqs.push(f * sample_rate);
    }

    // comparing results, find frequency
    // freqs[results.iter().enumerate().max().0]
    let mut index = 0;
    for (j, &value) in results.iter().enumerate() {
        if value > results[index] {
            index = j;
        }
    }

    (freqs[index].round() as u16)
}
