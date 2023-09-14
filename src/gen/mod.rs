use std::fs::File;
use std::path::Path;
use std::f32::consts::PI;

pub fn write_wav() {
    let mut inp_file = File::open(Path::new("test/audio/sine.wav")).unwrap();
    let (header, data) = wav::read(&mut inp_file).unwrap();
    println!("{:?}", header);

    let mut out_file = File::create(Path::new("test/audio/output.wav")).unwrap();
    // let new_data = wav::BitDepth::Sixteen(vec![1000; 50000]);
    let mut new_data: Vec<i16> = vec![];
    for i in 0..50000 {
        new_data.push(((i as f32 / 44100. * 440. * 2. * PI).sin() * 10000.) as i16);
    }
    let new_data = wav::BitDepth::Sixteen(new_data);
    //println!("{:?}", new_data);
    //wav::write(header, &data, &mut out_file).unwrap();
    wav::write(header, &new_data, &mut out_file).unwrap();
}
