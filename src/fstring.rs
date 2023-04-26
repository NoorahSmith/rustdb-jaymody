use std::usize;

/* fixed length ascii string */
pub struct FString<const SIZE: usize> {
    pub arr: [u8; SIZE],
    pub n: usize,
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

        let mut n: usize = 0;
        for (i, b) in s.bytes().enumerate() {
            arr[i] = b;
            n += 1;
        }

        Ok(FString { arr, n })
    }
}

impl<const SIZE: usize> std::fmt::Display for FString<{ SIZE }> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", std::str::from_utf8(&self.arr[0..self.n]).unwrap())
    }
}
