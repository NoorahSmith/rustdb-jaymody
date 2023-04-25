use std::usize;

/* fixed length ascii string */
pub struct FString<const SIZE: usize> {
    pub arr: [u8; SIZE],
}

impl<const SIZE: usize> std::str::FromStr for FString<{ SIZE }> {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if !s.is_ascii() {
            return Err("String contains non-ascii characters.".to_owned());
        }
        if s.len() >= SIZE {
            return Err(format!(
                "String len {} exceeds max length {}",
                s.len(),
                SIZE
            ));
        }

        let mut arr: [u8; SIZE] = [0; SIZE];

        for (i, b) in s.bytes().enumerate() {
            arr[i] = b;
        }

        Ok(FString { arr })
    }
}

impl<const SIZE: usize> std::fmt::Display for FString<{ SIZE }> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        unsafe { write!(f, "{}", std::str::from_utf8_unchecked(&self.arr)) }
    }
}
