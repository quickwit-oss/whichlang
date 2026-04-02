use std::io;
use std::io::BufRead;
use std::io::BufReader;

use whichlang::Lang;

fn main() -> io::Result<()> {
    let label_codes: Vec<&'static str> = whichlang::LANGUAGES
        .iter()
        .map(|lang| lang.three_letter_code())
        .collect();
    let mut line = String::new();
    let stdin = io::stdin();
    let mut stdinlocked = BufReader::new(stdin.lock());
    let mut total = 0;
    let mut error = 0;
    loop {
        line.clear();

        if stdinlocked.read_line(&mut line)? == 0 {
            break;
        }
        let trimmed_line = line.trim_end().trim_matches('\\');
        let id_label_sentence: Vec<&str> = trimmed_line.splitn(3, "\t").collect();
        if !label_codes.contains(&id_label_sentence[1]) {
            continue;
        }
        let detected: Lang = whichlang::detect_language(id_label_sentence[2]).unwrap_or(Lang::Eng);
        total += 1;
        if detected.three_letter_code() != id_label_sentence[1] {
            error += 1;
            println!(
                "{} {} {} : {}",
                id_label_sentence[0],
                id_label_sentence[1],
                id_label_sentence[2],
                detected.three_letter_code()
            );
            println!("precision: {}", 1.0 - (error as f64 / total as f64));
        }
    }
    Ok(())
}
