
/// Struct where we implement the sorting algorithms, but in different ways
/// IE. inplace sort vs creating a clone and sorting that clone, without mutating given data
struct ICantBelieveItCanSort;


impl ICantBelieveItCanSort{

    /// Sort a given vector of values, without mutating the original vector of values
    /// Thus, a clone is created, sorted and returned
    pub fn sorted<T>(values: Vec<T>) -> Vec<T>
    where T: Ord + Clone{
        // Start cloning the values into a mutable vector
        let mut new_values = values.clone();

        // Use the sorting algo
        for i in 1..new_values.len(){
            for j in 1..new_values.len(){
                if new_values[i] < new_values[j]{
                    new_values.swap(i, j);
                }
            }
        }

        return new_values;
    }

    /// Sort a given vector in-place, mutating it
    pub fn sort<T>(values: &mut Vec<T>)
    where T: Ord{
        // Use the sorting algo
        for i in 1..values.len(){
            for j in 1..values.len(){
                if values[i] < values[j]{
                    values.swap(i, j);
                }
            }
        }
    }

}



#[cfg(test)]
mod tests {

    use super::ICantBelieveItCanSort;

    /// For checking that a vec is sorted in non-decreasing order
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
    fn basic_cases_sorted() {

        let values = vec![6, 6, 5, 1, 2, 3, 9];
        let sorted_values = ICantBelieveItCanSort::sorted(values);
        assert!(check_non_decreasing_sorted(sorted_values));

        let values = vec![3, 1, 4, 1, 5, 9, 2];
        let sorted_values = ICantBelieveItCanSort::sorted(values);
        assert!(check_non_decreasing_sorted(sorted_values));


        let values = vec![3, 3, 3, 3, 3];
        let sorted_values = ICantBelieveItCanSort::sorted(values);
        assert!(check_non_decreasing_sorted(sorted_values));

        let values = vec![1, 2, 3, 4, 5, 6];
        let sorted_values = ICantBelieveItCanSort::sorted(values);
        assert!(check_non_decreasing_sorted(sorted_values));

        let values = vec![9, 5, 3, 2, 1];
        let sorted_values = ICantBelieveItCanSort::sorted(values);
        assert!(check_non_decreasing_sorted(sorted_values));

        let values = vec![-1, -10, 2, -14, 15];
        let sorted_values = ICantBelieveItCanSort::sorted(values);
        assert!(check_non_decreasing_sorted(sorted_values));
    }

    #[test]
    fn basic_cases_sort(){

        let mut values = vec![6, 6, 5, 1, 2, 3, 9];
        ICantBelieveItCanSort::sort(&mut values);
        assert!(check_non_decreasing_sorted(values));

        let mut values = vec![3, 1, 4, 1, 5, 9, 2];
        ICantBelieveItCanSort::sort(&mut values);
        assert!(check_non_decreasing_sorted(values));

        let mut values = vec![3, 3, 3, 3, 3];
        ICantBelieveItCanSort::sort(&mut values);
        assert!(check_non_decreasing_sorted(values));

        let mut values = vec![1, 2, 3, 4, 5, 6];
        ICantBelieveItCanSort::sort(&mut values);
        assert!(check_non_decreasing_sorted(values));


        let mut values = vec![9, 5, 3, 2, 1];
        ICantBelieveItCanSort::sort(&mut values);
        assert!(check_non_decreasing_sorted(values));

        let mut values = vec![-1, -10, 2, -14, 15];
        ICantBelieveItCanSort::sort(&mut values);
        assert!(check_non_decreasing_sorted(values));

    }
}

