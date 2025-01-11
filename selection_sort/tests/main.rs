#[cfg(test)]
mod tests {
    #[test]
    fn 正常系_重複のない自然数のみ() {
        assert_eq!(
            selection_sort::selection_sort(&mut vec![1, 3, 2, 100]),
            &vec![1, 2, 3, 100]
        );
    }

    #[test]
    fn 正常系_重複のある自然数のみ() {
        assert_eq!(
            selection_sort::selection_sort(&mut vec![1, 3, 2, 3]),
            &vec![1, 2, 3, 3]
        );
    }

    #[test]
    fn 正常系_重複がなく0と負の数を含む() {
        assert_eq!(
            selection_sort::selection_sort(&mut vec![1, -13, 0, 3]),
            &vec![-13, 0, 1, 3]
        );
    }
}
