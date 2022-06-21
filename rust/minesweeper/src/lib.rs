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
            match count_mines(&field, x, y) {
                None => current_string.push('*'),
                Some(0) => current_string.push(' '),
                Some(count) => current_string.push_str(count.to_string().as_str()),
            }
        }
        result.push(current_string);
    }

    result
}

fn count_mines(field: &[&[u8]], x: usize, y: usize) -> Option<usize> {
    if is_mine(field, x, y) {
        return None;
    }

    let height = field.len();
    let width = field[0].len();

    let mut surrounding_mines = 0;
    let (x_lower, x_upper) = (x.saturating_sub(1), (x + 1).min(height - 1));
    let (y_lower, y_upper) = (y.saturating_sub(1), (y + 1).min(width - 1));
    for neighbor_x in x_lower..=x_upper {
        for neighbor_y in y_lower..=y_upper {
            if is_mine(field, neighbor_x, neighbor_y) {
                surrounding_mines += 1;
            }
        }
    }

    Some(surrounding_mines)
}

fn is_mine(field: &[&[u8]], x: usize, y: usize) -> bool {
    field[x][y] == b'*'
}
