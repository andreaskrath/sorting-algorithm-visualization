use std::{fs::File, time::Instant};

use gif::Frame;
use sorting::{bingo_sort, bogosort, insertion_sort, util::generate_mixed_vector};

fn main() {
    let now = Instant::now();
    let mut vec = generate_mixed_vector(250);
    let mut frames: Vec<Frame> = Vec::new();
    insertion_sort::insertion_sort(&mut vec, &mut frames);

    let mut gif = File::create("sample.gif").unwrap();
    let mut encoder = gif::Encoder::new(&mut gif, 250, 250, &[]).unwrap();

    for (counter, frame) in frames.iter().enumerate() {
        encoder.write_frame(frame).unwrap();
        println!("Frame {} was written.", counter + 1);
    }
    println!("Time elapsed: {:.2?}", now.elapsed());
}
