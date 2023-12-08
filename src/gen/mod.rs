use std::f32::consts::PI;
use std::fs::File;
use std::path::Path;
use crate::eval::SAMPLING_RATE;

pub fn write_wav(filename: &str, data: Vec<i16>) {
    let header = wav::Header {
        audio_format: 1,
        channel_count: 1,
        sampling_rate: SAMPLING_RATE as u32,
        bytes_per_second: SAMPLING_RATE as u32 * 2,
        bytes_per_sample: 2,
        bits_per_sample: 16,
    };

    let mut out_file = File::create(Path::new(filename)).unwrap();
    // let mut new_data: Vec<i16> = vec![];
    // for i in 0..50000 {
    //     new_data.push(((i as f32 / 44100. * 440. * 2. * PI * 50000. / (i as f32 + 50000.)).sin() * 10000.) as i16);
    // }
    //println!("{:?}", new_data);
    //wav::write(header, &data, &mut out_file).unwrap();
    wav::write(
        header,
        &wav::BitDepth::Sixteen(data),
        &mut out_file
    ).unwrap();
}
