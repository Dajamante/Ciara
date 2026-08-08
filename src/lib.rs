/// Returns the message used by the demo binary.
pub fn level_up() -> &'static str {
    "Ciara says: level up!"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn levels_up() {
        assert_eq!(level_up(), "Ciara says: level up!");
    }
}
