pub fn evaluate(s: String, knowledge: Vec<Vec<String>>) -> String {
    let map: std::collections::HashMap<&[u8], &[u8]> = knowledge
        .iter()
        .map(|i| (i[0].as_bytes(), i[1].as_bytes()))
        .collect();

    let s = s.as_bytes();
    let mut buf: Vec<u8> = Vec::with_capacity(s.len());

    let mut i = 0;
    while i < s.len() {
        if s[i] == b'(' {
            let j = i + 1;
            let end = j + s[j..].iter().position(|&b| b == b')').unwrap();

            match map.get(&s[j..end]) {
                Some(value) => buf.extend_from_slice(value),
                None => buf.push(b'?'),
            }

            i = end + 1;
        } else {
            let end = i + s[i..]
                .iter()
                .position(|&b| b == b'(')
                .unwrap_or(s.len() - i);
            buf.extend_from_slice(&s[i..end]);
            i = end;
        }
    }

    String::from_utf8(buf).unwrap()
}
