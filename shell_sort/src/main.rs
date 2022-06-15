use rand::Rng;

#[allow(dead_code, unused)]
fn shell_sort<T>(nums: &mut [T]) -> &mut [T]
where
    T: std::cmp::PartialOrd + Copy,
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
fn main() {
        let mut nums = [0u16; 10];
        rand::thread_rng().fill(&mut nums[..]);
        println!("{:?}", nums);
        nums.reverse();
        println!("{:?}", nums);
        // shell_sort(&mut nums);
        // for i in 0..nums.len() - 1 {
        //     assert!(nums[i] <= nums[i + 1]);
        // }
}
