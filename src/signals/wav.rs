use hound;
use std::path;

// pub fn combine<P>(files: Vec<P>, out_file: P) -> Result<(), std::io::Error>
// where
//     P: AsRef<path::Path>,
// {
//     let mut reader = hound::WavReader::open("testsamples/pop.wav").unwrap();
//     let sqr_sum = reader.samples::<i16>().fold(0.0, |sqr_sum, s| {
//         let sample = s.unwrap() as f64;
//         sqr_sum + sample * sample
//     });
//     Ok(())
// }
