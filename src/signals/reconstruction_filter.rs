use nalgebra::Normed;
use rustfft::{num_complex::Complex, FftPlanner};
use std::f32::consts::PI;

fn max_width_factor(r: [f32; 2], step: f32) -> f32 {
    let base = f32::powf(f32::max(r[0], r[1]) / f32::min(r[0], r[1]), step);
    (base - 1.0) / (base + 1.0)
}

fn width_factor(r: [f32; 2], bands: f32, overlap: f32) -> f32 {
    assert!(
        overlap >= 0.0 && overlap <= 1.0,
        "Overlap must be in the range 0-1."
    );
    max_width_factor(r, 1.0 / bands) * overlap
}

// p = relative frequency
// P = relative width
// l = steepness
fn band_edge_impl(relative_frequency: f32, relative_width: f32, l: f32) -> f32 {
    if l != 0.0 {
        return f32::sin(PI * band_edge_impl(relative_frequency, relative_width, l - 1.0) / 2.0);
    } else {
        return ((relative_frequency / relative_width) + 1.0) / 2.0;
    }
}

fn lower_band_edge(relative_frequency: f32, relative_width: f32, l: f32) -> f32 {
    assert!(relative_width >= 0.0, "P must be greater or equal to 0.");
    if relative_width == 0.0 {
        if relative_frequency >= 0.0 {
            return 1.0;
        } else {
            return 0.0;
        }
    }
    return f32::powf(
        f32::sin(PI * band_edge_impl(relative_frequency, relative_width, l) / 2.0),
        2.0,
    );
}

fn upper_band_edge(relative_frequency: f32, relative_width: f32, l: f32) -> f32 {
    assert!(relative_width >= 0.0, "P must be greater or equal to 0.");
    if relative_width == 0.0 {
        if relative_frequency < 0.0 {
            return 1.0;
        } else {
            return 0.0;
        }
    }
    return f32::powf(
        f32::cos(PI * band_edge_impl(relative_frequency, relative_width, l) / 2.0),
        2.0,
    );
}

fn band_edge_frequency(band: f32, bands: f32, r: [f32; 2]) -> f32 {
    let r0 = f32::min(r[0], r[1]);
    let r1 = f32::max(r[0], r[1]);
    return r0 * f32::powf(r1 / r0, band / bands);
}

#[allow(dead_code)]
fn band_centre_frequency(band: f32, bands: f32, r: [f32; 2]) -> f32 {
    return band_edge_frequency(band * 2.0 + 1.0, bands * 2.0, r);
}

fn compute_bandpass_magnitude(frequency: f32, r: [f32; 2], width_factor: f32, l: f32) -> f32 {
    assert!(
        width_factor >= 0.0 && width_factor <= 1.0,
        "width_factor must be in the range 0-1."
    );
    return compute_lopass_magnitude(frequency, f32::max(r[0], r[1]), width_factor, l)
        * compute_hipass_magnitude(frequency, f32::min(r[0], r[1]), width_factor, l);
}

fn compute_lopass_magnitude(frequency: f32, edge: f32, width_factor: f32, l: f32) -> f32 {
    assert!(
        width_factor >= 0.0 && width_factor <= 1.0,
        "width_factor must be in the range 0-1."
    );
    let absolute_width = edge * width_factor;
    if frequency < edge - absolute_width {
        return 1.0;
    }
    if frequency < edge + absolute_width {
        return upper_band_edge(frequency - edge, absolute_width, l);
    }
    return 0.0;
}

fn compute_hipass_magnitude(frequency: f32, edge: f32, width_factor: f32, l: f32) -> f32 {
    assert!(
        width_factor >= 0.0 && width_factor <= 1.0,
        "width_factor must be in the range 0-1."
    );

    let absolute_width = edge * width_factor;
    if frequency < edge - absolute_width {
        return 0.0;
    }
    if frequency < edge + absolute_width {
        return lower_band_edge(frequency - edge, absolute_width, l);
    }
    return 1.0;
}

#[allow(dead_code)]
fn dirac_delta(length: u32, offset: u32) -> Vec<f32> {
    let mut samples: Vec<f32> = Vec::new();
    samples.reserve(length as usize);
    let index = offset % length;
    for i in 0..length {
        if i == index {
            samples.push(1.0);
        } else {
            samples.push(0.0);
        }
    }
    return samples;
}

///
/// Perfect reconstruction filter for banded signals
/// @param samples banded signals
/// @returns
///
pub fn filter_signals(samples: Vec<Vec<f32>>) -> Vec<Vec<f32>> {
    let mut planner = FftPlanner::<f32>::new();
    let fft = planner.plan_fft_forward(samples[0].len());
    let mut inv_planner = FftPlanner::<f32>::new();
    let inv_fft = inv_planner.plan_fft_inverse(samples[0].len());

    let bands = samples.len();
    let samplerate = 44100;
    let minf = 63.0;
    let maxf = 16000.0;
    let len = bands + 1;

    let mut band_edges: Vec<f32> = vec![0.0; len];
    for band in 0..len {
        band_edges[band] = band_edge_frequency(band as f32, bands as f32, [minf, maxf]);
    }

    let lower_edges = band_edges
        .get_mut(0..(len - 1))
        .expect("to slice the array")
        .to_vec();
    let upper_edges = band_edges
        .get_mut(1..(len))
        .expect("to slice the array")
        .to_vec();

    let wf = width_factor([minf, maxf], bands as f32, 1.0);

    let mut frequencies: Vec<f32> = vec![0.0; samples[0].len()];
    for i in 0..frequencies.len() {
        frequencies[i] = (i as f32) * (samplerate as f32) / (samples[0].len() as f32);
    }

    let mut filters: Vec<Vec<f32>> = Vec::new();
    for i in 0..bands {
        let mut filter: Vec<f32> = frequencies
            .iter()
            .map(|f| compute_bandpass_magnitude(*f, [lower_edges[i], upper_edges[i]], wf, 0.0))
            .collect();
        let half_len = f32::trunc(filter.len() as f32 / 2.0) as usize;
        let mut c = half_len;
        for j in (half_len + 1)..(filter.len()) {
            filter[j] = filter[c];
            c -= 1;
        }
        filters.push(filter)
    }

    let mut filtered_samples: Vec<Vec<f32>> = Vec::new();

    let mut complex_samples: Vec<Vec<Complex<f32>>> = samples
        .iter()
        .map(|arr| {
            arr.iter()
                .map(|x| Complex { re: *x, im: 0.0 })
                .collect::<Vec<Complex<f32>>>()
        })
        .collect();

    for i in 0..complex_samples.len() {
        // Perform a forward FFT of size 1234

        fft.process(&mut complex_samples[i]);

        for j in 0..complex_samples[i].len() {
            complex_samples[i][j].scale_mut(filters[i][j]);
        }

        inv_fft.process(&mut complex_samples[i]);
        filtered_samples.push(complex_samples[i].iter().map(|c| c.re).collect());
    }

    return filtered_samples;
}
