use crate::utils::ValueWithId;

pub struct PenaltyCodesDefinition {
    pub penalties: Vec<PenaltyCode>,
}

impl PenaltyCodesDefinition {
    pub fn add(&mut self, code: PenaltyCode) {
        self.penalties.push(code);
    }
}

pub struct PenaltyCode {
    pub code: String,
    pub verbal_cues: Vec<String>,
}

impl PenaltyCode {
    pub fn new(code: String, verbal_cues: Vec<String>) -> Self {
        Self { code, verbal_cues }
    }
}

impl ValueWithId for PenaltyCode {
    fn get_id(&self) -> String {
        self.code.clone()
    }

    fn get_value(&self) -> String {
        self.verbal_cues.join(",")
    }
}
