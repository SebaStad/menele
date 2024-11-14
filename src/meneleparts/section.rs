use crate::meneleparts::leftmenele::MeneleSectionLeft;
use crate::meneleparts::rightmenele::MeneleSectionRight;

pub enum MeneleSection {
    left(MeneleSectionLeft),
    right(MeneleSectionRight)
}