use std::slice::SliceIndex;

fn chop(target: i64, haystack: &[i64]) -> Option<usize> {
    if haystack.len() <= 1 {
        return haystack.get(0).filter(|x| *x == &target).map(|_| 0_usize);
    }

    let index = haystack.len() / 2;
    match haystack.get(index) {
        None => None,
        Some(value) if value == &target => Some(index),
        Some(value) if value > &target => chop(target, haystack.split_at(index).0),
        Some(value) if value < &target => {
            chop(target, haystack.split_at(index).1).map(|sub_index| sub_index + index)
        }
        Some(_) => None,
    }
}

#[cfg(test)]
mod tests {
    use crate::chop;

    #[test]
    fn empty_input() {
        assert_eq!(None, chop(3, &[]));
    }

    #[test]
    fn target_not_found() {
        assert_eq!(None, chop(3, &[1]));
    }

    #[test]
    fn single_element() {
        assert_eq!(Some(0), chop(1, &[1]));
    }

    #[test]
    fn first_of_three() {
        assert_eq!(Some(0), chop(1, &[1, 3, 5]));
    }

    #[test]
    fn middle_of_three() {
        assert_eq!(Some(1), chop(3, &[1, 3, 5]));
    }

    #[test]
    fn top_of_three() {
        assert_eq!(Some(2), chop(5, &[1, 3, 5]));
    }

    #[test]
    fn given_test_suite() {
        assert_eq!(None, chop(0, &[1, 3, 5]));
        assert_eq!(None, chop(2, &[1, 3, 5]));
        assert_eq!(None, chop(4, &[1, 3, 5]));
        assert_eq!(None, chop(6, &[1, 3, 5]));
        assert_eq!(Some(0), chop(1, &[1, 3, 5, 7]));
        assert_eq!(Some(1), chop(3, &[1, 3, 5, 7]));
        assert_eq!(Some(2), chop(5, &[1, 3, 5, 7]));
        assert_eq!(Some(3), chop(7, &[1, 3, 5, 7]));
        assert_eq!(None, chop(0, &[1, 3, 5, 7]));
        assert_eq!(None, chop(2, &[1, 3, 5, 7]));
        assert_eq!(None, chop(4, &[1, 3, 5, 7]));
        assert_eq!(None, chop(6, &[1, 3, 5, 7]));
        assert_eq!(None, chop(8, &[1, 3, 5, 7]));
    }
}
