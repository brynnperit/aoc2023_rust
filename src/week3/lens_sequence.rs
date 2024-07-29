use core::{panic, str};

use lens_step::LensStep;

mod lens_step;

pub struct LensSequence {
    lenses: Vec<LensStep>,
}

impl LensSequence {
    pub fn from_file(path: std::ffi::OsString) -> Self {
        let input = clio::Input::new(&path).unwrap();
        let mut input = std::io::BufRead::split(std::io::BufReader::new(input), b',');
        let mut lenses = Vec::new();
        while let Some(Ok(lens_vec)) = input.next() {
            if let Ok(lens_string) = str::from_utf8(&lens_vec) {
                lenses.push(LensStep::from_string(lens_string).unwrap_or_else(|| {
                    panic!("This string should look more like 'rn=1': {}", lens_string)
                }));
            }
        }
        Self { lenses }
    }

    pub fn get_hashes(&self) -> Vec<u8> {
        self.lenses.iter().map(|lens| lens.get_hash()).collect()
    }

    pub fn get_focusing_power(&self) -> u64 {
        let mut lens_boxes = vec![vec![]; 256];
        for lens_step in self.lenses.iter() {
            let box_number: usize = lens_step.get_label_hash().into();
            lens_step.perform_operation(&mut lens_boxes[box_number]);
        }
        let mut focusing_power = 0;
        for (box_index, lens_box) in lens_boxes.iter().enumerate() {
            for (lens_index, lens) in lens_box.iter().enumerate() {
                focusing_power += (1 + u64::try_from(box_index).unwrap())
                    * (1 + u64::try_from(lens_index).unwrap())
                    * u64::from(lens.get_focal_length().unwrap());
            }
        }
        focusing_power
    }
}
