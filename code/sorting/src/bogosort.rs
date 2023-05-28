use gif::Frame;

use crate::{
    image::generate_frame,
    util::{is_sorted, mix_vector},
};

pub fn bogosort(input: &mut Vec<u16>, frames: &mut Vec<Frame>) {
    generate_frame(input, frames);
    while !is_sorted(input) {
        mix_vector(input);
        generate_frame(input, frames);
    }
}
