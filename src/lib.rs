use chrono::NaiveDate;
use std::iter::Iterator;

struct BoundedDateRange {
    value: NaiveDate,
    end: NaiveDate
}

impl BoundedDateRange {
    fn new(start: NaiveDate, end: NaiveDate) -> BoundedDateRange {
        BoundedDateRange { value: start, end: end }
    }
}

impl Iterator for BoundedDateRange {
    type Item = NaiveDate;

    fn next(&mut self) -> Option<NaiveDate> {
        if self.value > self.end {
            return None
        } else {
            let current = self.value;
            self.value = self.value.succ_opt().unwrap();
            return Some(current);
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_just_three_days() {
        let start = NaiveDate::from_ymd_opt(2020, 1, 1).unwrap();
        let end = NaiveDate::from_ymd_opt(2020, 12, 31).unwrap();

        let dr = BoundedDateRange::new(start, end);

        assert_eq!(dr.collect::<Vec<NaiveDate>>().len(), 366); // leap year
    }

    #[test]
    fn test_make_a_vec() {
        let start = NaiveDate::from_ymd_opt(2020, 2, 28).unwrap();
        let middle = NaiveDate::from_ymd_opt(2020, 2, 29).unwrap();
        let end = NaiveDate::from_ymd_opt(2020, 3, 1).unwrap();

        let dr = BoundedDateRange::new(start, end);

        assert_eq!(dr.collect::<Vec<NaiveDate>>(), vec![start, middle, end]);
    }

    #[test]
    fn test_non_leap_year() {
        let start = NaiveDate::from_ymd_opt(1900, 1, 1).unwrap();
        let end = NaiveDate::from_ymd_opt(1900, 12, 31).unwrap();

        let dr = BoundedDateRange::new(start, end);

        assert_eq!(dr.collect::<Vec<NaiveDate>>().len(), 364);
    }
}
