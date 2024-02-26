use std::error::Error;

use super::csv_ops::read_csv;

pub fn lsrem() -> Result<(), Box<dyn Error>> {
    read_csv()
}
