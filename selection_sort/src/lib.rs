#[cfg(test)]
mod tests {
    #[allow(unused_variables,dead_code)]
    fn selection_sort(nums: &mut [i32]) -> &mut [i32] {
    
        for i in 0..nums.len(){
            let mut min_idx: usize = i;
            for j in (i + 1)..nums.len(){
                if nums[j] < nums[min_idx]{
                    min_idx = j;
                }
            }
            nums.swap(min_idx, i);
        }
        nums
    }
    #[test]
    fn it_works() {
        let nums: &mut [i32] = &mut [3,5,9,2,1,4,7,6,8];
        let sorted: &mut [i32] = selection_sort(nums);
        assert_eq!(sorted, [1,2,3,4,5,6,7,8,9]);
    }
}
