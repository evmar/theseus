pub fn main() {
    // The game reads its data files via paths relative to the current directory;
    // pass the game directory as the first argument to run from elsewhere.
    if let Some(dir) = std::env::args().nth(1) {
        std::env::set_current_dir(&dir).unwrap();
    }
    winpin::main();
}
