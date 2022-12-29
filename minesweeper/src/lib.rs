use std::str;

#[rustfmt::skip]
const OFFSETS: &'static[(i32, i32)]= &[
    (-1, -1), (-1, 0), (-1, 1),
    (0 , -1),          ( 0, 1),
    (1 , -1), ( 1, 0), ( 1, 1)
];

pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let rows = minefield.len() as i32;
    let minefield: Vec<&[u8]> = minefield.iter().map(|str| str.as_bytes()).collect();
    minefield
        .iter()
        .enumerate()
        .map(|(row, bytes)| {
            let cols = bytes.len() as i32;
            bytes
                .iter()
                .enumerate()
                .map(|(col, _)| {
                    if minefield[row][col] == b'*' {
                        '*'
                    } else {
                        match OFFSETS
                            .iter()
                            .map(|&(row_offset, col_offset)| {
                                (row as i32 + row_offset, col as i32 + col_offset)
                            })
                            .filter(|&(row, col)| row >= 0 && row < rows && col >= 0 && col < cols)
                            .filter(|&(row, col)| minefield[row as usize][col as usize] == b'*')
                            .count()
                        {
                            0 => ' ',
                            n => (n as u8 + b'0') as char,
                        }
                    }
                })
                .collect()
        })
        .collect()
}
