// strings3.rs
//
// Execute `rustlings hint strings3` or use the `hint` watch subcommand for a
// hint.


fn trim_me(input: &str) -> String {
    // TODO: Remove whitespace from both ends of a string!
    let removed_str:String = input.to_string();
    let mut begin = 0;
    let mut end = removed_str.len();
    for ch in removed_str.chars() {
        if ch == ' ' {
            begin += 1;
        }
        else {
            break;
        }
    }
    for ch in removed_str.chars().rev() {
        if ch == ' ' {
            end -= 1;
        }
        else {
            break;
        }
    }
    let mut s:String = removed_str[begin..end].to_string();
    s
}

fn compose_me(input: &str) -> String {
    // TODO: Add " world!" to the string! There's multiple ways to do this!
    let append:String = String::from(" world!");
    let s = input.to_string();
    let s1 = s + &append;
    s1
}

fn replace_me(input: &str) -> String {
    // TODO: Replace "cars" in the string with "balloons"!
    let replaced_str:String = input.to_string();
    let new_replaced_str:String = replaced_str.replace("cars", "balloons");
    new_replaced_str
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trim_a_string() {
        assert_eq!(trim_me("Hello!     "), "Hello!");
        assert_eq!(trim_me("  What's up!"), "What's up!");
        assert_eq!(trim_me("   Hola!  "), "Hola!");
    }

    #[test]
    fn compose_a_string() {
        assert_eq!(compose_me("Hello"), "Hello world!");
        assert_eq!(compose_me("Goodbye"), "Goodbye world!");
    }

    #[test]
    fn replace_a_string() {
        assert_eq!(replace_me("I think cars are cool"), "I think balloons are cool");
        assert_eq!(replace_me("I love to look at cars"), "I love to look at balloons");
    }
}
