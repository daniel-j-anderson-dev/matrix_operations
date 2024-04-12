use std::{fs::OpenOptions, io::Read, num::NonZeroUsize, path::Path, str::FromStr};

use crate::error::{DataSetError, ParseDataSetError};

pub struct DataPoint<T> {
    input: T,
    output: T,
}
impl<T> DataPoint<T> {
    pub fn input(&self) -> &T {
        return &self.input;
    }
    pub fn output(&self) -> &T {
        return &self.output;
    }
}

pub struct DataSet<T> {
    data: Vec<DataPoint<T>>,
}

impl<T> DataSet<T> {
    pub fn len(&self) -> usize {
        return self.data.len();
    }
    pub fn len_nonzero(&self) -> NonZeroUsize {
        return NonZeroUsize::new(self.len()).expect("Length can not be zero");
    }
    pub fn data(&self) -> &[DataPoint<T>] {
        return &self.data;
    }
    pub fn data_mut(&mut self) -> &mut [DataPoint<T>] {
        return &mut self.data;
    }
}


impl<T> DataSet<T>
where
    T: FromStr,
    T::Err: std::error::Error + 'static,
{
    /// - Must be only 2 columns
    /// - leftmost column is input values
    /// - rightmost column is output values
    /// - each input/output pair is separated by a newline
    /// - example: `input, output\n`
    /// 
    /// ```csv
    ///    4.0,  33.0
    ///    4.5,  42.0
    ///    5.0,  45.0
    ///    5.5,  51.0
    ///    6.0,  53.0
    ///    6.5,  61.0
    ///    7.0,  62.0
    /// ```
    /// 
    pub fn from_csv(path: impl AsRef<Path>) -> Result<Self, DataSetError> {
        let mut file_data = String::new();

        OpenOptions::new()
            .read(true)
            .open(path)?
            .read_to_string(&mut file_data)?;

        let data_set = file_data.parse()?;

        return Ok(data_set);
    }
}

impl<T> FromStr for DataSet<T>
where
    T: FromStr,
    T::Err: std::error::Error + 'static,
{
    type Err = ParseDataSetError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut data = Vec::new();

        for (line_index, line) in s.lines().enumerate() {
            let mut tokens = line.split(',');

            let input = tokens
                .next()
                .ok_or_else(|| ParseDataSetError::missing_input(line_index + 1))?
                .trim();
            let input = input.parse::<T>()
                .map_err(|parse_error| {
                    ParseDataSetError::parse_value_error(line_index + 1, parse_error, input.to_owned())
                })?;

            let output = tokens
                .next()
                .ok_or_else(|| ParseDataSetError::missing_output(line_index + 1))?
                .trim();
            let output = output.parse::<T>()
                .map_err(|parse_error| {
                    ParseDataSetError::parse_value_error(line_index + 1, parse_error, output.to_owned())
                })?;

            if tokens.next().is_some() {
                return Err(ParseDataSetError::too_many_columns(line_index + 1));
            }

            let data_point = DataPoint { input, output };

            data.push(data_point);
        }

        return Ok(Self { data });
    }
}

impl<T, const N: usize> TryFrom<[(T, T); N]> for DataSet<T>
where
    T: FromStr + Copy,
    T::Err: std::error::Error,
{
    type Error = DataSetError;
    fn try_from(value: [(T, T); N]) -> Result<Self, Self::Error> {
        if N == 0 {
            return Err(DataSetError::Empty);
        }

        let mut data = Vec::new();

        for (input, output) in value.into_iter() {
            let data_point = DataPoint { input, output };

            data.push(data_point);
        }

        return Ok(Self { data });
    }
}

impl<T> TryFrom<&[(T, T)]> for DataSet<T>
where
    T: FromStr + Copy,
    T::Err: std::error::Error + 'static,
{
    type Error = DataSetError;
    fn try_from(value: &[(T, T)]) -> Result<Self, Self::Error> {
        if value.len() == 0 {
            return Err(DataSetError::Empty);
        }

        let mut data = Vec::new();

        for (input, output) in value.into_iter().copied() {
            let data_point = DataPoint { input, output };

            data.push(data_point);
        }

        return Ok(Self { data });
    }
}

impl<T: Copy, const N: usize> TryFrom<([T; N], [T; N])> for DataSet<T> {
    type Error = DataSetError;

    fn try_from((inputs, outputs): ([T; N], [T; N])) -> Result<Self, Self::Error> {
        if N == 0 {
            return Err(DataSetError::Empty);
        }

        let data = inputs
            .into_iter()
            .zip(outputs)
            .map(|(input, output)| DataPoint { input, output })
            .collect();

        return Ok(Self { data });
    }
}
