#[derive(Debug, Default, PartialEq)]
pub(crate) struct Coordinate {
    pub(crate) x: usize,
    pub(crate) y: usize,
    pub(crate) content: char,
}

#[derive(Debug, Default, PartialEq)]
pub(crate) struct KeyTable(pub(crate) Vec<Coordinate>);

impl KeyTable {
    pub(crate) fn new(key: &str) -> KeyTable {
        let mut table = KeyTable::default();
        let mut input = String::new();

        // I = J
        let alphabet = "ABCDEFGHIKLMNOPQRSTUVWXYZ".to_ascii_uppercase().to_string();

        let mut key = key.to_ascii_uppercase();
        key.retain(|c| !c.is_whitespace());

        key.push_str(alphabet.as_str());

        for c in key.chars() {
            if !input.contains(c) {
                input.push(c);
            }
        }

        let mut ix = 0;

        for y in 1..=5 {
            for x in 1..=5 {
                table.0.push(Coordinate {
                    x,
                    y,
                    content: input.chars().nth(ix).unwrap(),
                });
                ix += 1;
            }
        }

        table
    }

    pub(crate) fn find_by_char(&self, ch: char) -> &Coordinate {
        self.0.iter().find(|c| c.content == ch).unwrap()
    }

    pub(crate) fn get_by_enc_rule_same_row(&self, x: usize, y: usize) -> char {
        self.0
            .iter()
            // If two characters are in the same row, take the letter from
            // below the given character in the key table (and wrap around if it's the last one)
            .find(|c| c.x == x && c.y == (y % 5 + 1))
            .unwrap()
            .content
    }

    pub(crate) fn get_by_dec_rule_same_row(&self, x: usize, y: usize) -> char {
        // Rule 1: Same row

        // Wrap around the table if index is 1 and we need index 5
        let y = (y as i32 - 1 + 5 - 1 as i32).rem_euclid(5) + 1;

        self.0
            .iter()
            // If two characters are in the same row, take the letter from
            // below the given character in the key table (and wrap around if it's the last one)
            .find(|c| c.x == x && c.y == y as usize)
            .unwrap()
            .content
    }

    pub(crate) fn get_by_enc_rule_same_column(&self, x: usize, y: usize) -> char {
        self.0
            .iter()
            // If two characters are in the same column, take the letter next to the
            // given character in the key table (and wrap around if it's the last one)
            .find(|c| c.x == (x % 5 + 1) && c.y == y)
            .unwrap()
            .content
    }

    pub(crate) fn get_by_dec_rule_same_column(&self, x: usize, y: usize) -> char {
        // Wrap around the table if index is 1 and we need index 5
        let x = (x as i32 - 1 + 5 - 1 as i32).rem_euclid(5) + 1;

        self.0
            .iter()
            // If two characters are in the same column, take the letter next to the
            // given character in the key table (and wrap around if it's the last one)
            .find(|c| c.x == x as usize && c.y == y)
            .unwrap()
            .content
    }

    pub(crate) fn get_by_rule_rectangle(&self, c1: &Coordinate, c2: &Coordinate) -> (char, char) {
        // Switch C1 column with C2 column, keep row
        let c1x = c2.x;
        let c1y = c1.y;

        // Switch C2 column with C1 column, keep row
        let c2x = c1.x;
        let c2y = c2.y;

        let l1 = self
            .0
            .iter()
            .find(|c| c.x == c1x && c.y == c1y)
            .unwrap()
            .content;

        let l2 = self
            .0
            .iter()
            .find(|c| c.x == c2x && c.y == c2y)
            .unwrap()
            .content;

        (l1, l2)
    }
}
