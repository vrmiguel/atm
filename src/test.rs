use crate::atm::Atm;
use crate::error::Result;

#[test]
fn must_return_two_configurations() -> Result<()> {
    let bills = vec![200, 100, 50, 20, 10];
    let bill_amounts = vec![10, 10, 10, 10, 10];
    let atm = Atm::new(&bills, &bill_amounts)?;
    let configurations = atm.configurations(300);
    
    assert!(
        configurations.higher_bills_sum() < configurations.lower_bills_sum()
    );

    Ok(())
}

#[test]
#[should_panic]
fn bills_must_be_descending() {
    let unordered_bills = [200, 100, 20, 50, 10];
    let bill_amounts = [10, 10, 10, 10, 10];

    Atm::new(&unordered_bills, &bill_amounts).unwrap();
}

#[test]
fn test_correct_solution() -> Result<()> {
    let bills = [100, 50, 20, 10];
    let bill_amounts = [10, 10, 10, 10];
    let initial_variation = [0; 4];
    let withdrawal_amount = 300;
    let result =
        Atm::new(&bills, &bill_amounts)?
            .solutions(&initial_variation, withdrawal_amount, 0)?;
    
    assert!(result.iter().any(|solution| vec_equals(solution, &[2, 2, 0, 0])));
    assert!(result.iter().any(|solution| vec_equals(solution, &[1, 3, 2, 1])));
    Ok(())
}



fn vec_equals<T: Copy + PartialEq>(a: &[T], b: &[T]) -> bool {
    if a.len() != b.len() {
        return false;
    }

    a.iter().zip(b).all(|(&a, &b)| a == b)
}
