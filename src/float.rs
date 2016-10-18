use super::sort::{sort_by};
use num_traits::{Float, zero};
use unreachable::UncheckedOptionExt;

/// Sorts floating point numbers efficiently.
/// The ordering used is
/// | -inf | < 0 | -0 | +0 | > 0 | +inf | NaN |
pub fn sort_floats<T: Float>(v: &mut [T]) {
    /*
     * We don't have hardware support for a total order on floats. NaN is not
     * smaller or greater than any number. We want NaNs to be last, so we could
     * just use is_nan() in the comparison function. It turns out that this hurts
     * performance a lot, and in most cases we probably don't have any NaNs anyway.
     * 
     * The solution is to first move all NaNs to the end of the array, and then
     * sort the remainder with efficient comparisons. After sorting, the zeros
     * might be in the wrong order since -0 and 0 compare equal. We want
     * -0 to be sorted before 0. We binary search to find the interval containing
     * all zeros and perform a counting sort on it.
     */

    if v.len() <= 1 {
        return;
    }

    // First we move all NaNs to the end
    let mut rnan = v.len() - 1;
    // Skip NaNs already in place
    while rnan > 0 && v[rnan].is_nan() {
        rnan -= 1;
    }
    let mut p = rnan;
    while p > 0 {
        p -= 1;
        if v[p].is_nan() {
            v.swap(p, rnan);
            rnan -= 1;
        }
    }

    // Sort the non-NaN part with efficient comparisons
    sort_by(&mut v[..rnan + 1], &|x: &T, y: &T| unsafe {
        x.partial_cmp(y).unchecked_unwrap()
    });


    let left = find_first_zero(&v[..rnan + 1]);

    // Count zeros of each type and then fill them in in the right order
    let mut zeros = 0;
    let mut neg_zeros = 0;
    for x in v[left..].iter() {
        if *x != zero() {
            break;
        }
        if x.is_sign_negative() {
            neg_zeros += 1;
        } else {
            zeros += 1;
        }
    }
    for x in v[left..].iter_mut() {
        if neg_zeros > 0 {
            *x = Float::neg_zero();
            neg_zeros -= 1;
        } else if zeros > 0 {
             *x = zero();
             zeros -= 1;
        } else {
            break;
        }
    }
}

/// Find the first zero in `v`.
/// If there is no zero, it return v.len() - 1
fn find_first_zero<T: Float>(v: &[T]) -> usize {
    if v.len() == 0 { return 0; }
    let mut hi = v.len() - 1;
    let mut left = 0;

    while left < hi {
        let mid = ((hi - left) / 2) + left;
        if v[mid] < zero() {
            left = mid + 1;
        } else {
            hi = mid;
        }
    }

    while left < v.len() && v[left] < zero() {
        left += 1;
    }
    return left;
}
