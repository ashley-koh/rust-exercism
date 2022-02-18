#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    if _first_list == _second_list {
        return Comparison::Equal;
    } else if _first_list.len() == 0 && _second_list.len() > 0 {
        return Comparison::Sublist;
    } else if _first_list.len() > 0 && _second_list.len() == 0 {
        return Comparison::Superlist;
    } else if _second_list.len() > _first_list.len() && _second_list.contains(&_first_list[0]) {
        let mut indexes = vec![];

        // Find all potential starting points of sublists
        for (i, item) in _second_list.iter().enumerate() {
            if *item == _first_list[0] {
                indexes.push(i);
            }
        }

        for i in indexes {
            let (_, right) = _second_list.split_at(i);
            if right.starts_with(_first_list) {
                return Comparison::Sublist;
            }
        }
    } else if _first_list.len() > _second_list.len() &&  _first_list.contains(&_second_list[0]) {
        let mut indexes = vec![];

        for (i, item) in _first_list.iter().enumerate() {
            if *item == _second_list[0] {
                indexes.push(i);
            }
        }

        for i in indexes {
            let (_, right) = _first_list.split_at(i);
            if right.starts_with(_second_list) {
                return Comparison::Superlist;
            }
        }
    }
        return Comparison::Unequal;
}