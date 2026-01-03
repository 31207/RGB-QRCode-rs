use qrcode::EcLevel;

#[derive(Debug, Clone, Copy)]
pub enum ErrorCorrection {
    Low,
    Med,
    High,
    Max,
}

impl ErrorCorrection {
    pub fn to_ec_level(self) -> EcLevel {
        match self {
            ErrorCorrection::Low => EcLevel::L,
            ErrorCorrection::Med => EcLevel::M,
            ErrorCorrection::High => EcLevel::Q,
            ErrorCorrection::Max => EcLevel::H,
        }
    }
}
