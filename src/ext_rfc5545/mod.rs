use crate::RecurFrequency;
use rfc5545::{
    Frequency as Freq,
    RecurRulePart as Part,
    RecurrenceRule as Rule,
};

impl RecurFrequency {
    pub fn as_rfc5545_rule(&self) -> Rule {
        match self {
            RecurFrequency::Never => Rule::new_single(),
            RecurFrequency::Daily => Rule::new_with_parts(vec![ Part::Freq(Freq::Daily) ]),
            RecurFrequency::Weekly => Rule::new_with_parts(vec![ Part::Freq(Freq::Weekly) ]),
            RecurFrequency::Monthly => Rule::new_with_parts(vec![ Part::Freq(Freq::Monthly) ]),
            RecurFrequency::Yearly => Rule::new_with_parts(vec![ Part::Freq(Freq::Yearly) ]),

            RecurFrequency::EveryOtherWeek => Rule::new_with_parts(vec![
                Part::Freq(Freq::Weekly),
                Part::Interval(2),
            ]),

            RecurFrequency::TwiceAMonth => Rule::new_with_parts(vec![
                Part::Freq(Freq::Daily),
                Part::Interval(15),
            ]),

            RecurFrequency::Every4Weeks => Rule::new_with_parts(vec![
                Part::Freq(Freq::Weekly),
                Part::Interval(4),
            ]),

            RecurFrequency::EveryOtherMonth => Rule::new_with_parts(vec![
                Part::Freq(Freq::Monthly),
                Part::Interval(2),
            ]),

            RecurFrequency::Every3Months => Rule::new_with_parts(vec![
                Part::Freq(Freq::Monthly),
                Part::Interval(3),
            ]),
            RecurFrequency::Every4Months => Rule::new_with_parts(vec![
                Part::Freq(Freq::Monthly),
                Part::Interval(4),
            ]),

            RecurFrequency::TwiceAYear => Rule::new_with_parts(vec![
                Part::Freq(Freq::Monthly),
                Part::Interval(6),
            ]),

            RecurFrequency::EveryOtherYear => Rule::new_with_parts(vec![
                Part::Freq(Freq::Yearly),
                Part::Interval(2),
            ]),
        }
    }
}