extern crate rand;

// Use for I/O
use std::io;

// use for getting random numbers - relies on extern crate rand
use rand::Rng;


enum Move {
	Rock,
	Paper,
	Scissors,
	Invalid,
	Quit
}

struct Stats {
    num_wins:  f32,
    num_losses:  f32,
	num_ties:  f32,
	num_games:  f32,
	rocks_thrown:  f32,
	papers_thrown:  f32,
	scissors_thrown:  f32
}

fn get_move()->Move{
    let mut to_return = String::new();
	let the_move: Move;
	println!("Enter choice (r,p,s) or q to quit");
    io::stdin().read_line(&mut to_return).expect("FAIL");
	let to_return = to_return.trim();
	match &to_return as &str{
		"r" => the_move = Move::Rock,
		"p" => the_move = Move::Paper,
		"s" => the_move = Move::Scissors,
		"q" => the_move = Move::Quit,
		_  =>  the_move = Move::Invalid,
	}

	the_move
}

fn get_comp_move() -> Move {
	let ret_val: Move;
	let rand_num = rand::thread_rng().gen_range(0, 3);
	match rand_num {
		0 =>ret_val = Move::Rock,
		1 =>ret_val = Move::Paper,
		2 =>ret_val = Move::Scissors,
		_ =>ret_val = Move::Invalid,
	}
	ret_val
}

fn print_moves(p: &Move, c: &Move ) {
	print!("Player chose: ");
	match p {
		&Move::Rock => println!("Rock"),
		&Move::Paper => println!("Paper"),
		&Move::Scissors => println!(" Scissors"),
		&Move::Invalid => println!("Please enter r, p, s, or q!!!!!"),
		&Move::Quit => println!("chose to quit"),
	}
	print!("Opponent chose: ");
		match c {
		&Move::Rock => println!("Rock"),
		&Move::Paper => println!("Paper"),
		&Move::Scissors => println!(" Scissors"),
		&Move::Invalid => println!("Please enter r, p, s, or q!!!!!"),
		&Move::Quit => println!("chose to quit"),
	}
	
}

fn display_stats(s: &Stats) {
	println!("Player Stats:");
	if s.num_wins == 0.0 {
		println!("Wins: {} (0.00%) ", s.num_wins);
	}
	else {
		println!("Wins: {} ({:.2}%) ", s.num_wins, (100.0* s.num_wins/s.num_games));
	}
	if s.num_ties == 0.0 {
		println!("Ties: {} (0.00%)", s.num_ties);
	}
	else{
		println!("Ties: {} ({:.2}%)", s.num_ties, (100.0* s.num_ties/s.num_games));
	}
	
	if s.num_losses ==0.0	{
		println!("Losses: {} (0.00%)", s.num_losses);
	}
	else{
		println!("Losses: {} ({:.2}%)", s.num_losses,(100.0* s.num_losses/s.num_games));
	}
	println!("Rocks: {}", s.rocks_thrown);
	println!("Papers: {}", s.papers_thrown);
	println!("Scissers: {}", s.papers_thrown);
}

fn play_round(p: &Move, c: &Move, stat: & mut Stats){
	match p {
		&Move::Rock => {
			match c {
				&Move::Scissors => {
					println!("player wins");
					stat.num_wins +=1.0;
					stat.num_games +=1.0;
					stat.rocks_thrown +=1.0;
				},
				&Move::Rock => {
					println!("It's a tie");
					stat.rocks_thrown +=1.0;
					stat.num_ties +=1.0;
					stat.num_games +=1.0;
				},
				&Move::Paper =>{
					println!("You Lose.");
					stat.num_losses +=1.0;
					stat.rocks_thrown +=1.0;
					stat.num_games +=1.0;
				},
				_=> {}
			}
		},
		&Move::Paper =>{
			match c {
				&Move::Rock => {
					println!("player wins");
					stat.num_wins +=1.0;
					stat.papers_thrown +=1.0;
					stat.num_games +=1.0;
				},
				&Move::Paper => {
					println!("It's a tie");
					stat.num_ties +=1.0;
					stat.papers_thrown +=1.0;
					stat.num_games +=1.0;
				},
				&Move::Scissors =>{
					println!("You Lose.");
					stat.num_losses +=1.0;
					stat.papers_thrown +=1.0;
					stat.num_games +=1.0;
				},
				_=> {}
			}
		},
		&Move::Scissors =>{
			match c {
				&Move::Paper => {
					println!("player wins");
					stat.num_wins +=1.0;
					stat.scissors_thrown +=1.0;
					stat.num_games +=1.0;
					},
				&Move::Scissors => {
					println!("It's a tie");
					stat.num_ties +=1.0;
					stat.scissors_thrown +=1.0;
					stat.num_games +=1.0;
				},
				&Move::Rock =>{
					println!("You Lose.");
					stat.num_losses +=1.0;
					stat.scissors_thrown +=1.0;
					stat.num_games +=1.0;
				},
				_=> {},
			}
		},
		_=> {},
	}
}

fn main() {
	let mut score_card = Stats { num_wins :0.0, num_losses: 0.0, num_ties: 0.0, num_games: 0.0, rocks_thrown: 0.0, papers_thrown: 0.0, scissors_thrown: 0.0};
	let mut player_move: Move;
	let mut comp_move: Move;
	loop {
		player_move = get_move();
		comp_move = get_comp_move();

		match player_move {
			Move::Invalid => println!("Please enter r, p, s, or q!!!!!"),
			Move::Quit => break,
			_ => {
				print_moves(&player_move, &comp_move);
				play_round(&player_move, &comp_move, &mut score_card);
				
			}
		}//end match
    }//end loop
	display_stats(&score_card);
}
