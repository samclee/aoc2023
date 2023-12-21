use super::*;

#[test]
fn test_string_to_limit() {
    assert_eq!(12, string_to_limit("red"));
    assert_eq!(13, string_to_limit("green"));
    assert_eq!(14, string_to_limit("blue"));
}

#[test]
fn test_is_pull_bad() {
    assert_eq!(false, is_pull_bad("1 red"));
    assert_eq!(false, is_pull_bad("1 blue"));
    assert_eq!(false, is_pull_bad("1 green"));
    assert_eq!(true, is_pull_bad("14 red"));
    assert_eq!(true, is_pull_bad("15 blue"));
    assert_eq!(true, is_pull_bad("16 green"));
}

#[test]
fn test_is_round_bad() {
    assert_eq!(true, is_round_bad("1 red, 14 red"));
    assert_eq!(true, is_round_bad("1 blue, 14 red"));
    assert_eq!(true, is_round_bad("1 green, 14 red"));
    assert_eq!(false, is_round_bad("1 red, 1 red"));
    assert_eq!(false, is_round_bad("1 blue, 1 red"));
    assert_eq!(false, is_round_bad("1 green, 1 red"));
}

#[test]
fn test_is_game_good() {
    assert_eq!(true, is_game_good("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"));
    assert_eq!(false, is_game_good("Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red"));
}