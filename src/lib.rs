#[cfg(test)]
#[macro_use]
extern crate quickcheck;


#[cfg(test)]
mod tests {
    use quickcheck::{Arbitrary, Gen, QuickCheck};
    use std::cmp::min;

    #[derive(Debug, Clone)]
    pub enum Op {
        FillSmallJug,
        FillBigJug,
        EmptySmallJug,
        EmptyBigJug,
        SmallToBig,
        BigToSmall,
    }

    #[derive(Debug,Default, Clone)]
    pub struct State {
        big: usize,
        small: usize,
    }

    impl State {
        fn apply(&mut self, op: &Op) {
            match op {
                &Op::FillSmallJug => {
                    self.small = 3;
                }
                &Op::FillBigJug => self.big = 5,
                &Op::EmptySmallJug => self.small = 0,
                &Op::EmptyBigJug => self.big = 0,
                &Op::SmallToBig => {
                    let old = self.clone();
                    self.big = min(old.big + self.small, 5);
                    self.small -= self.big - old.big
                }

                &Op::BigToSmall => {
                    let old = self.clone();
                    self.small = min(old.big + self.small, 3);
                    self.big -= self.small - old.small
                }
            }
        }

        fn assert_invariants(&self) {
            assert!(self.big <= 5);
            assert!(self.small <= 3);
        }
        fn finished(&self) -> bool {
            self.big == 4
        }
    }
    impl Arbitrary for Op {
        fn arbitrary<G: Gen>(g: &mut G) -> Op {
            static OPS: &'static [Op] = &[Op::FillSmallJug,
                                          Op::FillBigJug,
                                          Op::EmptySmallJug,
                                          Op::EmptyBigJug,
                                          Op::SmallToBig,
                                          Op::BigToSmall];
            let a = u32::arbitrary(g);
            let x = a % 6;
            return OPS[x as usize].clone();
        }
    }

    #[test]
    fn diehard() {
        fn prop(xs: Vec<Op>) -> bool {
            // println!("{:?}", xs);
            let mut sts = Vec::new();
            let mut st = State::default();
            for o in xs.iter() {
                st.apply(o);
                sts.push((o.clone(), st.clone()));
                st.assert_invariants();
                if st.finished() {
                    println!("match: {:#?}", sts);
                    return false;
                }
            }
            return true;
        }
        QuickCheck::new().tests(10000).quickcheck(prop as fn(Vec<Op>) -> bool);
    }
}
