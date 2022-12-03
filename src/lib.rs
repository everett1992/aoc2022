use std::num;

pub trait SumCalories: Iterator {
    /// Iterate over an iterator of strings and return the sum of each string (calories) for each
    /// contiguous section separated by an empty string. This will flush the last set if the iterator
    /// does not end with an empty line.
    fn sum_calories(self) -> SumCaloriesImpl<Self>
    where
        Self::Item: ToString,
        Self: Sized,
    {
        SumCaloriesImpl { iter: self }
    }
}

impl<I: Iterator> SumCalories for I {}

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
        while let Some(line) = self.iter.next() {
            let line = line.to_string();
            if line.is_empty() {
                return Some(Ok(sum.unwrap_or(0)));
            }

            let line: u32 = match line.parse() {
                Ok(line) => line,
                Err(err) => return Some(Err(err)),
            };

            sum = Some(sum.unwrap_or(0) + line);
        }
        // The delegate iterator returned None If we saw a line in this call to `next` sum will be
        // Some(sum) and we should return it, just in case the input doesn't end with a blank line.
        //
        // If we never entered the while body we'll return None.
        return sum.map(Ok);
    }
}
