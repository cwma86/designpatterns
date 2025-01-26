use factory::magic_maze::MagicMaze;
use factory::ordinary_maze::OrdinaryMaze;


#[test]
fn test_factory() {
    // Option 1: The game starts with an ordinary maze.
    let ordinary_maze = OrdinaryMaze::new();
    let game_output: String = factory::game::run(ordinary_maze);
    assert_eq!(
        game_output,
        "Loading resource. Starting Game. OrdinaryRoom: 2 OrdinaryRoom: 1 "
    );

    // Option 2: The game starts with a magic maze.
    let magic_maze = MagicMaze::new();
    let game_output: String = factory::game::run(magic_maze);
    assert_eq!(
        game_output,
        "Loading resource. Starting Game. Magic Room: Infinite Room Magic Room: Red Room "
    );

}
