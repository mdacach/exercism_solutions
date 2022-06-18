pub fn annotate(minefield: &[&str]) -> Vec<String> {
    if minefield.is_empty() {
        return Vec::new();
    }

    let mut field = Vec::new();
    for s in minefield {
        field.push(s.as_bytes());
    }

    let mut result = Vec::new();
    for (x, row) in field.iter().enumerate() {
        let mut current_string = String::new();
        for (y, col) in row.iter().enumerate() {
            match count_mines(&field, x as i32, y as i32) {
                None => current_string.push('*'),
                Some(0) => current_string.push(' '),
                Some(count) => current_string.push_str(count.to_string().as_str()),
            }
        }
        result.push(current_string);
    }

    result
}

fn count_mines(field: &[&[u8]], x: i32, y: i32) -> Option<usize> {
    if is_mine(field, x, y) {
        return None;
    }

    let mut surrounding_mines = 0;
    for neighbor_x in x - 1..=x + 1 {
        for neighbor_y in y - 1..=y + 1 {
            if is_mine(field, neighbor_x, neighbor_y) {
                surrounding_mines += 1;
            }
        }
    }

    Some(surrounding_mines)
}

fn is_in_bounds(field: &[&[u8]], x: i32, y: i32) -> bool {
    let height = field.len() as i32;
    let width = field[0].len() as i32;

    x >= 0 && x < height && y >= 0 && y < width
}

fn is_mine(field: &[&[u8]], x: i32, y: i32) -> bool {
    is_in_bounds(field, x, y) && field[x as usize][y as usize] == b'*'
}
