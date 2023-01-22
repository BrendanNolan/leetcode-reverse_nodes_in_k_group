#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

fn reverse_at(slice: &mut [i32], index: usize) {
    let left_index = index;
    let right_index = slice.len() - 1 - index;
    let left = slice[left_index];
    let right = slice[right_index];
    slice[left_index] = right;
    slice[right_index] = left;
}

fn reverse(slice: &mut [i32]) {
    let index_count = slice.len() / 2;
    for index in 0..index_count {
        reverse_at(slice, index);
    }
}

fn convert_to_vec(mut list: &ListNode) -> Vec<i32> {
    let mut vec = Vec::new();
    loop {
        vec.push(list.val);
        match list.next {
            Some(ref next_node) => list = next_node,
            None => break,
        }
    }
    vec
}

fn convert_to_list(slice: &[i32]) -> Option<Box<ListNode>> {
    let mut head = None;
    for val in slice.iter().rev() {
        head = Some(Box::new(ListNode {
            val: *val,
            next: head.take(),
        }));
    }
    head
}

fn reverse_groups(slice: &mut [i32], group_size: usize) {
    let group_count = slice.len() / group_size;
    for group_counter in 0..group_count {
        let start_point = group_counter * group_size;
        let current_slice = &mut slice[start_point..start_point + group_size];
        reverse(current_slice);
    }
}

fn reverse_list_groups(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
    match head {
        Some(head) => {
            let mut list_vec = convert_to_vec(&head);
            reverse_groups(&mut list_vec, k as usize);
            convert_to_list(&list_vec)
        }
        None => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reverse() {
        let mut vec = vec![1, 2, 3, 4, 5];
        reverse(&mut vec);
        assert_eq!(vec, vec![5, 4, 3, 2, 1]);
    }

    #[test]
    fn test_conversions() {
        let vec = vec![1, 2, 3, 4, 5];
        let list = convert_to_list(&vec);
        match list {
            Some(node) => {
                let round_trip_vec = convert_to_vec(&node);
                assert_eq!(vec, round_trip_vec);
            }
            None => panic!(),
        }
    }

    #[test]
    fn test_reverse_groups() {
        let mut vec = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11];
        reverse_groups(&mut vec, 3);
        assert_eq!(vec, vec![3, 2, 1, 6, 5, 4, 9, 8, 7, 10, 11]);
    }
}

fn main() {
    let mut vec = vec![1, 2, 3, 4, 5];
    reverse_groups(&mut vec, 3);
    assert_eq!(vec, vec![3, 2, 1, 4, 5])
}
