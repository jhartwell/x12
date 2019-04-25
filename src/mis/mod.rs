use crate::common::{Element, ElementType, Loop, Segment, SubLoop, Usage};
/* TODO: implement the following formats:
pub struct OneZeroOne {}
pub struct OneFiftyNine {}
pub struct TwoFortyTwo {}
pub struct EightFourteen {}
pub struct EightFifteen {}
pub struct EightSixtyFour {}
pub struct EightSixtyEight {}
pub struct NineNinetySix {}
pub struct NineNineSeven {}
pub struct NineNineEight {}
*/

pub struct NineNineNine {
    pub isa: Segment,
    pub st: Segment,
    pub ak1: Segment,
    pub two_thousand: Loop,
    pub two_thousand_one_hundred: SubLoop,
    pub two_thousand_one_hundred_ten: SubLoop,
    pub ak9: Segment,
    pub se: Segment,
}
