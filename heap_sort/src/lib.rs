pub fn heap_sort<T: std::cmp::Ord>(nums: &mut [T]) -> &[T] {
    let end = nums.len();
    for start in (0..end / 2).rev() {
        move_down(nums, start, end - 1);
    }
    for end in (1..nums.len()).rev() {
        nums.swap(end, 0);
        move_down(nums, 0, end - 1);
    }
    nums
}

fn move_down<T: std::cmp::Ord>(nums: &mut [T], start: usize, end: usize) {
    let mut root = start;
    loop {
        let mut child = root * 2 + 1; // Get the left child
        if child > end {
            break;
        }
        if child < end && nums[child] < nums[child + 1] {
            // Right child exists and is greater.
            child += 1;
        }
        if nums[root] < nums[child] {
            // If child is greater than root, swap'em!
            nums.swap(root, child);
            root = child;
        } else {
            break;
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use rand::Rng; // add -> rand = "0.8.5" under [dependencies] in Cargo.toml
    #[test]
    fn test_reverced_slice() {
        let mut nums = [0u16; 100];
        rand::thread_rng().fill(&mut nums[..]);
        nums.reverse();
        heap_sort(&mut nums);
        for i in 0..nums.len() - 1 {
            assert!(nums[i] <= nums[i + 1]);
        }
    }
    #[test]
    fn test_shell_sort_with_100_random_u16s() {
        let mut nums = [0u16; 100];
        rand::thread_rng().fill(&mut nums[..]);
        heap_sort(&mut nums);
        for i in 0..nums.len() - 1 {
            assert!(nums[i] <= nums[i + 1]);
        }
    }

    #[test]
    fn test_shell_sort_with_slice_of_chars() {
        let mut chars: [char; 26] = [
            'q', 'a', 'z', 'x', 's', 'w', 'e', 'd', 'c', 'r', 'f', 'v', 't', 'g', 'b', 'y', 'h',
            'n', 'u', 'j', 'm', 'i', 'k', 'o', 'l', 'p',
        ];
        heap_sort(&mut chars);
        for i in 0..chars.len() - 1 {
            assert!(chars[i] <= chars[i + 1]);
        }
    }

    #[test]
    fn test_shell_sort_with_strings() {
        let mut strs: [&str; 26] = [
            "q", "a", "z", "x", "s", "w", "e", "d", "c", "r", "f", "v", "t", "g", "b", "y", "h",
            "n", "u", "j", "m", "i", "k", "o", "l", "p",
        ];
        heap_sort(&mut strs);
        for i in 0..strs.len() - 1 {
            assert!(strs[i] <= strs[i + 1]);
        }
    }
}
