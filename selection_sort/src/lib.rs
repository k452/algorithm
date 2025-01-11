pub fn selection_sort(list: &mut Vec<isize>) -> &Vec<isize> {
    for i in 0..(list.len() - 1) {
        let mut min_idx = i;

        for j in i..(list.len() - 1) {
            if list[j] < list[min_idx] {
                min_idx = j;
            }
        }

        list.swap(i, min_idx);
    }

    list
}
