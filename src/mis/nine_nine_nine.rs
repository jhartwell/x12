use crate::common::{Element, ElementType, Isa, Segment, Usage};
pub struct NineNineNine {
    pub isa: Isa,
    pub st: Element,
    pub ak1: Element,
    pub loop_2000: Option<Loop2000>,
}

pub struct Loop2000 {
    pub ak2: Element,
    pub loop_2100: Option<Loop2100>,
}

impl Loop2000 {
    pub fn create(ak2: Element) -> Loop2000 {
        Loop2000 {
            ak2,
            loop_2100: None
        }
    }

    pub fn add_loop_2100(&mut self, loop_2100: Loop2100) {
        self.loop_2100 = Some(loop_2100);
    }
}

pub struct Loop2100 {
    pub ik3: Element,
    pub ctx: [Element; 2],
    pub loop_2110: Option<Loop2110>,
}

impl Loop2100 {
    pub fn create(ik3: Element, ctx: [Element; 2]) -> Loop2100 {
        Loop2100 {
            ik3,
            ctx,
            loop_2110: None,
        }
    }

    pub fn add_loop_2110(&mut self, loop_2110: Loop2110) {
        self.loop_2110 = Some(loop_2110);
    }
}

pub struct Loop2110 {
    pub ik4: Element,
    pub ctx: Element,
}

impl Loop2110 {
    pub fn create(ik4: Element, ctx: Element) -> Loop2110 {
        Loop2110 {
            ik4,
            ctx,
        }
    }
}