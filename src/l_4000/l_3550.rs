const MAX_VAL: usize = 1000;
const fn precomp() -> [i32; MAX_VAL + 1] {
    let mut arr = [0; MAX_VAL + 1];
    let mut i = 0;

    while i <= MAX_VAL {
        let mut sum = 0;
        let mut n = i as i32;

        while n > 0 {
            sum += n % 10;
            n /= 10;
        }

        arr[i] = sum;
        i += 1;
    }

    arr
}

static PRECOMP: [i32; MAX_VAL + 1] = precomp();

pub fn smallest_index(nums: Vec<i32>) -> i32 {
    for (i, &n) in nums.iter().enumerate() {
        let mut sum = 0;
        let mut n = n;
        while n > 0 {
            sum += n % 10;
            n /= 10;
        }

        if sum == i as i32 {
            return i as i32;
        }
    }

    -1
}

pub fn pre_smallest_index(nums: Vec<i32>) -> i32 {
    for (i, &n) in nums.iter().enumerate() {
        if PRECOMP[n as usize] == i as i32 {
            return i as i32;
        }
    }

    -1
}
