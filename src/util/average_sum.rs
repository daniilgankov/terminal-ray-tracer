use std::{
    collections::VecDeque,
    fmt::{Display, Formatter, Result},
};

pub(crate) struct AverageSum {
    values: VecDeque<f32>,
    count: usize,
    sum: f32,
}

impl AverageSum {
    pub(crate) fn new(count: usize) -> Self {
        Self {
            values: VecDeque::with_capacity(count),
            count,
            sum: 0.0,
        }
    }

    pub(crate) fn add(&mut self, value: f32) {
        let len = self.values.len();
        debug_assert!(len <= self.count);
        if len == self.count {
            let pop = self.values.pop_front().unwrap();
            self.sum -= pop;
        }
        self.values.push_back(value);
        self.sum += value;
    }

    pub(crate) fn value(&self) -> f32 {
        let len = self.values.len();
        if len == 0 { 0.0 } else { self.sum / len as f32 }
    }
}

impl Display for AverageSum {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        self.value().fmt(f)
    }
}

#[cfg(test)]
mod test {
    use crate::util::assert_nearly_eq_f32;

    use super::AverageSum;

    #[test]
    fn test() {
        let mut sum = AverageSum::new(3);
        assert!(assert_nearly_eq_f32(sum.value(), 0.0));
        sum.add(2.0);
        assert!(assert_nearly_eq_f32(sum.value(), 2.0));
        sum.add(3.0);
        assert!(assert_nearly_eq_f32(sum.value(), 2.5));
        sum.add(4.0);
        assert!(assert_nearly_eq_f32(sum.value(), 3.0));
        sum.add(5.0);
        assert!(assert_nearly_eq_f32(sum.value(), 4.0));
    }
}
