#![feature(rustc_private)]
#[macro_use]
mod common;
use common::*;

test_verify_one_file! {
    #[test] test1 code! {
        use crate::pervasive::seq::*;

        #[proof]
        fn test_seq() {
            let s1 = seq_new(5, |i: int| 10 * i);
            assert(s1.len() == 5);
            assert(s1.index(3) == 30);
            let s2 = seq_empty::<int>().push(0).push(10).push(20).push(30).push(40);
            assert(s1.ext_equal(s2));
            assert(equal(s1, s2));
            let s3 = s2.subrange(1, 4);
            assert(s3.len() == 3);
            let s4 = seq_empty::<int>().push(10).push(20).push(30);
            assert(s3.ext_equal(s4));
            let s5 = s3.add(s1);
            assert(s5.len() == 8);
            assert(s5.index(1) == 20);
            assert(s5.index(6) == 30);
        }
    } => Ok(())
}

test_verify_one_file! {
    #[test] test1_fails1 code! {
        use crate::pervasive::seq::*;

        #[proof]
        fn test_seq() {
            let s1 = seq_new(5, |i: int| 10 * i);
            assert(s1.len() == 5);
            assert(s1.index(3) == 30);
            assert(s1.index(5) == 50); // FAILS
        }
    } => Err(err) => assert_one_fails(err)
}

test_verify_one_file! {
    #[test] test1_fails2 code! {
        use crate::pervasive::seq::*;

        #[proof]
        fn test_seq() {
            let s1 = seq_new(5, |i: int| 10 * i);
            assert(s1.len() == 5);
            assert(s1.index(3) == 30);
            let s2 = seq_empty::<int>().push(0).push(10).push(20).push(30).push(40);
            assert(s1.ext_equal(s2));
            assert(equal(s1, s2));
            let s3 = s2.subrange(1, 4);
            assert(s3.len() == 3);
            let s4 = seq_empty::<int>().push(10).push(20).push(30);
            assert(s3.ext_equal(s4));
            let s5 = s3.add(s1);
            assert(s5.len() == 8);
            assert(s5.index(1) == 20);
            assert(s5.index(6) == 30);
            assert(false); // FAILS
        }
    } => Err(err) => assert_one_fails(err)
}