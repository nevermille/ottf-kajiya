/// The avar table
mod avar;
/// The unknown tabl
mod unknown;

use crate::error::ParseError;
use crate::traits::Parse;
pub use avar::Avar;
pub use unknown::Unknown;

#[derive(Default, Clone)]
/// The font's tables
pub struct Tables {
    /// `avar` table
    pub avar: Option<Avar>,
    /// Unknown tables
    pub unkn: Vec<Unknown>,
}

impl Tables {
    pub fn add_table(&mut self, tag: &str, data: &[u8]) -> Result<(), ParseError> {
        match tag {
            "avar" => self.avar = Some(Avar::parse(data)?),
            _ => {
                let mut unkn = Unknown::parse(data)?;
                unkn.tag = tag.to_string();
                self.unkn.push(unkn);
            }
        }
        Ok(())
    }
}
