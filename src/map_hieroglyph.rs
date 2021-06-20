pub fn convert(s: &str) -> String {
    let newname = base32::encode(base32::Alphabet::RFC4648 { padding: false }, s.as_bytes());
    newname.chars().map(map).collect::<String>()
}

fn map(c: char) -> &'static str {
    match c {
        'A' => "\u{1313F}",
        'B' => "\u{130C0}",
        'C' => "\u{133A1}",
        'D' => "\u{130A7}",
        'E' => "\u{131CB}",
        'F' => "\u{13191}",
        'G' => "\u{133BC}",
        'H' => "\u{1339B}",
        'I' => "\u{131CB}",
        'J' => "\u{13193}",
        'K' => "\u{133A1}",
        'L' => "\u{130ED}",
        'M' => "\u{13153}",
        'N' => "\u{13216}",
        'O' => "\u{1336F}",
        'P' => "\u{133E4}",
        'Q' => "\u{133D8}",
        'R' => "\u{1308B}",
        'S' => "\u{132F4}",
        'T' => "\u{133CF}",
        'U' => "\u{13171}",
        'V' => "\u{13191}",
        'W' => "\u{13171}",
        'X' => "\u{133A1}\u{132F4}",
        'Y' => "\u{131CB}",
        'Z' => "\u{13283}",
        '2' => "\u{130CA}",
        '3' => "\u{130CB}",
        '4' => "\u{130CC}",
        '5' => "\u{130CD}",
        '6' => "\u{130CE}",
        '7' => "\u{130CF}",
        c => panic!("{}", c),
    }
}
