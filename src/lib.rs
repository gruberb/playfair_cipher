use crate::key_table::KeyTable;
use crate::plaintext::Plaintext;
use std::str::FromStr;

mod key_table;
mod plaintext;

/// Encrypt a plaintext with a given key
pub fn encrypt(key: &str, text: &str) -> String {
    let prepared_plaintext = Plaintext::from_str(text).unwrap();
    let encrypion_table = KeyTable::new(key);

    let mut res = String::new();

    (0..prepared_plaintext.0.len()).step_by(2).for_each(|i| {
        let c1 = encrypion_table.find_by_char(prepared_plaintext.0.chars().nth(i).unwrap());
        let c2 = encrypion_table.find_by_char(prepared_plaintext.0.chars().nth(i + 1).unwrap());

        if c1.x == c2.x {
            // Rule 1: Same row
            res.push(encrypion_table.get_by_enc_rule_same_row(c1.x, c1.y));
            res.push(encrypion_table.get_by_enc_rule_same_row(c2.x, c2.y));
        } else if c1.y == c2.y {
            // Rule 2: Same column
            res.push(encrypion_table.get_by_enc_rule_same_column(c1.x, c1.y));
            res.push(encrypion_table.get_by_enc_rule_same_column(c2.x, c2.y));
        } else {
            // Rule 3: Rectangle
            let (l1, l2) = encrypion_table.get_by_rule_rectangle(c1, c2);
            res.push(l1);
            res.push(l2);
        }
    });

    res
}

/// Decrypting a ciphertext to a plaintext with a given key
pub fn decrypt(key: &str, ciphertext: &str) -> String {
    let prepared_plaintext = ciphertext.to_ascii_uppercase();
    let encrypion_table = KeyTable::new(key);
    let mut res = String::new();

    (0..ciphertext.len()).step_by(2).for_each(|i| {
        let c1 = encrypion_table.find_by_char(prepared_plaintext.chars().nth(i).unwrap());
        let c2 = encrypion_table.find_by_char(prepared_plaintext.chars().nth(i + 1).unwrap());

        if c1.x == c2.x {
            // Rule 1: Same row
            res.push(encrypion_table.get_by_dec_rule_same_row(c1.x, c1.y));
            res.push(encrypion_table.get_by_dec_rule_same_row(c2.x, c2.y));
        } else if c1.y == c2.y {
            // Rule 2: Same column
            res.push(encrypion_table.get_by_dec_rule_same_column(c1.x, c1.y));
            res.push(encrypion_table.get_by_dec_rule_same_column(c2.x, c2.y));
        } else {
            // Rule 3: Rectangle
            let (l1, l2) = encrypion_table.get_by_rule_rectangle(c1, c2);
            res.push(l1);
            res.push(l2);
        }
    });

    res
}

#[cfg(test)]
mod tests {
    use crate::key_table::Coordinate;
    use crate::plaintext::Plaintext;
    use crate::KeyTable;
    use crate::{decrypt, encrypt};
    use std::str::FromStr;

    #[test]
    fn plaintext_with_padding() {
        let plaintext = "HELLO";
        let r = Plaintext::from_str(plaintext).unwrap();
        assert_eq!(r, Plaintext::from_str("HELZLO").unwrap());
    }

    #[test]
    fn setup_key_table() {
        let table = KeyTable {
            0: vec![
                Coordinate {
                    x: 1,
                    y: 1,
                    content: 'H',
                },
                Coordinate {
                    x: 2,
                    y: 1,
                    content: 'E',
                },
                Coordinate {
                    x: 3,
                    y: 1,
                    content: 'L',
                },
                Coordinate {
                    x: 4,
                    y: 1,
                    content: 'O',
                },
                Coordinate {
                    x: 5,
                    y: 1,
                    content: 'T',
                },
                Coordinate {
                    x: 1,
                    y: 2,
                    content: 'R',
                },
                Coordinate {
                    x: 2,
                    y: 2,
                    content: 'A',
                },
                Coordinate {
                    x: 3,
                    y: 2,
                    content: 'B',
                },
                Coordinate {
                    x: 4,
                    y: 2,
                    content: 'C',
                },
                Coordinate {
                    x: 5,
                    y: 2,
                    content: 'D',
                },
                Coordinate {
                    x: 1,
                    y: 3,
                    content: 'F',
                },
                Coordinate {
                    x: 2,
                    y: 3,
                    content: 'G',
                },
                Coordinate {
                    x: 3,
                    y: 3,
                    content: 'I',
                },
                Coordinate {
                    x: 4,
                    y: 3,
                    content: 'K',
                },
                Coordinate {
                    x: 5,
                    y: 3,
                    content: 'M',
                },
                Coordinate {
                    x: 1,
                    y: 4,
                    content: 'N',
                },
                Coordinate {
                    x: 2,
                    y: 4,
                    content: 'P',
                },
                Coordinate {
                    x: 3,
                    y: 4,
                    content: 'Q',
                },
                Coordinate {
                    x: 4,
                    y: 4,
                    content: 'S',
                },
                Coordinate {
                    x: 5,
                    y: 4,
                    content: 'U',
                },
                Coordinate {
                    x: 1,
                    y: 5,
                    content: 'V',
                },
                Coordinate {
                    x: 2,
                    y: 5,
                    content: 'W',
                },
                Coordinate {
                    x: 3,
                    y: 5,
                    content: 'X',
                },
                Coordinate {
                    x: 4,
                    y: 5,
                    content: 'Y',
                },
                Coordinate {
                    x: 5,
                    y: 5,
                    content: 'Z',
                },
            ],
        };
        let key = "HELLO THERE";
        let r = KeyTable::new(key);
        assert_eq!(r, table);
    }

    #[test]
    fn encrypt_plaintext() {
        let plaintext = "instruments";
        let key = "monarchy";
        let encrypted_string = encrypt(key, plaintext);
        assert_eq!(
            encrypted_string,
            "gatlmzclrqtx".to_string().to_ascii_uppercase()
        );
    }

    #[test]
    fn decrypt_plaintext() {
        let ciphertext = "gatlmzclrqtx";
        let key = "monarchy";
        let decrypted_string = decrypt(key, ciphertext);
        assert_eq!(
            decrypted_string,
            "instrumentsz".to_string().to_ascii_uppercase()
        );
    }
}
