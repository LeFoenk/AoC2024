use std::fs;

fn main() {
    let file_path = "input.txt";

    let content = fs::read_to_string(file_path).unwrap();

    let table = CharTable::new(&content);

    let result = table.count_xmas();

    println!("Part 1: {}", result);

    let result_2 = table.count_mas_crosses();

    println!("Part 2: {}", result_2);
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

    fn count_mas_crosses(&self) -> usize {
        let mut result = 0;

        for (cursor_y, row) in self.data.iter().enumerate() {
            for (cursor_x, character) in row.iter().enumerate() {
                if *character == 'A' {
                    result += self.find_mas_cross(cursor_x, cursor_y);
                }
            }
        }

        result
    }

    fn find_mas_cross(&self, cursor_x: usize, cursor_y: usize) -> usize {
        if cursor_x == 0
            || cursor_x >= (self.data[0].len() - 1)
            || cursor_y == 0
            || cursor_y >= (self.data.len() - 1)
        {
            return 0;
        }

        let top_left = self.data[cursor_y - 1][cursor_x - 1];
        let bot_right = self.data[cursor_y + 1][cursor_x + 1];
        match top_left {
            'M' => {
                if bot_right != 'S' {
                    return 0;
                }
            }
            'S' => {
                if bot_right != 'M' {
                    return 0;
                }
            }
            _ => {
                return 0;
            }
        }

        let top_right = self.data[cursor_y - 1][cursor_x + 1];
        let bot_left = self.data[cursor_y + 1][cursor_x - 1];
        match top_right {
            'M' => {
                if bot_left != 'S' {
                    return 0;
                }
            }
            'S' => {
                if bot_left != 'M' {
                    return 0;
                }
            }
            _ => {
                return 0;
            }
        }

        return 1;
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
