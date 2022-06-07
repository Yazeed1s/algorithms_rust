#[cfg(test)]
mod tests {
    fn selection_sort(nums: &mut[i32]) -> &mut[i32] {
        for i in 0..nums.len(){
            let mut min_indx: usize = i;
            for j in (i + 1)..nums.len(){
                if nums[j] < nums[min_indx]{
                    min_indx = j;
                }
            }
            nums.swap(i, min_indx);
        }
        return nums;
    }
    #[test]
    fn it_works() {
        let nums: &mut[i32;5] = &mut [2,3,4,1,5];
        let _sorted: &mut[i32] = selection_sort(nums);
        assert_eq!(_sorted, [1,2,3,4,5]);
    }
}
