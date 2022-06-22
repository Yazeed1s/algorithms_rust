
#[allow(dead_code)]
fn bubble_sort<T: std::cmp::Ord>(nums: &mut [T]) -> &mut [T] {
    let mut swapped: bool;
    for i in 0..nums.len() {
        swapped = false;
        for j in 0..nums.len() - i - 1 {
            if nums[j] > nums[j + 1] {
                nums.swap(j, j + 1);
                swapped = true;
            }
        }
        if !swapped {
            break;
        }
    }
    nums
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::Rng;
    #[test]
    fn basic_test() {
        let nums: &mut [i32; 5] = &mut [2, 3, 4, 1, 5];
        let _sorted: &mut [i32] = bubble_sort(nums);
        assert_eq!(_sorted, [1, 2, 3, 4, 5]);
    }
    #[test]
    fn test_bubble_sort_1000s() {
        
        let mut rng = rand::thread_rng();
        let nums: &mut [i32; 10000] = &mut [rng.gen(); 10000];
        bubble_sort(nums);
        for i in 0..nums.len() - 1 {
            assert!(nums[i] <= nums[i + 1]);
        }
    }
}
