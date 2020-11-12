// https://leetcode.com/problems/rank-teams-by-votes/

pub fn rank_teams(votes: Vec<String>) -> String {
    let number_of_votes = votes.get(0).unwrap().len();
    let mut ranks = vec![vec![0; number_of_votes]; 26];
    let votes = &votes;
    for vote in votes {
        vote.char_indices().for_each(|(i, c)| {
            *ranks
                .get_mut((c as u32 - 'A' as u32) as usize)
                .unwrap()
                .get_mut(i)
                .unwrap() += 1;
        });
    }
    let mut ranks = ranks
        .iter()
        .enumerate()
        .collect::<Vec<(usize, &Vec<i32>)>>();
    ranks.sort_by(|(first_idx, first), (second_idx, second)| {
        let len = first.len();
        let mut is_first_greater: Option<bool> = None;
        for i in 0..len {
            if first.get(i).unwrap() == second.get(i).unwrap() {
                continue;
            }
            if first.get(i).unwrap() > second.get(i).unwrap() {
                is_first_greater = Some(true);
            } else {
                is_first_greater = Some(false);
            }
            break;
        }
        if is_first_greater.is_none() {
            if first_idx > second_idx {
                is_first_greater = Some(false);
            } else {
                is_first_greater = Some(true);
            }
        }
        match is_first_greater {
            None => panic!("Not possible"),
            Some(is_first_greater) => {
                if is_first_greater {
                    std::cmp::Ordering::Less
                } else {
                    std::cmp::Ordering::Greater
                }
            }
        }
    });
    let mut res = String::new();
    for i in 0..number_of_votes {
        let (c, _) = ranks.get(i).unwrap();
        res.push((*c as u8 + b'A') as char);
    }
    res
}

#[cfg(test)]
mod tests {
    use super::rank_teams;
    #[test]
    fn rank_team_by_votes_basic() {
        assert_eq!(
            rank_teams(vec![
                String::from("ABC"),
                String::from("ACB"),
                String::from("ABC"),
                String::from("ACB"),
                String::from("ACB"),
            ]),
            String::from("ACB"),
        );
        assert_eq!(
            rank_teams(vec![String::from("WXYZ"), String::from("XYZW")]),
            String::from("XWYZ"),
        );
        assert_eq!(
            rank_teams(vec![String::from("ZMNAGUEDSJYLBOPHRQICWFXTVK")]),
            String::from("ZMNAGUEDSJYLBOPHRQICWFXTVK"),
        );
        assert_eq!(
            rank_teams(vec![
                String::from("BCA"),
                String::from("CAB"),
                String::from("CBA"),
                String::from("ABC"),
                String::from("ACB"),
                String::from("BAC")
            ]),
            String::from("ABC"),
        );
        assert_eq!(
            rank_teams(vec![
                String::from("M"),
                String::from("M"),
                String::from("M"),
                String::from("M")
            ]),
            String::from("M"),
        );
    }
}
