use itertools::Itertools;

use crate::configurations::Configurations;
use crate::error::{Error, Result};

pub struct Atm<'a> {
    bills: &'a [i32],
    bill_amounts: &'a [i32],
}

impl<'a> Atm<'a> {
    pub fn new(bills: &'a [i32], bill_amounts: &'a [i32]) -> Result<Self> { 

        // Make sure bills and bill_amounts have the same length
        (bills.len() == bill_amounts.len())
            .then(|| ()).ok_or(Error::InvalidParamError)?;

        // Make sure `bills` is in descending order
        let is_descending = bills.into_iter().tuple_windows().all(|(x, y)| x >= y);
        if !is_descending {
            Err(Error::InvalidParamError)?;
        }

        Ok(Self {
            bills,
            bill_amounts,
        })
    }

    pub fn configurations(&self, amount: i32) -> Configurations {
        let initial_variation = [0; 5];
        let solutions = self.solutions(&initial_variation, amount, 0).unwrap();

        Configurations::new(
            solutions.first().expect("Solution was empty!"),
            solutions.last().expect("Solution was empty!")
        )
    }

    fn compute(&self, variation: &'a [i32]) -> i32 {
        self.bills
            .iter()
            .zip(variation)
            .map(|(bill, variation)| bill * variation)
            .sum()
    }

    pub fn solutions(&self, variation: &'a [i32], amount: i32, position: i32) -> Result<Vec<Vec<i32>>> {
        let mut list = vec![];
        let mut idx = position as usize;
        let value = self.compute(variation);
        if value < amount {
            while idx < self.bills.len() {
                if self.bill_amounts[idx] > variation[idx] {
                    let mut new_variation = variation.to_owned();
                    new_variation[idx] += 1;
                    let new_list = Atm::new(self.bills, self.bill_amounts)?.solutions(
                        &new_variation,
                        amount,
                        position,
                    )?;
                    if !new_list.is_empty() {
                        list.extend_from_slice(&new_list);
                    }
                }
                idx += 1;
            }
        } else if value == amount {
            list.push(variation.to_owned());
        }

        Ok(list)
    }


}
