use create::meneleparts::leftmenele::MeneleSectionLeft;
use create::meneleparts::rightmenele::MeneleSectionRight;

enum MeneleSection {
    left(MeneleSectionLeft),
    right(MeneleSectionRight)
}