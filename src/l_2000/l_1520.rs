/// 1. find the min and max occurrence of index
pub fn max_num_of_substrings(s: String) -> Vec<String> {
    let s = s.as_bytes();
    #[derive(Clone, Copy)]
    struct Seg {
        l: i32,
        r: i32,
    }

    let mut segs = vec![Seg { l: -1, r: -1 }; 26];

    for (i, c) in s.iter().enumerate() {
        let idx = (c - b'a') as usize;
        // set start point of given char
        if segs[idx].l == -1 {
            segs[idx].l = i as i32;
        }
        segs[idx].r = i as i32;
    }

    for i in 0..26 {
        // left index still -1 -> no such char
        if segs[i].l == -1 {
            continue;
        }

        let mut j = segs[i].l;
        while j <= segs[i].r {
            let idx = (s[j as usize] - b'a') as usize;
            if segs[idx].l >= segs[i].l && segs[idx].r <= segs[i].r {
            } else {
                segs[i].l = segs[idx].l.min(segs[i].l);
                segs[i].r = segs[idx].r.max(segs[i].r);
                j = segs[i].l;
            }
            j += 1;
        }
    }

    segs.sort_by(|a, b| {
        if a.r == b.r {
            b.l.cmp(&a.l)
        } else {
            a.r.cmp(&b.r)
        }
    });

    let mut ans = vec![];
    let mut end = -1;

    for seg in segs {
        if seg.l == -1 {
            continue;
        }

        if seg.l > end {
            ans.push(String::from_utf8(s[seg.l as usize..=seg.r as usize].to_vec()).unwrap());
            end = seg.r;
        }
    }

    ans
}
