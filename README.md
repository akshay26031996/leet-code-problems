# Rust solutions to leetcode problem

## Format
* File name - problem name
* Link - first line should contain the problem link
* Function - solution
* Tests - all tests should start with problem name in order to run problem specific tests only (cargo t problem\_name)

```rust
// LINK 
pub fn function_name(params) -> Result {
}

#[cfg(test)]
mod tests {
    use super::function_name;
    #[test]
    fn problem_name_test_name() {
    }
}
```
