pub fn annotate(minefield: &[&str]) -> Vec<String> {
    if minefield.is_empty() {
        return Vec::new();
    }

    let mut vec = Vec::new();
    for s in minefield {
        vec.push(s.as_bytes());
    }

    let board = Board::new(&vec);

    process_board(board)
}

enum CellType {
    Empty(usize),
    Mine,
}

struct Position {
    x: i32,
    y: i32,
}

struct Cell {
    cell_type: CellType,
    position: Position,
}

struct Board {
    cells: Vec<Vec<Cell>>,
    height: i32,
    width: i32,
}

impl Board {
    fn new(input: &[&[u8]]) -> Self {
        let mut cells = Vec::new();
        for (x, row) in input.iter().enumerate() {
            let mut inner_vec = Vec::new();
            for (y, col) in row.iter().enumerate() {
                match col {
                    b' ' => {
                        let cell_type = CellType::Empty(0);
                        let position = Position {
                            x: x as i32,
                            y: y as i32,
                        };
                        inner_vec.push(Cell {
                            cell_type,
                            position,
                        })
                    }
                    b'*' => {
                        let cell_type = CellType::Mine;
                        let position = Position {
                            x: x as i32,
                            y: y as i32,
                        };
                        inner_vec.push(Cell {
                            cell_type,
                            position,
                        })
                    }
                    _ => {}
                }
            }
            cells.push(inner_vec);
        }

        let height = input.len() as i32;
        let width = input[0].len() as i32;

        Board {
            cells,
            height,
            width,
        }
    }

    fn process_cell(&mut self, position: &Position) {
        let cell = &self.cells[position.x as usize][position.y as usize];
        if let CellType::Mine = cell.cell_type {
            let x = position.x as i32;
            let y = position.y as i32;
            for r in x - 1..=x + 1 {
                for c in y - 1..=y + 1 {
                    if let Some(p) = self.get_position(r, c) {
                        Board::update(self.cell_from_position(&p));
                    }
                }
            }
        }
    }

    fn cell_from_position<'a>(&'a mut self, position: &Position) -> &'a mut Cell {
        let Position { x, y } = *position;

        &mut self.cells[x as usize][y as usize]
    }

    fn update(cell: &mut Cell) {
        if let CellType::Empty(v) = &mut cell.cell_type {
            *v += 1;
        }
    }

    fn get_position(&self, x: i32, y: i32) -> Option<Position> {
        if x >= 0 && x < self.height as i32 && y >= 0 && y < self.width as i32 {
            return Some(Position { x, y });
        }
        None
    }
}

fn process_board(mut board: Board) -> Vec<String> {
    for x in 0..board.height {
        for y in 0..board.width {
            board.process_cell(&Position { x, y });
        }
    }

    board.into()
}

impl From<Board> for Vec<String> {
    fn from(b: Board) -> Self {
        let mut result = Vec::new();
        for row in b.cells {
            let mut s = String::new();
            for cell in row {
                match cell.cell_type {
                    CellType::Empty(0) => s.push(' '),
                    CellType::Empty(c) => s.push_str(&*c.to_string()),
                    CellType::Mine => s.push('*'),
                }
            }
            result.push(s);
        }

        result
    }
}
