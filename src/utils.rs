#[allow(dead_code)]

/// return the column count and row count
/// for example A1:I37 means have 9 column and 37 row
pub(crate) fn parse_dimension_ref(value: &str) -> (usize, usize) {
    let value: Vec<_> = value.split(":").collect();
    assert_eq!(value.len(), 2);
    let start = value[0];
    let end = value[1];
    let start_col = start
        .chars()
        .filter(|c| c.is_ascii_alphabetic())
        .collect::<String>();
    let start_col = column_name_to_number(start_col.as_str()) as usize;
    let start_row = start
        .chars()
        .filter(|c| c.is_ascii_digit())
        .collect::<String>()
        .parse::<usize>()
        .unwrap();
    let end_col = end
        .chars()
        .filter(|c| c.is_ascii_alphabetic())
        .collect::<String>();
    let end_col = column_name_to_number(end_col.as_str()) as usize;
    let end_row = end
        .chars()
        .filter(|c| c.is_ascii_digit())
        .collect::<String>()
        .parse::<usize>()
        .unwrap();

    (end_col - start_col + 1, end_row - start_row + 1)
}

fn column_name_to_number(column_name: &str) -> u32 {
    column_name
        .bytes()
        .fold(0, |acc, byte| acc * 26 + u32::from(byte) - 64)
}

#[cfg(test)]
mod test {
    use crate::utils::{column_name_to_number, parse_dimension_ref};

    #[test]
    fn check_column_name_to_number() {
        assert_eq!(column_name_to_number("A"), 1);
        assert_eq!(column_name_to_number("B"), 2);
        assert_eq!(column_name_to_number("C"), 3);
        assert_eq!(column_name_to_number("Z"), 26);
        assert_eq!(column_name_to_number("AA"), 27);
        assert_eq!(column_name_to_number("AB"), 28);
        assert_eq!(column_name_to_number("AC"), 29);
        assert_eq!(column_name_to_number("AZ"), 52);
        assert_eq!(column_name_to_number("BA"), 53);
    }

    #[test]
    fn check_parse_dimension_ref() {
        assert_eq!(parse_dimension_ref("A1:I37"), (9, 37));
    }
}
