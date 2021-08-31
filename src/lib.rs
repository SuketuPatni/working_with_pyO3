use std::collections::HashMap;

fn mode(numbers: &Vec<i64>) -> i64 {
    let mut occurrences = HashMap::new();

    for &value in numbers {
        *occurrences.entry(value).or_insert(0) += 1;
    }

    occurrences
        .into_iter()
        .max_by_key(|&(_, count)| count)
        .map(|(val, _)| val)
        .expect("Cannot compute the mode of zero numbers")
}

fn main() {
    use pyo3::prelude::*;

    #[pyfunction]
    fn solve(row_1: Vec<i64>, row_2: Vec<i64>, row_3: Vec<i64>) -> PyResult<i64> {
        let row_01 = row_1.to_vec();
        let row_02 = row_2.to_vec();
        let row_03 = row_3.to_vec();

        let mut num_aps: i64 = 0;
        if row_02[0] - row_01[0] == row_03[0] - row_02[0] {
            num_aps += 1;
        }
        if row_01[1] - row_01[0] == row_01[2] - row_01[1] {
            num_aps += 1;
        }
        if row_02[1] - row_01[2] == row_03[2] - row_02[1] {
            num_aps += 1;
        }
        if row_03[1] - row_03[0] == row_03[2] - row_03[1] {
            num_aps += 1;
        }

        let mut sum_vec: Vec<i64> = vec![];
        if (row_01[0] + row_03[2]) % 2 == 0 {
            sum_vec.push(row_01[0] + row_03[2]);
        }
        if (row_01[1] + row_03[1]) % 2 == 0 {
            sum_vec.push(row_01[1] + row_03[1]);
        }
        if (row_01[2] + row_03[0]) % 2 == 0 {
            sum_vec.push(row_01[2] + row_03[0]);
        }
        if (row_02[0] + row_02[1]) % 2 == 0 {
            sum_vec.push(row_02[0] + row_02[1]);
        }
        // let sum_vec_2 = setify(&sum_vec);
        // num_aps += (sum_vec.len() - sum_vec_2.len() + 1) as i64;
        if sum_vec.len() == 0 {
            num_aps += 0;
        } else if sum_vec.len() == 1 {
            num_aps += 1;
        } else if sum_vec.len() == 2 {
            if sum_vec[0] == sum_vec[1] {
                num_aps += 2;
            } else {
                num_aps += 1;
            }
        } else {
            let mode = mode(&sum_vec);
            for i in 0..sum_vec.len() {
                if sum_vec[i] == mode {
                    num_aps += 1;
                }
            }
        }

        Ok(num_aps)
    }

    #[pymodule]
    fn ks_d1_2021(_py: Python, m: &PyModule) -> PyResult<()> {
        m.add_function(wrap_pyfunction!(solve, m)?)?;
        Ok(())
    }
}
