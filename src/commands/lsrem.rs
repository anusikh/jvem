use std::error::Error;

use crate::utils::csv_ops::read_csv;

pub fn lsrem() -> Result<(), Box<dyn Error>> {
    read_csv()
}
