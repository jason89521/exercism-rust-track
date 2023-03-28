pub struct PascalsTriangle(Vec<Vec<u32>>);

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        let mut rows = Vec::with_capacity(row_count as usize);
        for row_size in 0..row_count {
            let mut row = Vec::with_capacity(row_size as usize + 1);
            if row_size == 0 {
                row.push(1);
                rows.push(row);
            } else {
                let prev_row = &rows[row_size as usize - 1];
                for i in 0..=row_size {
                    let a = if i == 0 {
                        0
                    } else {
                        *prev_row.get(i as usize - 1).unwrap_or(&0)
                    };
                    let b = *prev_row.get(i as usize).unwrap_or(&0);
                    row.push(a + b);
                }
                rows.push(row);
            }
        }
        Self(rows)
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        self.0.to_owned()
    }
}
