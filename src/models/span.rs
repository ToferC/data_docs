#[derive(Debug, Serialize, Deserialize)]
pub struct Span {
    pub start: usize,
    pub end: usize,
    pub modifier: Modifier,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Modifier {
    Strong,
    Underline,
    Italic,
    Code,
    Redact(Rationale),
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Rationale {
    PersonalInformation,
    PublicInterest,
    LegalAdvice,
    Other(String),
}