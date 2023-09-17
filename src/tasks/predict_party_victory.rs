use std::collections::VecDeque;

pub fn predict_party_victory(senate: String) -> String {
    let mut q = VecDeque::with_capacity(senate.len());
    for &c in senate.as_bytes() {
        q.push_back(c)
    }
    q.push_back(b'C');
    let mut r_points = 0;
    let mut remain_r = 0;
    let mut d_points = 0;
    let mut remain_d = 0;
    let winner = loop {
        let c = q.pop_front()
            .unwrap();
        if c == b'R' {
            if d_points > 0 {
                d_points -= 1;
            } else {
                q.push_back(c);
                r_points += 1;
                remain_r += 1;
            }
        } else if c == b'D' {
            if r_points > 0 {
                r_points -= 1;
            } else {
                q.push_back(c);
                d_points += 1;
                remain_d += 1;
            }
        } else {
            if q.len() == 1 || remain_r == 0 || remain_d == 0 {
                break q.pop_front().unwrap();
            }
            q.push_back(c);
            remain_r = 0;
            remain_d = 0;
        }
    };
    if winner == b'R' {
        "Radiant".to_string()
    } else {
        "Dire".to_string()
    }
}
