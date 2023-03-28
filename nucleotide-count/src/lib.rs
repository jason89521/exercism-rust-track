use std::collections::HashMap;

fn is_valid(nucleotide: char) -> bool {
    match nucleotide {
        'A' | 'C' | 'G' | 'T' => true,
        _ => false,
    }
}

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    if is_valid(nucleotide) {
        let mut result = 0;
        for ch in dna.chars() {
            match ch {
                ch if ch == nucleotide => {
                    result += 1;
                }
                ch if is_valid(ch) => {}
                ch => return Err(ch),
            }
        }
        Ok(result)
    } else {
        Err(nucleotide)
    }
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    let mut result = HashMap::from([('A', 0), ('C', 0), ('G', 0), ('T', 0)]);
    for ch in dna.chars() {
        if is_valid(ch) {
            result
                .entry(ch)
                .and_modify(|count| *count += 1)
                .or_insert(1);
        } else {
            return Err(ch);
        }
    }

    Ok(result)
}
