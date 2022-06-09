pub fn binary_search<T>(nums: &[T], key: T) -> Result<usize,usize>
where
    T: std::cmp::Ord,
{
    let mut low: i32 = 0;
    let mut high: i32 = nums.len() as i32 - 1;
    while low <= high {
        let mid: i32 = (((high + low) / 2) as f64).floor() as i32;
        if let Some(index) = nums.get(mid as usize) {
            if *index == key {
                return Ok(mid as usize);
            }
            if *index > key {
                high = mid - 1;
            }
            if *index < key {
                low = mid + 1;
            }
        }
    }
    Err(0)
}