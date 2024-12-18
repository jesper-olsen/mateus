// convert board move coordinates "d2d4" to int tuple
pub fn str2move(s: &str) -> Option<(usize, usize)> {
    if let Some(frm) = parse_chess_coord(&s[0..2]) {
        if let Some(to) = parse_chess_coord(&s[2..4]) {
            return Some((frm, to));
        }
    }
    None
}

pub fn parse_chess_coord(coord: &str) -> Option<usize> {
    if coord.len() != 2 {
        return None; // 2 characters
    }

    let bytes = coord.as_bytes();
    let file = bytes[0]; // column letter, e.g., 'a'
    let rank = bytes[1]; // row number, e.g., '4'

    // a-h, 1-8
    if file < b'a' || file > b'h' || rank < b'1' || rank > b'8' {
        return None; // Invalid input
    }

    let col = (file - b'a') as usize;
    let row = (rank - b'1') as usize;

    let sq = (7 - col) * 8 + row;
    Some(sq)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_chess_coord_valid() {
        let coord = "a4";
        if let Some(sq) = parse_chess_coord(coord) {
            assert_eq!(
                sq, 59,
                "Failed to parse valid chess coordinate '{}' '{}'",
                coord, sq
            );
        } else {
            panic!(
                "Expected valid chess coordinate, but got None for '{}'",
                coord
            );
        }
    }

    #[test]
    fn test_parse_chess_coord_invalid() {
        let coord = "z9";
        assert!(
            parse_chess_coord(coord).is_none(),
            "Expected None for invalid coordinate '{}'",
            coord
        );
    }
}
