fn main() {
    solution("abc", "c");
}

fn solution(word: &str, ending: &str) -> bool {
    let mut counter = 0;
    if ending.len() > word.len() {
        return false;
    }
    for i in ending.chars() {
        if i == word
            .chars()
            .nth(word.len() - ending.len() + counter)
            .unwrap()
        {
            counter += 1
        } else {
            return false;
        }
    }
    true
}
