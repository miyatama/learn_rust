use super::heap_sort;
use std::io::Write;
use std::cmp::Ordering;

pub fn search<W: Write>(w: &mut W, n: u64, list: Vec<u64>, value: u64) {
    let result = binary_search_tree(list, value);
    match result {
        None => writeln!(w, "Not Found").unwrap(),
        Some(_) => writeln!(w, "Found").unwrap(),
    }
}

fn binary_search_tree(list: Vec<u64>, value: u64) -> Option<()> {
    let bst = create_tree(list);
    eprintln!("value: {}, bst: {:?}", value, bst);
    search_from_binary_search_tree(bst, value, 0)
}

fn search_from_binary_search_tree(bst: Vec<Option<u64>>, value: u64, index: usize) -> Option<()> {
    eprintln!("search index: {}", index);
    if bst.len() <= index {
        return None;
    }
    if bst[index] == None {
        return None;
    }
    let bst_value = bst[index].unwrap();
    eprintln!("value: {}, bst value: {}", value, bst_value);
    let (left_index, right_index) = get_node_indixes(index);
    let next_index = match value.cmp(&bst_value) {
        Ordering::Less => left_index,
        Ordering::Greater => right_index,
        Ordering::Equal => index,
    };
    if index == next_index {
        return Some(());
    }
    if next_index >= bst.len() {
        return None;
    }
    search_from_binary_search_tree(bst, value, next_index)
}

fn get_node_indixes(index: usize) -> (usize, usize) {
    if index == 0 {
        return (1, 2);
    }
    (index * 2, index * 2 + 1)
}

fn create_tree(list: Vec<u64>) -> Vec<Option<u64>> {
    let sorted_list = heap_sort::sort(list.clone());
    let max_depth = get_max_depth_index(sorted_list.clone(), 0u64);
    let dst_list = Vec::<Option<u64>>::new();
    create_bst(sorted_list, dst_list, 0, max_depth)
}

