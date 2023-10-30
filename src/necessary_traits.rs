/// ### Description
/// We expect you to run the merge or sorting function on a list-like structure.
/// For generalisation sake, you need to generate the structure yourself such as
/// to be able to define the comparison and swapping operations for any sort of 
/// data you may be using.
/// 
/// ### Example
/// ```no_run
/// use generalised-odd-even-merge::necessary_traits::CompareSwap;
/// 
/// // (1) Create your data structure
/// #[derive(Debug)]
/// pub struct DataBase {
///     pub row: Vec<usize>,
/// }
/// 
/// // (2) Implement the CompareSwap type for you structure
/// impl CompareSwap for DataBase {
///     fn compare_swap(&mut self, a: usize, b: usize) -> std::io::Result<()> {
///         let cmp:usize = if self.row[a]>=self.row[b] {0} else {1};
///         let tempo_a = self.row[a]*cmp + self.row[b]*(1-cmp);
///         self.row[b] = self.row[b]*cmp + self.row[a]*(1-cmp);
///         self.row[a] = tempo_a;
///         Ok(())
///     }
/// }
/// ```
pub trait CompareSwap {
    fn compare_swap(&mut self, a: usize, b: usize) -> std::io::Result<()>;
}