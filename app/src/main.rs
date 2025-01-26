use app::adder::adder::add;
use singleton;
use factory;
use factory::ordinary_maze::OrdinaryMaze;
use factory::magic_maze::MagicMaze;

fn main() {
    // Testing an internal library function
    let value = add(2, 3);
    println!("Hello, world! {value}");

    // Test a seperate workspace singleton library
    let singleton_instance = singleton::SingletonInt{};
    let mut value = singleton_instance.get_value();
    println!("singleton value: {value}");
    singleton_instance.increment_value();
    value = singleton_instance.get_value();
    println!("singleton value: {value}");

    // Running the factory
    // Option 1: The game starts with an ordinary maze.
    let ordinary_maze = OrdinaryMaze::new();
    let game_output: String = factory::game::run(ordinary_maze);
    println!("game_output: {game_output}");

    
    // Option 2: The game starts with a magic maze.
    let magic_maze = MagicMaze::new();
    let game_output: String = factory::game::run(magic_maze);
    println!("game_output: {game_output}");
    
}
