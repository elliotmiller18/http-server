pub const fn max(nums: &[usize], len: usize) -> usize {
    let mut max_elt: usize = nums[0];
    let i = 1;
    while i < len {
        max_elt = max(&[max_elt, nums[i]], 2);
    }
    max_elt
}