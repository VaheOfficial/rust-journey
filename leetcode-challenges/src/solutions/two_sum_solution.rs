use std::collections::HashMap;

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
   for (num, &x) in nums.iter().enumerate() {
       for (secondary, &y) in nums.iter().enumerate() {
           if num != secondary {
                if x + y == target {
                    return vec![num as i32, secondary as i32];
                }
           }    
       }
   }
   return nums;
}

pub fn two_sum_second_solution(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut map: HashMap<i32, i32> = HashMap::new(); // To store value -> index

    for (index, &num) in nums.iter().enumerate() {
        let complement = target - num;
        if let Some(&complement_index) = map.get(&complement) {
            return vec![complement_index, index as i32];
        }
        map.insert(num, index as i32);
    }

    return vec![]; // Return an empty vector if no solution is found
}

