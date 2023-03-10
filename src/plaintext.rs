use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub(crate) struct Plaintext(pub(crate) String);

impl FromStr for Plaintext {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut input = s.to_ascii_uppercase();

        (0..input.len()).step_by(2).for_each(|a| {
            if input.chars().nth(a + 1) == input.chars().nth(a) {
                input.insert_str(a + 1, "Z");
            }
        });

        if input.len() % 2 != 0 {
            if input.as_bytes()[input.len() - 1] == "Z".as_bytes()[0] {
                input.push('X');
            } else {
                input.push('Z');
            }
        }

        Ok(Plaintext(input))
    }
}
