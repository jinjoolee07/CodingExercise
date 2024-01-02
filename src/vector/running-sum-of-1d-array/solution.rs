impl Solution {
    pub fn running_sum(nums: Vec<i32>) -> Vec<i32> {
        //create new vector
        //making it same length as nums
        let mut running_sum = vec![0; nums.len()];

        //iterate over each index i
        for i in 0..nums.len() {
            running_sum[i] = if i == 0 {
                nums[i]
            } else {
                running_sum[i - 1] + nums[i]
            };
        }

        running_sum
    }
}
