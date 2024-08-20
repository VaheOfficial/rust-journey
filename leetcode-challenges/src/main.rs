mod global;
mod solutions;

fn main() {
    println!("LeetCode challenges");
    loop {
       println!("Challenges!");
       println!("1. Two Sum");
        
       let choice = global::input_acceptor::read_input().trim().to_string();
        
       match choice.as_str() {
           "1" => {
               let nums = [2,7,11,15].to_vec();
               let target = 9;
               let expected_output = [0,1];
               let v = solutions::two_sum_solution::two_sum(nums.clone(), target);
                if v == expected_output {
                    println!("Worked, results {:?}", v);
                } else {
                    println!("Failed, results {:?}", v);
                }
                println!("That was the first variation which resulted in a time complexity of O(n^2) but the new soltions runs at O(n) so its a lot quicker
First solution takes around 55ms while second one takes 0ms");
                let x = solutions::two_sum_solution::two_sum_second_solution(nums, target);
                if v == expected_output {
                    println!("Worked, results: {:?}", x);
                } else {
                    println!("Didnt work, results: {:?}", x);
                }
           }
            _ => println!("Invalid choice, please select again."),
       }
    }
}
