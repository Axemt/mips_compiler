#[allow(dead_code)]
#[derive(Debug)]
pub enum MetadataError {
    NoSegmentData(String),
    Align(String),
}

impl std::fmt::Display for MetadataError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            MetadataError::NoSegmentData(segment_tag) => {
                write!(f, "No segment metadata found for segment {segment_tag}")
            }
            MetadataError::Align(segment_tag) => {
                write!(
                    f,
                    "Base address for \"{segment_tag}\" segment is not word-aligned"
                )
            }
        }
    }
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum CompileError {
    ImmSize,
    RegisterParse(String),
    TagResolution(String),
    AlignmentError(u32, u32, String),
}

impl std::fmt::Display for CompileError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CompileError::ImmSize => write!(f, "Given immediate does not fit in 16b"),
            CompileError::RegisterParse(reg) => write!(f, "Error parsing register {reg}"),
            CompileError::TagResolution(tag) => {
                write!(f, "Unresolved tag \"{tag}\" in compile step")
            }
            CompileError::AlignmentError(alignment, addr, symbol) => {
                write!(
                    f,
                    "type address is not {}-aligned: @ {} ; Symbol: {}",
                    alignment,
                    format!("{:08X}", addr),
                    symbol
                )
            }
        }
    }
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum SyntaxError {
    NoMatchingPair(char),
}

impl std::fmt::Display for SyntaxError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SyntaxError::NoMatchingPair(c) => write!(f, "Syntax error: No {} pair found", c),
        }
    }
}
