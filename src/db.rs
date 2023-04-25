use std::usize;

use crate::fstring::FString;

pub const COLUMN_USERNAME_SIZE: usize = 32;
pub const COLUMN_EMAIL_SIZE: usize = 255;

pub struct Row {
    pub id: u32,
    pub username: FString<COLUMN_USERNAME_SIZE>,
    pub email: FString<COLUMN_EMAIL_SIZE>,
}

impl std::str::FromStr for Row {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut a = s.split_ascii_whitespace();

        Ok(Row {
            id: a
                .next()
                .ok_or("id not found")?
                .parse::<u32>()
                .or(Err("could not parse id to uint"))?,
            username: FString::from_str(a.next().ok_or("username not found")?)?,
            email: FString::from_str(a.next().ok_or("email not found")?)?,
        })
    }
}

impl std::fmt::Display for Row {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.id, self.username, self.email)
    }
}
