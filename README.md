# mocker
Rust library for transforming text into lowercase and uppercase letters, resembling the Spongebob Mock.

# example

## printing Hello, world! in an alternating case.

    use mocker::AlternatingMocker;
    
    fn main() {
        let text = "Hello, world!";
    
        let mut mocker = AlternatingMocker::new();
    
        let mocked = mocker::mock(&text, &mut mocker);
    
        println!("{}", mocked);
    }