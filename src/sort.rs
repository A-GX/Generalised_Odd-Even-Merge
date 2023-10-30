use crate::*;

use super::merge::odd_even_merge;
use super::necessary_traits::CompareSwap;
use std::cmp::min;

//Need to be verified

pub fn odd_even_merge_sort<T: CompareSwap>(db:&mut T, start: usize, len: usize) {
    if len > 1 {
        let len_right = len - (len/2);
        odd_even_merge_sort(db,start, len_right); // sort right
        odd_even_merge_sort(db, start+len_right, len-len_right); // sort left

        /*
         * Initialisation to call odd_even_merge :
         *  - We compute len of left and right subdb n_l and n_r 
         *  - We give two subdb : 
         *      + the left one starting at index "start", having n_l elements with position of i+1th element being at position of element i + step (=1)
         *      + the right one starting at index "start+n_l" etc...
         */
        let n_l = len/2 + min(1,len%2); // in case we cannot divide by two, we want the longest one in right
        let n_r = len - n_l;

        //println!("Merge {}", len);

        odd_even_merge(db, n_l, n_r, start, start+n_l, 1); 
    }

    // we always steop the recursion after calling on len = 1, so we will see everyone. 
    else if ty == FEAT {
        let mut test = db.rows[start].feat.eq(zero).reveal().expect("comparison failed").inner();
        while test == 1 {
            db.rows.remove(start);
            if start<db.rows.len() {
                test = db.rows[start].feat.eq(zero).reveal().expect("comparison failed").inner();
            } else {
                test = 0;
            }
        }
        // Need to add offset to value here.
        db.rows[start].feat = db.rows[start].feat-db.dist;
        //println!("WARNING :: OFFSET NOT ADDED YET ! NOT SECURE !");
    }
}