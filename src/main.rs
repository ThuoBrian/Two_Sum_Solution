use std::collections::HashMap;

fn main() {
    let array = [4, 5, 5, 15];
    let target = 9;

    match find_index_two_sum(&array, target) {
        Some((index1, index2)) => {
            println!("Because nums[{}] + nums[{}] == {} ", index1, index2, target)
        }
        None => println!("the index of the sum target wasn't found"),
    }
}

fn find_index_two_sum(array: &[i32], target: i32) -> Option<(usize, usize)> {
    let mut map = HashMap::new();

    for (i, &value1) in array.iter().enumerate() {
        let remainder = target - value1;

        if let Some(&j) = map.get(&remainder) {
            return Some((j, i));
        }
        map.insert(value1, i);
    }
    None
}
