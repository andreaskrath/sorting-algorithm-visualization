use gif::Frame;

use crate::image::generate_frame;

pub fn insertion_sort(input: &mut Vec<u16>, frames: &mut Vec<Frame>) {
    if input.len() < 2 {
        return;
    }

    generate_frame(input, frames);

    for index in 1..input.len() {
        let mut jindex = index;
        while jindex > 0 && input.get(jindex - 1).unwrap() > input.get(jindex).unwrap() {
            input.swap(jindex - 1, jindex);
            jindex -= 1;
        }
        generate_frame(input, frames);
    }
}

#[cfg(test)]
mod sort {
    use crate::util::is_sorted;

    use super::insertion_sort;

    #[test]
    fn sample() {
        let mut input = vec![432, 7, 12, 37, 74, 1, 7, 5, 1, 9];
        insertion_sort(&mut input, &mut Vec::new());
        assert!(is_sorted(&input));
    }
}
