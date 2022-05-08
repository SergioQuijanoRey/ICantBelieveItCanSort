/// The main sorting algorithm
/// It creates a copy of the original values and returns that copy sorted
/// It sorts in non-decreasing order
pub fn ICantBelieveItCanSort<T>(values: Vec<T>) -> Vec<T> where T: Ord + Clone {

    // Start cloning the values into a mutable vector
    let mut new_values = values.clone();

    // Use the sorting algo
    for i in 1..new_values.len(){
        for j in 1..new_values.len(){
            if new_values[i] < new_values[j]{
                let tmp = new_values[i].clone();
                new_values[i] = new_values[j].clone();
                new_values[j] = tmp;
            }
        }
    }

    return new_values;
}


#[cfg(test)]
mod tests {

    use super::ICantBelieveItCanSort;

    // For checking that a vec is sorted in non-decreasing order
    fn check_non_decreasing_sorted<T: Ord>(values: Vec<T>) -> bool{

        for first in 1..values.len(){
            for second in first..values.len(){

                // Check for a possible failure
                if values[first] > values[second]{
                    return false;
                }
            }
        }

        // All checks passed
        return true;
    }

    #[test]
    fn basic_cases() {
        let values = vec![6, 6, 5, 1, 2, 3, 9];
        let sorted_values = ICantBelieveItCanSort(values);
        assert!(check_non_decreasing_sorted(sorted_values));

        let values = vec![3, 1, 4, 1, 5, 9, 2];
        let sorted_values = ICantBelieveItCanSort(values);
        assert!(check_non_decreasing_sorted(sorted_values));


        let values = vec![3, 3, 3, 3, 3];
        let sorted_values = ICantBelieveItCanSort(values);
        assert!(check_non_decreasing_sorted(sorted_values));

        let values = vec![1, 2, 3, 4, 5, 6];
        let sorted_values = ICantBelieveItCanSort(values);
        assert!(check_non_decreasing_sorted(sorted_values));


        let values = vec![9, 5, 3, 2, 1];
        let sorted_values = ICantBelieveItCanSort(values);
        assert!(check_non_decreasing_sorted(sorted_values));

        let values = vec![-1, -10, 2, -14, 15];
        let sorted_values = ICantBelieveItCanSort(values);
        assert!(check_non_decreasing_sorted(sorted_values));
    }
}

