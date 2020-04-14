type Data = Vec<(Vec<f64>, Vec<f64>)>;

#[derive(Debug)]
pub struct Dataset {
    data: Data,
}

impl From<Data> for Dataset {
    fn from(data: Data) -> Self {
        Self { data }
    }
}

impl Dataset {
    pub fn from_csv(
        file_path: impl AsRef<std::path::Path>,
        includes_headers: bool,
        num_inputs: usize,
    ) -> Result<Self, ParseCsvError> {
        use std::str::FromStr;

        let file = std::fs::File::open(file_path)?;
        let mut reader = csv::ReaderBuilder::new()
            .has_headers(includes_headers)
            .from_reader(file);

        let data: Result<Data, ParseCsvError> = reader
            .records()
            .map(|row| {
                let row = row?;
                let row = row
                    .iter()
                    .map(|val| {
                        let val = val.trim();
                        f64::from_str(val)
                    })
                    .collect::<Result<Vec<_>, _>>()?;

                let mut input = row;
                let output = input.split_off(num_inputs);
                Ok((input, output))
            })
            .collect();
        Ok(Dataset::from(data?))
    }
}

#[derive(thiserror::Error, Debug)]
pub enum ParseCsvError {
    #[error("failed to read file")]
    Read(#[from] std::io::Error),
    #[error("failed to parse CSV")]
    Parse(#[from] csv::Error),
    #[error("failed to convert value into float")]
    Convert(#[from] std::num::ParseFloatError),
}