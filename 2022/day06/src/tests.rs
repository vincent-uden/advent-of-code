#[cfg(test)]

mod tests {
    use crate::{part_1, unique_chars, part_2};

    #[test]
    fn test_uniqe() {
        assert!(!unique_chars("aaaa"));
        assert!(!unique_chars("aabc"));
        assert!(unique_chars("abcd"));
    }

    #[test]
    fn test_part_1_ex() {
        assert_eq!(part_1("bvwbjplbgvbhsrlpgdmjqwftvncz"), 5);
        assert_eq!(part_1("nppdvjthqldpwncqszvftbrmjlhg"), 6);
        assert_eq!(part_1("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 10);
        assert_eq!(part_1("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 11);
    }

    #[test]
    fn test_part_2_ex() {
        assert_eq!(part_2("mjqjpqmgbljsphdztnvjfqwrcgsmlb")    , 19);
        assert_eq!(part_2("bvwbjplbgvbhsrlpgdmjqwftvncz")      , 23);
        assert_eq!(part_2("nppdvjthqldpwncqszvftbrmjlhg")      , 23);
        assert_eq!(part_2("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg") , 29);
        assert_eq!(part_2("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw")  , 26);
    }
}

