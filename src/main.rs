#[allow(dead_code)]
fn xor_chars(l: char, r: char) -> Result<char, String> {
    // Convert chars to u32 (Unicode scalar value) representations.
    // Conerting to u8 or u16 won't do.
    let l = l as u32;
    let r = r as u32;
    let xor_res = l ^ r;

    // Attempt to convert the u32 result back to a char.
    // char::from_u32 returns an Option<char> because
    // not all u32 values are valid Unicode scalar values.
    char::from_u32(xor_res).ok_or_else(|| format!("Invalid Unicode scalar value: {}", xor_res))
}

/// Encrypts or decrypts a sentence using a key with the XOR cipher.
///
/// This sentence iterates over each character of the input sentence
/// and performs XOR with a character from the key. The key is cycled if
/// the sentence is longer than the key, ensuring that the cipher can
/// handle inputs longer than the key itself.
fn cipher(sentence: &str, key: &str) -> String {
    // If the key is empty, cannot cipher.
    // Return an empty String.
    if key.is_empty() {
        return String::new();
    }

    // Create a cycling iterator over the key's characters.
    // `cycle()` ensures that if `sentence` is longer than `key`, the key characters
    // will repeat from the beginning, effectively "stretching" the key.
    let mut key_chars = key.chars().cycle();

    // Use `map` to transform each character of the sentence
    // and `collect` to gather the results into a new `String`.
    sentence
        .chars()
        .map(|cleartext_char| -> char {
            // Get the next character from the cycling key iterator.
            let key_char = key_chars.next().expect("Non-empty key.");

            // Perform the XOR operation.
            // `expect()` is used here for simplicity, assuming `xor_chars` will not
            // fail with valid character inputs. In a production system, more robust
            // error propagation (e.g., returning a `Result<String, String>`)
            // might be preferred for `cipher` as well.
            xor_chars(cleartext_char, key_char)
                .expect("XOR operation on valid sentence char and key char")
        })
        .collect() // Collect the characters into a string.
}

fn main() {
    let sentence = "Hey Mister, Tambourine Man, play a song for me.";
    let encryption_key = "youthought this was gona b easy but it ain't wasy. buum shaka lakw.";
    let message = &cipher(sentence, encryption_key);
    let deciphered_sentence = cipher(message, encryption_key);
    assert_eq!(deciphered_sentence, sentence);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_song() {
        let sentence = "Hey Mister, Tambourine Man, play a song for me.";
        let encryption_key = "youthought this was gona b easy but it ain't wasy. buum shaka lakw.";
        let message = &cipher(sentence, encryption_key);
        let deciphered_sentence = cipher(message, encryption_key);
        assert_eq!(deciphered_sentence, sentence);
    }

    #[test]
    fn test_xor_chars_list() {
        let key = 'a';
        let chars = ['a', 'b', 'c', 'λ'];

        for character in chars {
            let cleartext = character;
            let cipher: char = xor_chars(cleartext, key).expect("hardcoded correctly");
            let decipher: char = xor_chars(cipher, key).expect("correct flow");

            println!(
                "XOR of cleartext {:#?} ({:#x}) and key {:#?} ({:#x}) is cipher {:#?} ({:#x})",
                cleartext, cleartext as u32, key, key as u32, cipher, cipher as u32
            );
            assert_eq!(cleartext, decipher);
        }
    }

    #[test]
    fn large_chars() {
        let large_val1 = 0x10FFFF; // Max valid Unicode scalar value
        let large_val2 = 0x000001; // Small value
        let char_large1 = char::from_u32(large_val1).expect("hardcoded");
        let char_large2 = char::from_u32(large_val2).expect("hardcoded"); // This will be '\x01'

        let cipher = xor_chars(char_large1, char_large2).expect("hardcoded");
        let decipher = xor_chars(cipher, char_large2).expect("hardcoded");

        assert_eq!(char_large1, decipher);
    }
}
