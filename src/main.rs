
fn print_position(idx: u32, vec: &Vec<u32>){
    let mut index = 0;
    for i in vec.iter() {
        if index  == idx {
            print!(" ({}) ", i);
        } else {
            print!(" {} ", i);
        }
        index += 1;
    }
    println!();
}

fn check_for_exit() -> bool {
    let mut input = String::new();
    let mut first_try = true;
    while input.trim() != "y" && input.trim() != "n" {
        if !first_try {
            println!("Invalid input. Please enter y or n");
        }
        println!("Do you want to exit? (y/n)");
        input.clear();
        std::io::stdin().read_line(&mut input).unwrap();
        first_try = false;
        println!("You entered: {}", input.trim());
    }
    
    input.trim() == "y"
}

fn get_winner(num_players: u32, show_steps: bool) -> u32 {
    let mut players: Vec<u32> = Vec::new();
    let mut current_idx = 0;

    for i in 1..=num_players {
        players.push(i);
    }

    if show_steps {
        print_position(0, &players);
    }
    
    while players.len() > 1{
        let kill_idx = (current_idx + 1) % players.len();
        players.remove(kill_idx);
        current_idx += 1;
        if current_idx >= players.len() {
            current_idx = 0;
            if show_steps {
                print_position(current_idx as u32, &players);
            }
        }
    }
    if show_steps{ 
        println!("Winner is {}", players[0])
    }

    return players[0];
}

fn main() {
    let mut exit = false;
    while !exit {
        println!("Enter number of players till which you want to get the winners:");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Input error");
        let limit: u32 = input.trim().parse().expect("Input error");

        for i in 1..=limit {
            println!("Winner for {} players is {}", i, get_winner(i, false));
        }

        exit = check_for_exit();
        if exit {
            println!("Exiting...");
        }   
    }
}
