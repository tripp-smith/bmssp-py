use std::fmt;

#[derive(Debug, Clone)]
pub enum BmsspError {
    InvalidGraph(String),
    InvalidWeights(String),
    InvalidSource { source: usize, num_vertices: usize },
    InvalidEnabledMask { expected: usize, actual: usize },
    NonFiniteWeight,
    NegativeWeight,
}

impl fmt::Display for BmsspError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BmsspError::InvalidGraph(msg) => write!(f, "Invalid graph: {}", msg),
            BmsspError::InvalidWeights(msg) => write!(f, "Invalid weights: {}", msg),
            BmsspError::InvalidSource { source, num_vertices } => {
                write!(
                    f,
                    "Invalid source vertex {} (graph has {} vertices)",
                    source, num_vertices
                )
            }
            BmsspError::InvalidEnabledMask { expected, actual } => {
                write!(
                    f,
                    "Invalid enabled mask length: expected {}, got {}",
                    expected, actual
                )
            }
            BmsspError::NonFiniteWeight => write!(f, "Non-finite weight encountered"),
            BmsspError::NegativeWeight => write!(f, "Negative weight encountered"),
        }
    }
}

impl std::error::Error for BmsspError {}

pub type Result<T> = std::result::Result<T, BmsspError>;
