/// Calculate the air attenuation in dB per meter
///
/// # Arguments
///
/// * `frequency` - frequency in Hz
/// * `temperature` - temperature temperature in °C (20°C)
/// * `humidity` - relative humidity in % (40%)
/// * `pressure` - pressure atmospheric pressure in Pa (101325 Pa)
///
/// [more info](http://en.wikibooks.org/wiki/Engineering_Acoustics/Outdoor_Sound_Propagation)
///
pub fn air_attenuation(
    frequency: &Vec<f32>,
    temperature: f32,
    humidity: f32,
    pressure: f32,
) -> Vec<f32> {
    let t = temperature + 273.15;
    let t0 = 293.15;
    let t01 = 273.16;
    let ps0 = 1.01325e5;
    let ps: f32 = pressure;
    let c_sat = -6.8346 * f32::powf(t01 / t, 1.261) + 4.6151;
    let rhosat = f32::powf(10.0, c_sat);
    let h = (rhosat * humidity * ps0) / ps;
    let frn = (ps / ps0)
        * f32::powf(t0 / t, 0.5)
        * (9.0 + 280.0 * h * f32::exp(-4.17 * (f32::powf(t0 / t, 1.0 / 3.0) - 1.0)));
    let fro = (ps / ps0) * (24.0 + (4.04e4 * h * (0.02 + h)) / (0.391 + h));
    let mut alphas: Vec<f32> = Vec::new();
    frequency.iter().for_each(|f| {
        let alpha = f32::powf(*f, 2.0)
            * (1.84e-11 / ((f32::powf(t0 / t, 0.5) * ps) / ps0)
                + f32::powf(t0 / t, -2.5)
                    * ((0.1068 * f32::exp(-3352.0 / t) * frn) / (f * f + frn * frn)
                        + (0.01278 * f32::exp(-2239.1 / t) * fro) / (f * f + fro * fro)));
        alphas.push((20.0 * alpha) / f32::ln(10.0));
    });

    alphas
}
