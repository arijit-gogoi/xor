fn xor_chars(l: char, r: char) -> Option<char> {
    // Convert chars to u32 (Unicode scalar value) representations.
    // Conerting to u8 or u16 won't do.
    let l = l as u32;
    let r = r as u32;
    let xor_res = l ^ r;

    // Attempt to convert the u32 result back to a char.
    // char::from_u32 returns an Option<char> because
    // not all u32 values are valid Unicode scalar values.
    char::from_u32(xor_res)
}

fn main() {
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
        print!("\n---------------------------------------------------------------\n\n");
        assert_eq!(cleartext, decipher);
    }

    let large_val1 = 0x10FFFF; // Max valid Unicode scalar value
    let large_val2 = 0x000001; // Small value
    let char_large1 = char::from_u32(large_val1).expect("hardcoded");
    let char_large2 = char::from_u32(large_val2).expect("hardcoded"); // This will be '\x01'

    let cipher = xor_chars(char_large1, char_large2).expect("hardcoded");
    let decipher = xor_chars(cipher, char_large2).expect("hardcoded");

    assert_eq!(char_large1, decipher);
}
