
fn shell_sort<T>(nums: &mut [T]) -> &mut [T]
where
    T: std::cmp::PartialOrd +  Copy,
{
    let mut size: usize = nums.len() / 2;
    while size > 0 {
        for i in size..nums.len() {
            let temp = nums[i as usize];
            let mut j: usize = i;
            while j >= size && nums[j - size] > temp {
                nums[j] = nums[j - size];
                j -= size;
            }
            nums[j] = temp;
        }
        size /= 2;
    }
    nums
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
        shell_sort(&mut nums);
        for i in 0..nums.len() - 1 {
            assert!(nums[i] <= nums[i + 1]);
        }
    }
    #[test]
    fn test_shell_sort_with_100_random_u16s() {
        let mut nums = [0u16; 100];
        rand::thread_rng().fill(&mut nums[..]);
        shell_sort(&mut nums);
        for i in 0..nums.len() - 1 {
            assert!(nums[i] <= nums[i + 1]);
        }
    }

    #[test]
    fn test_shell_sort_with_100_random_f32s() {
        let mut nums = [0f32; 100];
        rand::thread_rng().fill(&mut nums[..]);
        shell_sort(&mut nums);
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
        shell_sort(&mut chars);
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
        shell_sort(&mut strs);
        for i in 0..strs.len() - 1 {
            assert!(strs[i] <= strs[i + 1]);
        }
    }
}