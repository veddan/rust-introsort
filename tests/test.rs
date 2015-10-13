#![feature(test)]
#![feature(step_by)]

extern crate introsort;
extern crate test;
extern crate rand;

use introsort::{sort_by, insertion_sort, heapsort};
use rand::{Rng, weak_rng};

macro_rules! do_test_sort(
    ($sortfun:ident) => ({
        let cmp = |a: &usize, b: &usize| a.cmp(b);
        let cmp_rev = |a: &usize, b: &usize| b.cmp(a);
        for len in (4usize .. 250).step_by(5) {
            for _ in 0isize .. 100 {
                let mut v = weak_rng().gen_iter::<u8>().take(len).map(|x| 10 + (x % 89) as usize)
                                        .collect::<Vec<usize>>();
                let mut v1 = v.clone();

                $sortfun(&mut v[..], &cmp);
                assert!(v.windows(2).all(|w| w[0] <= w[1]));

                $sortfun(&mut v1[..], &cmp);
                assert!(v1.windows(2).all(|w| w[0] <= w[1]));

                $sortfun(&mut v1[..], &cmp_rev);
                assert!(v1.windows(2).all(|w| w[0] >= w[1]));
            }
        }
        // shouldn't fail/crash
        let mut v: [usize; 0] = [];
        $sortfun(&mut v[..], &cmp);

        let mut v = [0xDEADBEEFusize];
        $sortfun(&mut v[..], &cmp);
        assert!(v == [0xDEADBEEF]);
    })
);

#[test]
fn test_introsort() {
    do_test_sort!(sort_by);
}

#[test]
fn test_heapsort() {
    do_test_sort!(heapsort);
}

#[test]
fn test_insertion_sort() {
    do_test_sort!(insertion_sort);
}

