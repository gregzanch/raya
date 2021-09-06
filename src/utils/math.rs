pub fn interpolate_log(x1: f32, y1: f32, x2: f32, y2: f32, xi: f32) -> f32 {
    y1 + (f32::log10(xi) - f32::log10(x1)) / (f32::log10(x2) - f32::log10(x1)) * (y2 - y1)
}

pub fn interpolate_alpha(alpha: Vec<f32>, freq: Vec<f32>) -> impl FnOnce(f32) -> f32 {
    let freq_clone = freq.clone();
    let alpha_clone = alpha.clone();
    let func = move |f: f32| {
        let mut i = 0;
        while f > freq_clone[i] && i < freq_clone.len() {
            i += 1;
        }
        if i > 0 && i < freq_clone.len() {
            let x1 = freq_clone[i - 1];
            let y1 = alpha_clone[i - 1];
            let x2 = freq_clone[i];
            let y2 = alpha_clone[i];
            let xi = f;
            return interpolate_log(x1, y1, x2, y2, xi);
        } else {
            if i == 0 {
                return alpha_clone[i];
            } else {
                return alpha_clone[freq_clone.len() - 1];
            }
        }
    };
    func
}

/// modulus operation. wraps a number
///
/// @param n dividend
/// @param m divisor
///
pub fn modulo(n: i32, m: i32) -> i32 {
    if n < 0 {
        return m - (i32::abs(n) % m);
    } else {
        return n % m;
    }
}

/// reflected modulus operation. instead of wrapping it reflects
///
/// @param n dividend
/// @param m divisor
///
pub fn reflected_modulo(n: i32, m: i32) -> i32 {
    return m - 2 * i32::abs(modulo(n / 2, m) - 1);
}
