use thiserror::Error;

#[derive(Debug, Error)]
pub enum DataSetError {
    #[error("Failed to read dataset: {0}")]
    Io(#[from] std::io::Error),

    #[error("Failed to parse dataset: {0}")]
    Parse(#[from] ParseDataSetError),

    #[error("Empty DataSet is not supported")]
    Empty,
}

#[derive(Debug, Error)]
#[error("Could not parse DataSet because {kind} on line {line_number}")]
pub struct ParseDataSetError {
    kind: ParseDataSetErrorKind,
    line_number: usize,
}
impl ParseDataSetError {
    pub fn missing_output(line_number: usize) -> Self {
        return Self {
            kind: ParseDataSetErrorKind::MissingOutput,
            line_number,
        };
    }
    pub fn missing_input(line_number: usize) -> Self {
        return Self {
            kind: ParseDataSetErrorKind::MissingInput,
            line_number,
        };
    }
    pub fn too_many_columns(line_number: usize) -> Self {
        return Self {
            kind: ParseDataSetErrorKind::TooManyColumns,
            line_number,
        };
    }
    pub fn parse_value_error<E: std::error::Error + 'static>(
        line_number: usize,
        parse_error: E,
        unparsed_value: String,
    ) -> Self {
        return Self {
            kind: ParseDataSetErrorKind::ParseValueError {
                parse_error: parse_error.into(),
                unparsed_value,
            },
            line_number,
        };
    }
}

#[derive(Debug, Error)]
pub enum ParseDataSetErrorKind {
    #[error("There is an output value missing")]
    MissingOutput,

    #[error("There is an input value missing")]
    MissingInput,

    #[error("There are too many columns for one (input, output) pair")]
    TooManyColumns,

    #[error("Could not parse {unparsed_value} because {parse_error}")]
    ParseValueError {
        parse_error: Box<dyn std::error::Error>,
        unparsed_value: String,
    },
}
