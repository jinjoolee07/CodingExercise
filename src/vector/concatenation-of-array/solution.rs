impl Solution {
    pub fn get_concatenation(nums: Vec<i32>) -> Vec<i32> {
        //create a new array to store results
        let mut result = Vec::new();

        for num in nums {
            result.push(num);
            result.push(num);
        }
        result
    }
}
