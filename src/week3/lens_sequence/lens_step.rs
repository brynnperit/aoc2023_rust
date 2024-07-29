#[derive(Clone)]
pub struct LensStep {
    label: String,
    step_type: LensStepType,
    whole_string: String,
}

impl LensStep {
    pub fn from_string(lens_string: &str) -> Option<Self> {
        match lens_string.split_once('=') {
            Some((label, focal_length_str)) => Some(LensStep {
                label: label.to_string(),
                step_type: LensStepType::AddOrReplace(focal_length_str.parse().unwrap()),
                whole_string: lens_string.to_string(),
            }),
            None => lens_string.split_once('-').map(|(label, _)| LensStep {
                label: label.to_string(),
                step_type: LensStepType::Remove,
                whole_string: lens_string.to_string(),
            }),
        }
    }

    pub fn get_hash(&self) -> u8 {
        Self::hash_str(&self.whole_string)
    }

    pub fn get_label_hash(&self) -> u8 {
        Self::hash_str(&self.label)
    }

    fn hash_str(to_hash: &str) -> u8 {
        let mut hash_value: u16 = 0;
        for byte in to_hash.bytes() {
            hash_value += u16::from(byte);
            hash_value *= 17;
            hash_value %= 256;
        }
        hash_value.try_into().unwrap()
    }

    pub fn perform_operation<'a>(&'a self, lens_box: &mut Vec<&'a LensStep>) {
        match self.step_type {
            LensStepType::AddOrReplace(_) => {
                if let Some((lens_index, _)) = lens_box
                    .iter()
                    .enumerate()
                    .find(|(_, lens)| lens.label == self.label)
                {
                    lens_box[lens_index] = self;
                } else {
                    lens_box.push(self);
                }
            }
            LensStepType::Remove => {
                if let Some((lens_index, _)) = lens_box
                    .iter()
                    .enumerate()
                    .find(|(_, lens)| lens.label == self.label)
                {
                    lens_box.remove(lens_index);
                }
            }
        }
    }

    pub fn get_focal_length(&self) -> Option<u8> {
        match self.step_type {
            LensStepType::AddOrReplace(focal_length) => Some(focal_length),
            LensStepType::Remove => None,
        }
    }
}

#[derive(Clone, Copy)]
enum LensStepType {
    AddOrReplace(u8),
    Remove,
}
