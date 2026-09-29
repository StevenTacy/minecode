/// 1. collect each group that differences within the limit
/// 2. cuz we can only do valid swap for element within same group
///    we just sorted the order of the group member in original array order
/// 3. get smallest out of it
pub fn lexicographically_smallest_array(mut nums: Vec<i32>, limit: i32) -> Vec<i32> {
    use std::collections::{HashMap, VecDeque};
    let mut sorted_nums = nums.clone();
    sorted_nums.sort();

    let mut cur_group = 0i32;
    // map to mapping each number to its group
    let mut num_to_group = HashMap::<i32, i32>::with_capacity(nums.len());
    num_to_group.insert(sorted_nums[0], cur_group);
    //store each group mapping with group number
    let mut group_to_list = HashMap::<i32, VecDeque<i32>>::with_capacity(nums.len()); // record current group number
    group_to_list.insert(cur_group, VecDeque::from(vec![sorted_nums[0]]));

    for i in 1..nums.len() {
        if (sorted_nums[i] - sorted_nums[i - 1]).abs() > limit {
            cur_group += 1;
        }

        num_to_group.insert(sorted_nums[i], cur_group);
        group_to_list
            .entry(cur_group)
            .or_insert(VecDeque::new())
            .push_back(sorted_nums[i]);
    }

    for i in 0..nums.len() {
        let group = num_to_group.get(&nums[i]).unwrap();
        let group_list = group_to_list.get_mut(group).unwrap();
        nums[i] = group_list.pop_front().unwrap();
    }

    nums
}

// def lexicographicallySmallestArray(self, nums, limit):
//     nums_sorted = sorted(nums)
//
//     curr_group = 0
//     num_to_group = {}
//     num_to_group[nums_sorted[0]] = curr_group
//
//     group_to_list = {}
//     group_to_list[curr_group] = deque([nums_sorted[0]])
//
//     for i in range(1, len(nums)):
//         if abs(nums_sorted[i] - nums_sorted[i - 1]) > limit:
//             # new group
//             curr_group += 1
//
//         # assign current element to group
//         num_to_group[nums_sorted[i]] = curr_group
//
//         # add element to sorted group deque
//         if curr_group not in group_to_list:
//             group_to_list[curr_group] = deque()
//         group_to_list[curr_group].append(nums_sorted[i])
//
//     # iterate through input and overwrite each element with the next element in its corresponding group
//     for i in range(len(nums)):
//         num = nums[i]
//         group = num_to_group[num]
//         nums[i] = group_to_list[group].popleft()
//
//     return nums
