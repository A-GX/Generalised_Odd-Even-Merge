use crate::*;

use super::necessary_traits::CompareSwap;
use std::cmp::min;
use std::io::Result;

/// ###Execute the recursive odd-even merge :
///  * We denote by "position" the index of an element in the original list, and by "index" the index of an element in
/// the sub-list (i.e in when inside a recurisve step). To illustrate why this distinction is usefull : take an element at index 2 in 
/// one of the recursive step, its actual position in the full list can perfectly be 7. In this case, the aforementionned element is
/// considered to be an "even" element in the sub-list built for a recursive step on the "odd" elements of the intial list.
/// 
/// In the following, we describe how the algorithm work. Details exclusive to the work by Batcher are not mentionned, please refer
/// to "Sorting Networks and Applications" by Batcher, 1968.
///  
///  * RECURSIVE RULES: The (sub-)list received by the function are of arbitrary length (i.e without any restriction on the slengths) l and r. We denote 
/// 3 possible situation when receiving the list :
///     (1) Both sub-list are of list 1 : we thus only need to compare-swap the two elements
///     (2) one sub-list is empty : we can stop here. Indeed, both list are considered to be correctly ordered at the start. The remaining list
///         is thus ordered
///     (3) Both list are >=1, and at least one is > 1. In this case, we need to proceed recursively on both sub-list : we thus compute the new strt punt
///         and number of element for each four sub lists (odd and even sublists from the left and right sublists)
/// 
///  * RULES FOR 2 BY 2 COMPARISONS: Once again, we distinguish 3 possible cases :
///     (1) Both elements we need to compare are in the left sub list : In this case, we just need to compareswap elemet at index i and i+1,
///         which translates to elements at position s_l + i*steps and s_l + (i+1)*steps.
///     (2) First element we need to compare is the last element of left sub list, second element is the first element of right sublist :
///         in this case, we just need to compare element at position s_l + i*steps and s_r
///     (3) Both elements are in the right sub list : in this case, we need to compare_swap element at index i and i+1, which translates in
///         comparing elements at position s_r +(i-n_l)*steps and s_r + (i-n_l +1)*steps
/// 
///  * The formal proof of correctness can be found in our paper, "...." by ...
/// 
/// ### Parameters
/// * list:&mut  T : mutable structure T. See example in main.rs
/// 
/// * n_l: usize : length of the left sublist
/// * s_l: usize : starting index of the left sublist
/// 
/// * n_r: usize : length of the right sublist
/// * s_r: usize : starting indec of the right sublist
/// 
/// * step: usize : necessary step to go from an element to the next one INSIDE the sublist (NOT IN BETWEEN SUBLISTS)
pub fn odd_even_merge<T: CompareSwap>(list:&mut T, n_l: usize, n_r: usize, s_l: usize, s_r: usize, step: usize) {
    if (n_l >= 1 && n_r > 1) || (n_l > 1 && n_r >= 1) {
        let n_even_l = n_l/2 + min(1,n_l%2); // number of evens in the right sublist
        let n_even_r = n_r/2 + min(1,n_r%2); // number of evens in the left sublist
        let n_odd_l = n_l/2; // number of odds in the right sublist
        let n_odd_r = n_r/2; // number of odd in the left sublist
        let new_step = step*2; // min(step * 2,s_r-s_l);// goal is that, with final step, s_l+new_step = s_r

        odd_even_merge(list,n_even_l, n_even_r, s_l, s_r, new_step);
        odd_even_merge(list, n_odd_l, n_odd_r, s_l+step, s_r+step, new_step);
        
        let mut i = 1;
        loop {
            print!("i = {:?} -  ",i);
            /* 
             * i is the number of comparison done. the len of the list is n_r+n_l,
             * and we need to do (n-1)/2 comparison (as we don't touch the first element)
             */
            if i+1 >= n_r+n_l { break; } 

            /* 
             * The biggest issue was to go from the left to the right sublist, we distinguish
             * three cases :
             */
            // (1) We went in the right sub-list
            if i >= n_l {
                /*
                 * - index i is now only used to know that we are in right sublist AND to know
                 * when to leave the loop.
                 * - We know use index j to know what elements to compare in the right sublist 
                 */
                list.compare_swap(s_r+(i-n_l)*step, s_r +(i-n_l+1)*step).expect("compare_swap failed !");
            }

            // (2) We compare last of left sub-list with first of right sublist
            else if i+1 == n_l {
                /*
                 * We increment the index j that we will use later if we fully enter right sublist
                 */
                list.compare_swap(s_l+ i*step, s_r).expect("compare_swap failed !");
            }

            // (3) We are only in the left sub-list
            else { 
                // We go through the list the intuitive way
                list.compare_swap(s_l+(i*step), s_l+(i+1)*step).expect("compare_swap failed !");
            }
            i += 2;
            
        }
    }
    
    /*
     * Here, we can also have n_l == 1 && n_l == 0, in which case we don't do anything
     */
    else if (n_l == 1) && (n_r == 1) {
        list.compare_swap(s_l, s_r).expect("compare_swap failed !");
    }
}



