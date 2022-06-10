use rusty_opengl::hello_world;

fn main() {
    hello_world();
}

#[cfg(test)]
mod tests {
    #[test]
    fn hello_world_examples_test() {
        assert_eq!(4, 4);
    }
}