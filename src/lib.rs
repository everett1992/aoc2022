use std::num;

pub struct SumCaloriesImpl<I>
where
    I: Iterator,
    I::Item: ToString,
{
    iter: I,
}

impl<I> Iterator for SumCaloriesImpl<I>
where
    I: Iterator,
    I::Item: ToString,
{
    type Item = Result<u32, num::ParseIntError>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut sum = None;
        while let Some(line) = self.iter.next().filter(|l| !l.to_string().is_empty()) {
            let line = match line.to_string().parse::<u32>() {
                Ok(line) => line,
                Err(err) => return Some(Err(err)),
            };

            sum = Some(sum.unwrap_or(0) + line);
        }
        return sum.map(Ok)
    }
}

pub trait SumCalories: Iterator {
    fn sum_calories(self) -> SumCaloriesImpl<Self>
    where
        Self::Item: ToString,
        Self: Sized,
    {
        SumCaloriesImpl {
            iter: self,
        }
    }
}

impl<I: Iterator> SumCalories for I {}