fn create_bst(
    list: Vec<u64>,
    dst_list: Vec<Option<u64>>,
    depth: u64,
    max_depth: u64,
) -> Vec<Option<u64>> {
    if depth >= max_depth {
        return dst_list;
    }

    if dst_list.len() == 0 {
        let index = list.len() / 2;
        return create_bst(list.clone(), vec![Some(list[index])], depth + 1, max_depth);
    }

    let mut dst_list = dst_list.clone();
    let child_nodes = get_child_nodes(list.clone(), depth);
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
    fn test_create_tree_empty() {
        let param: Vec<u64> = vec![];
        let actual = create_tree(param);
        let expect: Vec<Option<u64>> = vec![]
            .into_iter()
            .map(|val| Some(val))
            .collect::<Vec<Option<u64>>>();
        assert_eq!(actual.len(), expect.len());
        for i in 0..expect.len() {
            assert_eq!(actual[i], expect[i]);
        }
    }

    #[test]
    fn test_create_tree_one() {
        let param: Vec<u64> = vec![100];
        let actual = create_tree(param);
        let expect: Vec<Option<u64>> = vec![100]
            .into_iter()
            .map(|val| Some(val))
            .collect::<Vec<Option<u64>>>();
        assert_eq!(actual.len(), expect.len());
        for i in 0..expect.len() {
            assert_eq!(actual[i], expect[i]);
        }
    }

    #[test]
    fn test_create_tree_two() {
        let param: Vec<u64> = vec![100, 101];
        let actual = create_tree(param);
        let expect: Vec<Option<u64>> = vec![101,100]
            .into_iter()
            .map(|val| Some(val))
            .collect::<Vec<Option<u64>>>();
        assert_eq!(actual.len(), expect.len());
        for i in 0..expect.len() {
            assert_eq!(actual[i], expect[i]);
        }
    }

    #[test]
    fn test_create_tree_three() {
        let param: Vec<u64> = vec![103, 102, 101];
        let actual = create_tree(param);
        let expect: Vec<Option<u64>> = vec![102, 101, 103]
            .into_iter()
            .map(|val| Some(val))
            .collect::<Vec<Option<u64>>>();
        assert_eq!(actual.len(), expect.len());
        for i in 0..expect.len() {
            assert_eq!(actual[i], expect[i]);
        }
    }

    #[test]
    fn test_create_tree_duplicate() {
        let param: Vec<u64> = vec![ 100, 100, 100, 100, 102, 101];
        let actual = create_tree(param);
        let expect: Vec<Option<u64>> = vec![101, 100, 102]
            .into_iter()
            .map(|val| Some(val))
            .collect::<Vec<Option<u64>>>();
        assert_eq!(actual.len(), expect.len());
        for i in 0..expect.len() {
            assert_eq!(actual[i], expect[i]);
        }
    }


    #[test]
    fn test_create_tree_five_01() {
        let param: Vec<u64> = vec![ 100, 101, 102, 103, 104];
        let actual = create_tree(param);
        let expect: Vec<Option<u64>> = vec![Some(102), Some(101), Some(104), Some(100), None, Some(10), None];
        assert_eq!(actual.len(), expect.len());
        for i in 0..expect.len() {
            assert_eq!(actual[i], expect[i]);
        }
    }

    #[test]
    fn test_create_tree_five_02() {
        let param: Vec<u64> = vec![1, 2, 3, 4, 5];
        let actual = create_tree(param);
        let expect: Vec<Option<u64>> =
            vec![Some(3), Some(2), Some(5), Some(1), None, Some(4), None];
        for i in 0..expect.len() {
            assert_eq!(actual[i], expect[i]);
        }
    }


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
    fn test_search01() {
        let mut buff = Vec::<u8>::new();
        search(&mut buff, 10, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 1);
        let actual = String::from_utf8(buff).unwrap();
        let actual = actual.split("\n").collect::<Vec<&str>>();
        let expect = vec!["Found"];
        (0..expect.len()).for_each(|index| {
            assert_eq!(expect[index], actual[index]);
        });
    }

    #[test]
    fn test_search02() {
        let mut buff = Vec::<u8>::new();
        search(&mut buff, 10, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 10);
        let actual = String::from_utf8(buff).unwrap();
        let actual = actual.split("\n").collect::<Vec<&str>>();
        let expect = vec!["Found"];
        (0..expect.len()).for_each(|index| {
            assert_eq!(expect[index], actual[index]);
        });
    }

    #[test]
    fn test_search03() {
        let mut buff = Vec::<u8>::new();
        search(&mut buff, 10, vec![1, 3, 4, 5, 6, 7, 8, 9, 10, 11], 2);
        let actual = String::from_utf8(buff).unwrap();
        let actual = actual.split("\n").collect::<Vec<&str>>();
        let expect = vec!["Not Found"];
        (0..expect.len()).for_each(|index| {
            assert_eq!(expect[index], actual[index]);
        });
    }

    #[test]
    fn test_search04() {
        let mut buff = Vec::<u8>::new();
        search(&mut buff, 3, vec![1, 2, 3], 3);
        let actual = String::from_utf8(buff).unwrap();
        let actual = actual.split("\n").collect::<Vec<&str>>();
        let expect = vec!["Found"];
        (0..expect.len()).for_each(|index| {
            assert_eq!(expect[index], actual[index]);
        });
    }


    #[test]
    fn test_search05() {
        let mut buff = Vec::<u8>::new();
        search(
            &mut buff,
            10,
            vec![
                0, 1, 4, 5, 6, 8, 9, 10, 11, 14, 17, 18, 21, 22, 23, 24, 26, 27, 28, 29, 30, 32,
                34, 36, 37, 40, 41, 43, 45, 46, 47, 48, 49, 51, 53, 54, 55, 56, 57, 58, 59, 60, 62,
                63, 65, 70, 72, 73, 74, 75, 76, 77, 78, 79, 80, 81, 85, 86, 87, 88, 90, 93, 97, 98,
                99,
            ],
            99,
        );

        let actual = String::from_utf8(buff).unwrap();
        let actual = actual.split("\n").collect::<Vec<&str>>();
        let expect = vec!["Found"];
        (0..expect.len()).for_each(|index| {
            assert_eq!(expect[index], actual[index]);
        });
    }

    #[test]
    fn test_search06() {
        let mut buff = Vec::<u8>::new();
        search(
            &mut buff,
            10,
            vec![
                4, 77, 88, 1, 90, 93, 49, 4, 41, 77, 75, 54, 54, 73, 30, 36, 27, 87, 70, 97, 99,
                72, 58, 80, 24, 65, 8, 57, 90, 37, 55, 63, 60, 99, 74, 34, 41, 10, 8, 24, 18, 21,
                45, 46, 85, 65, 79, 86, 87, 54, 21, 11, 29, 74, 78, 99, 56, 46, 6, 98, 9, 72, 57,
                28, 51, 47, 78, 41, 40, 81, 88, 59, 41, 41, 54, 0, 22, 17, 29, 49, 75, 36, 36, 1,
                53, 5, 98, 51, 26, 62, 97, 14, 43, 57, 1, 76, 32, 49, 48, 23,
            ],
            300,
        );
        let actual = String::from_utf8(buff).unwrap();
        let actual = actual.split("\n").collect::<Vec<&str>>();
        let expect = vec!["Not Found"];
        (0..expect.len()).for_each(|index| {
            assert_eq!(expect[index], actual[index]);
        });
    }
}
