use std::io;
use std::io::BufRead;
use std::io::BufReader;
use std::io::BufWriter;
use std::io::Write;

use whichlang::emit_tokens;
use whichlang::DIMENSION;

fn main() -> io::Result<()> {
    let language_codes = whichlang::LANGUAGES
        .iter()
        .map(|lang| lang.three_letter_code())
        .collect::<Vec<&str>>();
    let mut line = String::new();
    let mut features: Vec<usize> = vec![0; whichlang::DIMENSION];
    let stdin = io::stdin();
    let mut stdinlocked = BufReader::new(stdin.lock());
    let stdout = io::stdout();
    let mut stdoutlock = BufWriter::new(stdout.lock());
    write!(&mut stdoutlock, "id,label")?;
    for i in 0..whichlang::DIMENSION {
        write!(&mut stdoutlock, ",feature{i}")?;
    }
    writeln!(&mut stdoutlock)?;
    loop {
        line.clear();
        if stdinlocked.read_line(&mut line)? == 0 {
            break;
        }
        let trimmed_line = line.trim_end().trim_matches('\\');
        features.fill(0);
        let id_label_sentence: Vec<&str> = trimmed_line.splitn(3, "\t").collect();
        if !language_codes.contains(&id_label_sentence[1]) {
            continue;
        }
        let sentence: &str = id_label_sentence[2];
        emit_tokens(sentence, |token| {
            features[token.to_hash() as usize % DIMENSION] += 1;
        });
        write!(
            &mut stdoutlock,
            "{},{}",
            id_label_sentence[0], id_label_sentence[1]
        )?;
        for i in 0..whichlang::DIMENSION {
            let feature = features[i];
            write!(&mut stdoutlock, ",{feature}")?;
        }
        write!(&mut stdoutlock, "\n")?;
        line.clear();
    }
    Ok(())
}
