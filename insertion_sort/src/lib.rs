pub fn insertion_sort(list: &mut Vec<isize>) -> &Vec<isize> {
    for i in 1..list.len() {
        // インデックスをusizeで扱う
        let v = list[i]; // 現在の値を保持
        let mut j = i as isize - 1; // 比較のために一つ前のインデックスをisizeで初期化

        while j >= 0 && list[j as usize] > v {
            // ソート済み部分を後ろにシフト
            list[(j + 1) as usize] = list[j as usize];
            j -= 1;
        }

        list[(j + 1) as usize] = v; // 挿入位置に値をセット
    }

    list
}
