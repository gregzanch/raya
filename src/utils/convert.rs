use super::standard::{P_REF, I_REF, W_REF};
use std::f32::consts::PI;

/// Convert sound power level to sound pressure level
///
/// # Arguments
///
/// * `lw` - sound power level
/// * `r` - radius
/// * `q` - directivity
///
pub fn lw_2_lp(lw: Vec<f32>, r: f32, q: f32) -> Vec<f32> {
  lw.iter().map(|x| x - f32::abs(10.0 * f32::log10(q / (4.0 * PI * r * r)))).collect()
}

/// Convert sound pressure level to sound power level
///
/// # Arguments
///
/// * `lp` - sound pressure level
/// * `r` - radius
/// * `q` - directivity
///
pub fn lp_2_lw(lp: Vec<f32>, r: f32, q: f32) -> Vec<f32> {
  lp.iter().map(|x| f32::abs(10.0 * f32::log10(q / (4.0 * PI * r * r))) + x).collect()
}

/// Convert pressure to sound pressure level
///
/// # Arguments
///
/// * `p` - pressure
pub fn p_2_lp(p: Vec<f32>) -> Vec<f32> {
  p.iter().map(|x| 20.0 * f32::log10(x / P_REF)).collect()
}

/// Convert sound pressure level to pressure
///
/// # Arguments
///
/// * `lp` - sound pressure level
///
pub fn lp_2_p(lp: Vec<f32>) -> Vec<f32> {
  lp.iter().map(|x| f32::powf(10.0, x / 20.0) * P_REF).collect()
}


/// Convert Intensity to sound intensity level
///
/// # Arguments
///
/// * `i` - intensity
///
pub fn i_2_li(i: Vec<f32>) -> Vec<f32> {
  i.iter().map(|x| 10.0 * f32::log10(x / I_REF)).collect()
}

/// Convert sound intensity level to Intensity
///
/// # Arguments
///
/// * `li` - sound intensity level
///
pub fn li_2_i(li: Vec<f32>) -> Vec<f32> {
  li.iter().map(|x| f32::powf(10.0, x / 10.0) * I_REF).collect()
}



/// Convert Power to sound power level
///
/// # Arguments
///
/// * `w` - power
///
pub fn w_2_lw(w: Vec<f32>) -> Vec<f32> {
  w.iter().map(|x| 10.0 * f32::log10(x / W_REF)).collect()
}


/// Convert sound power level to power
///
/// # Arguments
///
/// * `lw` - sound power level
///
pub fn lw_2_w(lw: Vec<f32>) -> Vec<f32> {
  lw.iter().map(|x| f32::powf(10.0, x / 10.0) * W_REF).collect()
}

/// Convert pressure to intensity
///
/// # Arguments
///
/// * `p` - pressure in Pa
/// * `z0` - specific acoustic impedance (400 N·s/m3 for air)
///
pub fn p_2_i(p: Vec<f32>, z0: f32) -> Vec<f32> {
  p.iter().map(|x| f32::powf(*x, 2.0 / z0)).collect()
}

/// Convert intensity to pressure
///
/// # Arguments
///
/// * `i` - intensity in W/m^2
/// * `z0` - specific acoustic impedance (400 N·s/m3 for air)
///
pub fn i_2_p(i: Vec<f32>, z0: f32) -> Vec<f32> {
  i.iter().map(|x| f32::sqrt(x * z0)).collect()
}

pub fn lp_2_i(lp: f32) -> f32 {
  f32::powf(f32::powf(10.0, lp / 20.0) * P_REF, 0.005)
}


pub fn i_2_lp(i: f32) -> f32 {
  20.0 * f32::log10(f32::sqrt(i * 400.0) / P_REF)
}

