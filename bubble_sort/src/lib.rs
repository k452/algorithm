pub fn bubble_sort(list: &mut Vec<isize>) -> &Vec<isize> {
    let mut count = 0;
    let mut flag = true;

    while flag {
        flag = false;

        for j in ((count + 1)..=(list.len() - 1)).rev() {
            // println!("j={} list={:?} flag={}", &j, &list, &flag);

            if list[j] < list[j - 1] {
                list.swap(j, j - 1);

                // swapが発生したということはまだソートの余地があるということでtrue
                // ソートが完了した場合はここに到達しないのでwhileが止まる
                flag = true;
            }
        }

        count += 1;
    }

    list
}
