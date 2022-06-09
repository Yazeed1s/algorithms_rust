
#[allow(dead_code)]
fn selection_sort<T: std::cmp::Ord>(nums: &mut [T]) -> &mut [T] {
    for i in 0..nums.len() {
        let mut min_indx: usize = i;
        for j in (i + 1)..nums.len() {
            if nums[j] < nums[min_indx] {
                min_indx = j;
            }
        }
        nums.swap(i, min_indx);
    }
    nums
}
#[cfg(test)]
mod tests {
    use super::selection_sort;
    use rand::Rng;
    #[test]
    fn it_works() {
        let nums: &mut [i32; 5] = &mut [2, 3, 4, 1, 5];
        let _sorted: &mut [i32] = selection_sort(nums);
        assert_eq!(_sorted, [1, 2, 3, 4, 5]);
    }
    #[test]
    fn test_bubble_sort_1000s() {
        let mut rng = rand::thread_rng();
        let nums: &mut [i32; 10000] = &mut [rng.gen(); 10000];
        selection_sort(nums);
        for i in 0..nums.len() - 1 {
            assert!(nums[i] <= nums[i + 1]);
        }
    }
}
