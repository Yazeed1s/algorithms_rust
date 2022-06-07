#[cfg(test)]
mod tests {
    fn bubble_sort(nums: &mut [i32]) -> &mut [i32] {
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
    #[test]
    fn it_works() {
        let nums: &mut [i32; 5] = &mut [2, 3, 4, 1, 5];
        let _sorted: &mut [i32] = bubble_sort(nums);
        assert_eq!(_sorted, [1, 2, 3, 4, 5]);
    }
}
