use super::heap_sort;
use std::io::Write;

pub fn search<W: Write>(w: &mut W, n: u64, list: Vec<u64>, value: u64) {
    let result = binary_search_tree(list, value);
    match result {
        None => writeln!(w, "Not Found").unwrap(),
        Some(_) => writeln!(w, "Found").unwrap(),
    }
}

fn binary_search_tree(list: Vec<u64>, value: u64) -> Option<()> {
    let bst = create_tree(list);
    eprintln!("{:?}", bst);
    Some(())
}

fn create_tree(list: Vec<u64>) -> Vec<Option<u64>> {
    let sorted_list = heap_sort::sort(list.clone());
    let max_depth = get_max_depth_index(sorted_list.clone(), 0u64);
    eprintln!("max depth: {}", max_depth);
    let dst_list = Vec::<Option<u64>>::new();
    create_bst(sorted_list, dst_list, 0, max_depth)
}

fn create_bst(
    list: Vec<u64>,
    dst_list: Vec<Option<u64>>,
    depth: u64,
    max_depth: u64,
) -> Vec<Option<u64>> {
    eprintln!(
        "depth: {}, max depth: {}, dest list: {:?}",
        depth, max_depth, dst_list
    );
    if depth >= max_depth {
        return dst_list;
    }

    if dst_list.len() == 0 {
        let index = list.len() / 2;
        return create_bst(list.clone(), vec![Some(list[index])], depth + 1, max_depth);
    }

    let mut dst_list = dst_list.clone();
    let child_nodes = get_child_nodes(list.clone(), depth);
    eprintln!("depth: {}, child nodes: {:?}", depth, child_nodes);
    dst_list.extend(child_nodes);
    create_bst(list, dst_list, depth + 1, max_depth)
}

fn get_child_nodes(list: Vec<u64>, depth: u64) -> Vec<Option<u64>> {
    let lists = split_depth_list(list, depth);
    get_splited_lists_center_nodes(lists)
}

fn get_splited_lists_center_nodes(lists: Vec<Vec<Option<u64>>>) -> Vec<Option<u64>> {
    lists
        .iter()
        .map(|list| list[list.len() / 2])
        .collect::<Vec<Option<u64>>>()
}

fn split_depth_list(list: Vec<u64>, depth: u64) -> Vec<Vec<Option<u64>>> {
    if depth <= 0 {
        return vec![list
            .into_iter()
            .map(|val| Some(val))
            .collect::<Vec<Option<u64>>>()];
    }
    if list.len() == 1 {
        return vec![vec![None], vec![None]];
    }
    if list.len() == 2 {
        return vec![vec![Some(list[0])], vec![None]];
    }
    let center_index = list.len() / 2;
    let next_depth = depth - 1;
    let (left_start, left_end) = (0, center_index);
    let mut left_lists = split_depth_list(list[left_start..left_end].to_vec(), next_depth);
    let (right_start, right_end) = (center_index + 1, list.len());
    let right_lists = split_depth_list(list[right_start..right_end].to_vec(), next_depth);

    eprintln!("left:[{}..{}], right: [{}..{}]",left_start, left_end, right_start, right_end);
    

    left_lists.extend(right_lists);
    left_lists
}

fn get_max_depth_index(list: Vec<u64>, index: u64) -> u64 {
    if (list.len() as u64) < power(2u64, index) {
        index
    } else {
        get_max_depth_index(list, index + 1)
    }
}

fn power(v1: u64, v2: u64) -> u64 {
    if v2 <= 0 {
        return 1;
    }
    v1 * power(v1, v2 - 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search_01() {}

    #[test]
    fn test_create_tree_01() {
        let param: Vec<u64> = vec![3, 2, 1];
        let actual = create_tree(param);
        let expect: Vec<Option<u64>> = vec![2, 1, 3]
            .into_iter()
            .map(|val| Some(val))
            .collect::<Vec<Option<u64>>>();
        assert_eq!(actual.len(), expect.len());
        for i in 0..expect.len() {
            assert_eq!(actual[i], expect[i]);
        }
    }

    #[test]
    fn test_create_tree_02() {
        let param: Vec<u64> = vec![2];
        let actual = create_tree(param);
        let expect: Vec<Option<u64>> = vec![2]
            .into_iter()
            .map(|val| Some(val))
            .collect::<Vec<Option<u64>>>();
        assert_eq!(actual.len(), expect.len());
        for i in 0..expect.len() {
            assert_eq!(actual[i], expect[i]);
        }
    }

    #[test]
    fn test_create_tree_03() {
        let param: Vec<u64> = vec![2, 1];
        let actual = create_tree(param);
        let expect: Vec<Option<u64>> = vec![Some(2), Some(1), None];
        assert_eq!(actual.len(), expect.len());
        for i in 0..expect.len() {
            assert_eq!(actual[i], expect[i]);
        }
    }

    #[test]
    fn test_create_tree_five() {
        let param: Vec<u64> = vec![1,2,3,4,5];
        let actual = create_tree(param);
        let expect: Vec<Option<u64>> = vec![Some(3), Some(2), Some(5), Some(1), None, Some(4), None];
        for i in 0..expect.len() {
            assert_eq!(actual[i], expect[i]);
        }
    }
}
