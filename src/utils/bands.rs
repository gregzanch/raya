use crate::utils::standard;

/// Returns the nominal octave band frequencies between a given range (inclusive)
///
/// # Arguments
///
/// * `start` - start frequency
/// * `end` - end frequency
///
/// # Examples
///
/// ```
/// // get octave band frequencies between 63 and 1000
/// octave(63.0, 4000.0) // [63.0, 125.0, 250.0, 500.0, 1000.0]
/// ```
pub fn octave(start: f32, end: f32) -> Vec<f32> {
    standard::WHOLE_OCTAVE
        .iter()
        .filter_map(|x| {
            if *x >= start && *x <= end {
                Some(*x)
            } else {
                None
            }
        })
        .collect()
}

/// Returns the nominal third octave band frequencies between a given range (inclusive)
///
/// # Arguments
///
/// * `start` - start frequency
/// * `end` - end frequency
///
/// # Examples
///
/// ```
/// // get third octave band frequencies between 250 and 800
/// octave(250.0, 800.0) // [250.0, 315.0, 400.0, 500.0, 630.0, 800.0]
/// ```
pub fn third_octave(start: f32, end: f32) -> Vec<f32> {
    standard::THIRD_OCTAVE
        .iter()
        .filter_map(|x| {
            if *x >= start && *x <= end {
                Some(*x)
            } else {
                None
            }
        })
        .collect()
}
