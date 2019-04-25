use crate::common::{Element, ElementType, Isa, Segment};

pub mod TwoThousandA {
    use super::*;
    pub struct TwoThousand {
        pub hl: Segment,
        pub two_thousand_one_hundred: TwoThousandOneHundred,
        pub two_thousand_two_hundred: TwoThousandTwoHundred,
    }
    pub struct TwoThousandOneHundred {
        pub nm1: Segment,
    }
    pub struct TwoThousandTwoHundred {
        pub trn: Segment,
        pub dtp: Vec<Segment>,
    }

}

pub mod TwoThousandB {
    use super::*;
    pub struct TwoThousand {
        pub hl: Segment,
        pub two_thousand_one_hundred: TwoThousandTwoHundred,
        pub two_thousand_two_hundred: TwoThousandTwoHundred,
    }
    pub struct TwoThousandOneHundred {
        pub nm1: Segment,
    }
    pub struct TwoThousandTwoHundred {
        pub trn: Segment,
        pub stc: Segment,
        pub qty: Vec<Segment>,
        pub amt: Vec<Segment>,
    }
}

pub mod TwoThousandC {
    use super::*;
    pub struct TwoThousand {
        pub hl: Segment,
        pub two_thousand_one_hundred: TwoThousandTwoHundred,
        pub two_thousand_two_hundred: TwoThousandTwoHundred,
    }

    pub struct TwoThousandOneHundred {
        pub nm1: Segment,
    }
    pub struct TwoThousandTwoHundred {
        pub trn: Segment,
        pub stc: Segment,
        pub ref_seg: Segment,
        pub qty: Vec<Segment>,
        pub amt: Vec<Segment>,
    }
}

pub mod TwoThousandD {
    use super::*;

    pub struct TwoThousand {
        pub hl: Segment,
        pub se: Segment,
    }

    pub struct TwoThousandOneHundred {
        pub nm1: Segment,
    }
    pub struct TwoThousandTwoHundred {
        pub trn: Segment,
        pub stc: Segment,
        pub ref_seg: [Segment; 3],
        pub dtp: Segment,
        pub two_thousand_two_hundred_twenty_d: TwoThousandTwoHundredTwenty,
    }

    pub struct TwoThousandTwoHundredTwenty {
        pub svc: Segment,
        pub stc: Segment,
        pub ref_seg: [Segment; 2],
        pub dtp: Segment,
    }

}

pub struct TwoSevenSeven {
    pub isa: Option<Isa>,
    pub st: Option<Segment>,
    pub two_thousand_a: Option<TwoThousandA::TwoThousand>,
    pub two_thousand_b: Option<TwoThousandB::TwoThousand>,
    pub two_thousand_c: Option<TwoThousandC::TwoThousand>,
    pub two_thousand_d: Option<TwoThousandD::TwoThousand>,
}

impl TwoSevenSeven {
    fn new() -> TwoSevenSeven {
        TwoSevenSeven {
            isa: None,
            st: None,
            two_thousand_a: None,
            two_thousand_b: None,
            two_thousand_c: None,
            two_thousand_d: None,
        }
    }

    fn add_isa(&mut self, isa: Isa) {
        self.isa = Some(isa);
    }

    fn add_st(&mut self, st: Segment) {
        self.st = Some(st);
    }

    fn add_two_thousand_a(&mut self, sub_loop: TwoThousandA::TwoThousand) {
        self.two_thousand_a = Some(sub_loop);
    }

    fn add_two_thousand_b(&mut self, sub_loop: TwoThousandB::TwoThousand) {
        self.two_thousand_b = Some(sub_loop);
    }

    fn add_two_thousand_c(&mut self, sub_loop: TwoThousandC::TwoThousand) {
        self.two_thousand_c = Some(sub_loop);
    }

    fn add_two_thousand_d(&mut self, sub_loop: TwoThousandD::TwoThousand) {
        self.two_thousand_d = Some(sub_loop);
    }
}
