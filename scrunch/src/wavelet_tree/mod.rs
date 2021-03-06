pub trait WaveletTree {
    fn new(s: &[usize]) -> Self;

    fn len(&self) -> usize;

    fn rank_q(&self, x: usize, q: usize) -> usize;
    fn select_q(&self, x: usize, q: usize) -> usize;
}

#[derive(Debug)]
pub struct ReferenceWaveletTree {
    string: Vec<usize>,
}

impl WaveletTree for ReferenceWaveletTree {
    fn new(s: &[usize]) -> Self {
        Self { string: s.to_vec() }
    }

    fn len(&self) -> usize {
        self.string.len()
    }

    fn rank_q(&self, x: usize, q: usize) -> usize {
        let mut rank: usize = 0;
        for i in 0..self.string.len() {
            if i == x {
                return rank;
            }
            if self.string[i] == q {
                rank += 1;
            }
        }
        rank
    }

    fn select_q(&self, x: usize, q: usize) -> usize {
        let mut rank: usize = 0;
        for i in 0..self.string.len() {
            if self.string[i] == q {
                if rank == x {
                    return i;
                }
                rank += 1;
            }
        }
        //XXX panic("OH NO");
        self.string.len()
    }
}

#[cfg(test)]
mod tests {
    pub use super::*;

    fn simple_evens<WT: WaveletTree>(new: fn(&[usize]) -> WT) {
        // try 010101
        let wt = new(&[0, 1, 0, 1, 0, 1]);
        assert_eq!(0, wt.rank_q(0, 1));
        assert_eq!(0, wt.rank_q(1, 1));
        assert_eq!(1, wt.rank_q(2, 1));
        assert_eq!(1, wt.rank_q(3, 1));
        assert_eq!(2, wt.rank_q(4, 1));
        assert_eq!(2, wt.rank_q(5, 1));
        assert_eq!(3, wt.rank_q(6, 1));

        assert_eq!(0, wt.rank_q(0, 0));
        assert_eq!(1, wt.rank_q(1, 0));
        assert_eq!(1, wt.rank_q(2, 0));
        assert_eq!(2, wt.rank_q(3, 0));
        assert_eq!(2, wt.rank_q(4, 0));
        assert_eq!(3, wt.rank_q(5, 0));
        assert_eq!(3, wt.rank_q(6, 0));

        assert_eq!(1, wt.select_q(0, 1));
        assert_eq!(3, wt.select_q(1, 1));
        assert_eq!(5, wt.select_q(2, 1));

        assert_eq!(0, wt.select_q(0, 0));
        assert_eq!(2, wt.select_q(1, 0));
        assert_eq!(4, wt.select_q(2, 0));
    }

    fn simple_odds<WT: WaveletTree>(new: fn(&[usize]) -> WT) {
        // try 101010
        let wt = new(&[1, 0, 1, 0, 1, 0]);
        assert_eq!(0, wt.rank_q(0, 1));
        assert_eq!(1, wt.rank_q(1, 1));
        assert_eq!(1, wt.rank_q(2, 1));
        assert_eq!(2, wt.rank_q(3, 1));
        assert_eq!(2, wt.rank_q(4, 1));
        assert_eq!(3, wt.rank_q(5, 1));
        assert_eq!(3, wt.rank_q(6, 1));

        assert_eq!(0, wt.rank_q(0, 0));
        assert_eq!(0, wt.rank_q(1, 0));
        assert_eq!(1, wt.rank_q(2, 0));
        assert_eq!(1, wt.rank_q(3, 0));
        assert_eq!(2, wt.rank_q(4, 0));
        assert_eq!(2, wt.rank_q(5, 0));
        assert_eq!(3, wt.rank_q(6, 0));

        assert_eq!(0, wt.select_q(0, 1));
        assert_eq!(2, wt.select_q(1, 1));
        assert_eq!(4, wt.select_q(2, 1));

        assert_eq!(1, wt.select_q(0, 0));
        assert_eq!(3, wt.select_q(1, 0));
        assert_eq!(5, wt.select_q(2, 0));
    }

    fn bug_31_select_q_0_1<WT: WaveletTree>(new: fn(&[usize]) -> WT) {
        let wt = new(&[3, 1]);
        assert_eq!(1, wt.select_q(0, 1));
    }

    // TODO(rescrv): Test proper bounds
    //  assert_eq!(6, wt.select_q(3, 1));
    //  should panic

    macro_rules! test_WaveletTree {
        ($name:ident, $WT:tt) => {
            mod $name {
                use super::*;

                #[test]
                fn simple_evens() {
                    super::simple_evens($WT::new);
                }

                #[test]
                fn simple_odds() {
                    super::simple_odds($WT::new);
                }

                #[test]
                fn bug_31_select_q_0_1() {
                    super::bug_31_select_q_0_1($WT::new);
                }
            }
        };
    }

    test_WaveletTree!(reference, ReferenceWaveletTree);
}
