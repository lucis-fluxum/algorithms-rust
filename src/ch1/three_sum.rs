/// Counts the number of triples in an array that sum to zero.
pub fn count(nums: &[i32]) -> usize {
    let mut count = 0;

    for i in 0..nums.len() {
        for j in i + 1..nums.len() {
            for k in j + 1..nums.len() {
                if nums[i] + nums[j] + nums[k] == 0 {
                    count += 1;
                }
            }
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ch1::stopwatch::Stopwatch;
    use rand::{thread_rng, Rng};
    use statistical::univariate::average_deviation;

    #[test]
    fn basic_behavior() {
        assert_eq!(0, count(&vec![0, 1, 2]));
        assert_eq!(1, count(&vec![-1, -2, 3]));
        assert_eq!(3, count(&vec![-1, -2, 3, -4, 5, 6]));
    }

    #[test]
    #[ignore]
    // This test assumes the host machine is consistent within a small tolerance.
    fn matches_efficiency_model() {
        // model: T = a*N^3
        const START: usize = 125;
        const END: usize = 150;
        const TOLERANCE: f32 = 1.5; // for average absolute deviation

        // This holds our values for a
        let mut constants: Vec<f32> = Vec::with_capacity(END - START);

        for size in START..END {
            let mut rng = thread_rng();
            let mut nums: Vec<i32> = Vec::with_capacity(size);
            for _ in 0..size {
                nums.push(rng.gen_range(-1000, 1000));
            }
            let timer = Stopwatch::new();
            count(&nums);
            let duration = timer.elapsed_time().unwrap();
            // Divide by N^3 to get a
            let a = (duration / (size as u32).pow(3)).subsec_nanos() as f32;
            constants.push(a);
        }
        let avg_deviation = average_deviation(&constants, None);
        // println!("AvgDev: {}", avg_deviation);
        assert!(avg_deviation < TOLERANCE);
    }
}
