use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Pos {
    pub row: usize,
    pub col: usize,
    pub index: usize,
}
impl Pos {
    pub(crate) fn initial() -> Pos {
        Pos {
            row: 1,
            col: 1,
            index: 0,
        }
    }

    pub(crate) fn inc_col(&mut self) {
        self.index += 1;
        self.col += 1;
    }

    pub(crate) fn inc_row(&mut self) {
        self.index += 1;
        self.col = 0;
        self.row += 1;
    }
}

impl Display for Pos {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "line {}, column {}", self.row, self.col)
    }
}

pub struct PosRange(pub(crate) Pos, pub(crate) Pos);

impl PosRange {
    /// Prints and source code area that's been affected
    pub fn print_source(&self, source: &str) {
        for n in self.0.row..self.1.row + 1 {
            if let Some(line) = source.lines().nth(n - 1) {
                print!("{:>4} | {}\n     | ", n, line);
                if n == self.0.row {
                    for _ in 0..self.0.col - 1 {
                        print!(" ");
                    }
                }
                let from: usize;
                let mut to: usize;
                if n == self.0.row && n == self.1.row {
                    from = self.0.col;
                    to = self.1.col;
                    if to == from {
                        to += 1
                    }
                } else if n > self.0.row && n < self.1.row {
                    from = 0;
                    to = line.len();
                } else if n == self.0.row {
                    from = 0;
                    to = line.len() - self.0.col;
                } else {
                    from = 0;
                    to = self.1.col;
                }
                for _ in from..to {
                    print!("^");
                }
                println!();
            }
        }
    }
}

impl Display for PosRange {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.0 == self.1 {
            write!(f, "{}", self.0)
        } else if self.0.row == self.1.row {
            write!(
                f,
                "line {} column {}..{}",
                self.0.row, self.0.col, self.1.col
            )
        } else {
            write!(f, "{} .. {}", self.0, self.1)
        }
    }
}
