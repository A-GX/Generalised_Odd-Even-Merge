//! ### Description
//! Recursively sort a list using the generalised Odd-Even merging networg : split the initial list
//! recursively until it obtains list of size 1 (hence sorted) and then feed those sorted list
//! to the merging network.
//! 
//! ### Parameters
//! * list:&mut  T -> mutable structure T. See example in main.rs
//! 
//! * start: usize => the starting index of the (sub-)list to sort. Initial value is usually 0
//! * len: usize => the lenght of the (sub-)list to sort. Intitall value is usually list.len()
//! 
//!  ### Example
//! **/!\\** This example use the DataBase type defined for the example in the ***necessary_traits*** section **/!\\**
//! ```no_run
//! use generalised-odd-even-merge::merge::odd_even_merge;
//! use rand::Rng;
//! 
//! fn is_ordered(list: Vec<usize>) {
//!     let mut res = true;
//!     for i in 0..list.len()-1 {
//!         res = res && list[i] <= list [i+1];
//!     }
//!     assert!(res);
//! }
//! 
//! let mut rng = rand::thread_rng();
//! for j in 2..100 {
//!     for i in 0..j*j {
//!         let mut list = DataBase { row: (0..j).map(|_| rng.gen_range(0..1000)).collect() };
//!         println!("{:?}",list.row);
//!         odd_even_merge_sort(&mut list, 0, j);
//!         print!("{:?} -- i: {} - j: {}\n",list.row,i,j);
//!         is_ordered(list.row);
//!     }
//! }
//! 
//! ```

use super::merge::odd_even_merge;
use super::necessary_traits::CompareSwap;
use std::cmp::min;

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

        odd_even_merge(db, n_l, n_r, start, start+n_l, 1); 
    }
}