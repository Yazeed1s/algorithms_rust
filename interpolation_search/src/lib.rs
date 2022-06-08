use std::cmp;

fn interpolation_search<Ordering>(nums: &[i32], key: &i32) -> Result<usize, usize> {
    // early check
    if nums.is_empty() {
        return Err(0);
    }
    let mut low: usize = 0;
    let mut high: usize = nums.len() - 1;
    while low <= high {
        if *key < nums[low] || *key > nums[high] {
            break;
        }
        let offset: usize =
        low + (((high - low) / (nums[high] - nums[low]) as usize) * (key - nums[low]) as usize);
        match nums[offset].cmp(&*key) {
            cmp::Ordering::Equal => return Ok(offset),
            cmp::Ordering::Less => low = offset + 1,
            cmp::Ordering::Greater => high = offset - 1,
        }
        // if nums[offset] == *key {
        //     return Ok(offset);
        // } else if nums[offset] < *key {
        //     low = offset + 1;
        // } else {
        //     high = offset - 1;
        // }
    }
    Err(0)
}

#[cfg(test)]
mod tests {
    use crate::interpolation_search;
    use std::cmp::Ordering;

    #[test]
    fn it_works() {
        let nums = [1, 2, 3, 4, 5, 6];
        assert_eq!(interpolation_search::<Ordering>(&nums, &3), Ok(2));
    }
    #[test]
    fn it_works2() {
        let nums = [1, 2, 3, 4, 5, 6];
        assert_eq!(interpolation_search::<Ordering>(&nums, &6), Ok(5));
    }
}
