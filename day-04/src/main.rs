use std::fs;

fn main() {
    let file_path = "input.txt";

    let content = fs::read_to_string(file_path).unwrap();

    let table = CharTable::new(&content);

    let result = table.count_xmas();

    println!("Part 1: {}", result);
}

struct CharTable {
    data: Vec<Vec<char>>,
}

impl CharTable {
    fn new(content: &String) -> CharTable {
        let mut content = content.chars();
        let mut table = CharTable { data: vec![vec![]] };
        let mut line = 0;
        while let Some(character) = content.next() {
            if character == '\n' {
                line += 1;
                table.data.push(vec![]);
            } else {
                table.data[line].push(character);
            }
        }

        if table.data[line].len() == 0 {
            _ = table.data.pop();
        }
        table
    }

    fn count_xmas(&self) -> usize {
        let mut result = 0;
        let mut cursor_x = 0;
        let mut cursor_y = 0;

        for row in self.data.iter() {
            for character in row.iter() {
                if *character == 'X' {
                    result += self.find_xmas(cursor_x, cursor_y);
                }

                cursor_x += 1;
            }
            cursor_x = 0;
            cursor_y += 1;
        }

        result
    }

    fn find_xmas(&self, cursor_x: usize, cursor_y: usize) -> usize {
        let mut result = 0;

        // TOP
        if cursor_y > 2 {
            if self.data[cursor_y - 1][cursor_x] == 'M' {
                if self.data[cursor_y - 2][cursor_x] == 'A' {
                    if self.data[cursor_y - 3][cursor_x] == 'S' {
                        result += 1;
                    }
                }
            }
        }

        // TOP-RIGHT
        if cursor_y > 2 && (cursor_x + 3) < self.data[0].len() {
            if self.data[cursor_y - 1][cursor_x + 1] == 'M' {
                if self.data[cursor_y - 2][cursor_x + 2] == 'A' {
                    if self.data[cursor_y - 3][cursor_x + 3] == 'S' {
                        result += 1;
                    }
                }
            }
        }

        // RIGHT
        if (cursor_x + 3) < self.data[0].len() {
            let row = &self.data[cursor_y];

            if row[cursor_x + 1] == 'M' {
                if row[cursor_x + 2] == 'A' {
                    if row[cursor_x + 3] == 'S' {
                        result += 1;
                    }
                }
            }
        }

        // BOT RIGHT
        if (cursor_y + 3) < self.data.len() && (cursor_x + 3) < self.data[0].len() {
            if self.data[cursor_y + 1][cursor_x + 1] == 'M' {
                if self.data[cursor_y + 2][cursor_x + 2] == 'A' {
                    if self.data[cursor_y + 3][cursor_x + 3] == 'S' {
                        result += 1;
                    }
                }
            }
        }

        // BOT
        if (cursor_y + 3) < self.data.len() {
            if self.data[cursor_y + 1][cursor_x] == 'M' {
                if self.data[cursor_y + 2][cursor_x] == 'A' {
                    if self.data[cursor_y + 3][cursor_x] == 'S' {
                        result += 1;
                    }
                }
            }
        }

        // BOT LEFT
        if (cursor_y + 3) < self.data.len() && cursor_x > 2 {
            if self.data[cursor_y + 1][cursor_x - 1] == 'M' {
                if self.data[cursor_y + 2][cursor_x - 2] == 'A' {
                    if self.data[cursor_y + 3][cursor_x - 3] == 'S' {
                        result += 1;
                    }
                }
            }
        }

        // LEFT
        if cursor_x > 2 {
            if self.data[cursor_y][cursor_x - 1] == 'M' {
                if self.data[cursor_y][cursor_x - 2] == 'A' {
                    if self.data[cursor_y][cursor_x - 3] == 'S' {
                        result += 1;
                    }
                }
            }
        }

        // TOP LEFT
        if cursor_x > 2 && cursor_y > 2 {
            if self.data[cursor_y - 1][cursor_x - 1] == 'M' {
                if self.data[cursor_y - 2][cursor_x - 2] == 'A' {
                    if self.data[cursor_y - 3][cursor_x - 3] == 'S' {
                        result += 1;
                    }
                }
            }
        }
        result
    }
}
