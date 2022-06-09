
fn binary_search<T>(nums: &[T], key: &T) -> Result<usize, usize>
where
    T: std::cmp::Ord,
{
    let mut low: i32 = 0;
    let mut high: i32 = nums.len() as i32 - 1;
    while low <= high {
        let mid: i32 = (((high + low) / 2) as f64).floor() as i32;
        if let Some(index) = nums.get(mid as usize) {
            if *index == *key {
                return Ok(mid as usize);
            }
            if *index > *key {
                high = mid - 1;
            }
            if *index < *key {
                low = mid + 1;
            }
        }
    }
    Err(0)
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn finds_a_value_in_the_middle_of_an_array() {
        let nums: [i32; 7] = [1, 3, 4, 6, 8, 9, 11];
        assert_eq!(binary_search(&nums, &6), Ok(3));
    }

    #[test]
    fn finds_a_value_at_the_beginning_of_an_array() {
        let nums: &mut [i32] = &mut [1, 3, 4, 6, 8, 9, 11];
        assert_eq!(binary_search(nums, &1), Ok(0));
    }

    #[test]
    fn finds_a_value_at_the_end_of_an_array() {
        let nums: [i32; 7] = [1, 3, 4, 6, 8, 9, 11];
        assert_eq!(binary_search(&nums, &11), Ok(6));
    }

    #[test]
    fn finds_a_value_in_an_array_of_odd_length() {
        let nums: [i32; 13] = [1, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233, 377, 634];
        assert_eq!(binary_search(&nums, &144), Ok(9));
    }

    #[test]
    fn finds_a_value_in_an_array_of_even_length() {
        let nums: [i32; 13] = [1, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233, 377, 634];
        assert_eq!(binary_search(&nums, &21), Ok(5));
    }

    #[test]
    fn identifies_that_a_value_is_not_included_in_the_array() {
        let nums: [i32; 7] = [1, 3, 4, 6, 8, 9, 11];
        assert_eq!(binary_search(&nums, &7), Err(0));
    }

    #[test]
    fn a_value_smaller_than_the_arrays_smallest_value_is_not_included() {
        let nums: [i32; 7] = [1, 3, 4, 6, 8, 9, 11];
        assert_eq!(binary_search(&nums, &0), Err(0));
    }

    #[test]
    fn a_value_larger_than_the_arrays_largest_value_is_not_included() {
        let nums: [i32; 7] = [1, 3, 4, 6, 8, 9, 11];
        assert_eq!(binary_search(&nums, &13), Err(0));
    }

    #[test]
    fn nothing_is_included_in_an_empty_array() {
        let nums: [_; 0] = [];
        assert_eq!(binary_search(&nums, &1), Err(0));
    }

    #[test]
    fn nothing_is_found_when_the_left_and_right_bounds_cross() {
        let nums: [i32; 2] = [1, 2];
        assert_eq!(binary_search(&nums, &0), Err(0));
    }

    #[test]
    #[feature("generic")]
    fn works_for_vec() {
        let vector: Vec<i32> = vec![6];
        assert_eq!(binary_search(&vector, &6), Ok(0));
        assert_eq!(binary_search(&vector, &6), Ok(0));
    }

    #[test]
    #[feature("generic")]
    fn works_for_char_elements() {
        let chars: [char; 2] = ['a', 'b'];
        assert_eq!(binary_search(&['a'], &'a'), Ok(0));
        assert_eq!(binary_search(&chars, &'b'), Ok(1));
    }

    #[test]
    #[feature("generic")]
    fn works_for_str_elements() {
        let strs: [&str; 2] = ["here", "there"];
        assert_eq!(binary_search(&["abc"], &"abc"), Ok(0));
        assert_eq!(binary_search(&strs, &"there"), Ok(1));
    }
}
