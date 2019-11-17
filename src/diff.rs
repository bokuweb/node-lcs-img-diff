use std::cmp;

#[derive(Debug, PartialEq)]
pub enum DiffResult {
    Removed(DiffElement),
    Common(DiffElement),
    Added(DiffElement),
}

#[derive(Debug, PartialEq)]
pub struct DiffElement {
    pub old_index: Option<usize>,
    pub new_index: Option<usize>,
}

fn create_table<T: PartialEq>(old: &[T], new: &[T]) -> Vec<Vec<u32>> {
    let new_len = new.len();
    let old_len = old.len();
    let mut table = vec![vec![0; old_len + 1]; new_len + 1];
    for i in 0..new_len {
        let i = new_len - i - 1;
        table[i][old_len] = 0;
        for j in 0..old_len {
            let j = old_len - j - 1;
            table[i][j] = if new[i] == old[j] {
                table[i + 1][j + 1] + 1
            } else {
                cmp::max(table[i + 1][j], table[i][j + 1])
            }
        }
    }
    table
}

pub fn diff<T: PartialEq>(old: &Vec<T>, new: &Vec<T>) -> Vec<DiffResult> {
    let mut result: Vec<DiffResult> = Vec::new();
    let new_len = new.len();
    let old_len = old.len();

    if new_len == 0 {
        let mut o = 0;
        while o < old_len {
            result.push(DiffResult::Removed(DiffElement {
                old_index: Some(o),
                new_index: None,
            }));
            o += 1;
        }
        return result;
    } else if old_len == 0 {
        let mut n = 0;
        while n < new_len {
            result.push(DiffResult::Added(DiffElement {
                old_index: None,
                new_index: Some(n),
            }));
            n += 1;
        }
        return result;
    } else {
        let mut o = 0;
        let mut n = 0;
        let common_prefix = old.iter().zip(new).take_while(|p| p.0 == p.1);
        let prefix_size = common_prefix.count();
        let common_suffix = old
            .iter()
            .rev()
            .zip(new.iter().rev())
            .take(cmp::min(old_len, new_len) - prefix_size)
            .take_while(|p| p.0 == p.1);
        let suffix_size = common_suffix.count();
        let table = create_table(
            &old[prefix_size..(old_len - suffix_size)],
            &new[prefix_size..(new_len - suffix_size)],
        );
        let new_len = new_len - prefix_size - suffix_size;
        let old_len = old_len - prefix_size - suffix_size;

        // Restore common prefix
        let mut prefix_index = 0;
        while prefix_index < prefix_size {
            result.push(DiffResult::Common(DiffElement {
                old_index: Some(prefix_index),
                new_index: Some(prefix_index),
            }));
            prefix_index += 1;
        }

        loop {
            if n >= new_len || o >= old_len {
                break;
            }
            let new_index = n + prefix_size;
            let old_index = o + prefix_size;
            if new[new_index] == old[old_index] {
                result.push(DiffResult::Common(DiffElement {
                    old_index: Some(old_index),
                    new_index: Some(new_index),
                }));
                n += 1;
                o += 1;
            } else if table[n + 1][o] >= table[n][o + 1] {
                result.push(DiffResult::Added(DiffElement {
                    old_index: None,
                    new_index: Some(new_index),
                }));
                n += 1;
            } else {
                result.push(DiffResult::Removed(DiffElement {
                    old_index: Some(old_index),
                    new_index: None,
                }));
                o += 1;
            }
        }
        while n < new_len {
            let new_index = n + prefix_size;
            result.push(DiffResult::Added(DiffElement {
                old_index: None,
                new_index: Some(new_index),
            }));
            n += 1;
        }
        while o < old_len {
            let old_index = o + prefix_size;
            result.push(DiffResult::Removed(DiffElement {
                old_index: Some(old_index),
                new_index: None,
            }));
            o += 1;
        }

        // Restore common suffix
        let mut suffix_index = 0;
        while suffix_index < suffix_size {
            let old_index = suffix_index + old_len + prefix_size;
            let new_index = suffix_index + new_len + prefix_size;
            result.push(DiffResult::Common(DiffElement {
                old_index: Some(old_index),
                new_index: Some(new_index),
            }));
            suffix_index += 1;
        }
    }
    result
}
