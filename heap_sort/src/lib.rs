pub fn heapsort<T: Ord>(arr: &mut [T]) {
    let end = arr.len();
    for start in (0..end / 2).rev() {
       
        sift_down(arr, start, end - 1);
    }
    for end in (1..arr.len()).rev() {
        arr.swap(end, 0);
        sift_down(arr, 0, end - 1);
    }
}

fn sift_down<T: Ord>(arr: &mut [T], start: usize, end: usize) {
    let mut root = start;
    loop {
        let mut child = root * 2 + 1; // Get the left child
        if child > end {
            break;
        }
        if child < end && arr[child] < arr[child + 1] {
            // Right child exists and is greater.
            child += 1;
        }
        if arr[root] < arr[child] {
            // If child is greater than root, swap'em!
            arr.swap(root, child);
            root = child;
        } else {
            break;
        }
    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
