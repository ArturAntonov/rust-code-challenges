fn median(mut nums: Vec<f32>) -> Option<f32> {
    if nums.is_empty() {
        return None;
    }

    nums.sort_by(|x, y| x.partial_cmp(y).unwrap());

    let middle = nums.len() / 2;

    let med = if nums.len() % 2 == 0 {
        (nums[middle - 1] + nums[middle]) / 2.0
    } else {
        let middle_idx = (nums.len() as f32 / 2.0).floor();
        nums[middle_idx as usize]
    };

    Some(med)
}

fn main() {
    let answer = median(vec![1.0, 4.0, 5.0]).unwrap();

    println!("median([1, 4, 5]) is {:?}", answer);
}

#[test]
fn empty_list() {
    let list_three = vec![];
    assert_eq!(median(list_three), None);
}

#[test]
fn even_elements_list() {
    let list_two = vec![1.0, 3.0, 4.0, 6.0];
    assert_eq!(median(list_two), Some(3.5));
}

#[test]
fn odd_elements_list() {
    let list_one = vec![1.0, 4.0, 5.0];
    assert_eq!(median(list_one), Some(4.0));
}
