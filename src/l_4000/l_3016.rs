/// 1. count the frequency of each character in the word
/// 2. sort the characters by frequency DESC
/// 3. gather each presses of characters offset by its location in phone keypad -> due to only 8
///    number of keys.
pub fn minimum_pushes(word: String) -> i32 {
    let word = word.as_bytes();
    let mut freq = [0i32; 26];
    for &c in word {
        freq[(c - b'a') as usize] += 1;
    }

    freq.sort_by(|a, b| b.cmp(a));

    let mut res = 0i32;
    for i in 0..26 {
        if freq[i] == 0 {
            break;
        }

        res += (i as i32 / 8 + 1) * freq[i];
    }

    res
}
