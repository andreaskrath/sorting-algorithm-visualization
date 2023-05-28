use gif::Frame;

use crate::image::generate_frame;

pub fn bingo_sort(input: &mut Vec<u16>, frames: &mut Vec<Frame>) {
    if input.len() < 2 {
        return;
    }

    generate_frame(input, frames);

    let in_len = input.len();

    let minmax = get_min_max(input);
    let min = minmax[0];
    let max = minmax[1];

    let mut bingo = min;
    let mut n_bingo = max;
    let mut n_index = 0;

    while bingo < max {
        let start = n_index;
        for i in start..in_len {
            if input[i] == bingo {
                input.swap(i, n_index);
                n_index += 1;
            }
            if input[i] < n_bingo {
                n_bingo = input[i];
            }
        }
        bingo = n_bingo;
        n_bingo = max;

        generate_frame(input, frames);
    }
}

fn get_min_max(input: &[u16]) -> [u16; 2] {
    let mut min = input[0];
    let mut max = input[0];
    for num in input.iter().skip(1) {
        if *num < min {
            min = *num;
        }
        if *num > max {
            max = *num;
        }
    }
    [min, max]
}
