// author: Brayden
// name: lemonadestand
// version: W1.0.1
// date: 12/23/21 (last edited) 12/20/2021 (created)
// language: Rust


// =======================================================
// includes
// =======================================================
extern crate rand;
extern crate clearscreen;

use std::io;
use std::io::{Write, BufRead, BufReader, ErrorKind};
use std::{thread, time};
use std::fs::File;
use rand::Rng;



// =======================================================
// structs
// =======================================================
struct Player // holds data related to the player
{
	name: String,
	money: f32,
	customer_interest: u32,
	base_customer_interest: u32,
}

struct Session // holds data related to the day session
{
	price: f32,
	location: Location,
	quantity: f32,
	customer_count: u32,
	starting_money: f32,
}

struct World // holds data related to the world
{
	month_day: u32,
	week_day: Day,
	weather: Weather,
	season: Season,
}

struct Upgrades // holds player upgrade data
{
	lvl_advertising: u32,
	cost_advertising: u32,
	lvl_quality: u32,
	cost_quality: u32,
	lvl_cost_efficiency: u32,
	cost_cost_efficiency: u32,
}

struct Constants // holds variables that need to be constant
{
	base_price_per_gal: f32,
	error_wait_time: u64,
}



// =======================================================
// enumerators
// =======================================================
enum Season // holds enumerators needed for all 4 seasons
{
	Summer,
	Autumn,
	Winter,
	Spring,
}

impl Season // functions connected to Season
{
	fn get_str(&self) -> String // turns the current Season stored into a string
	{
		let get_season = match self
		{
			Season::Summer => String::from("Summer"),
			Season::Autumn => String::from("Autumn"),
			Season::Winter => String::from("Winter"),
			Season::Spring => String::from("Spring"),
		};
		get_season // return get_season
	}
}


enum Day // holds all week days
{
	Sunday,
	Monday,
	Tuesday,
	Wednesday,
	Thursday,
	Friday,
	Saturday,
}

impl Day // functions connected to Day
{
	fn get_str(&self) -> String // turns the current week Day into a string
	{
		let get_week_day = match self
		{
			Day::Sunday    => String::from("Sunday"),
			Day::Monday    => String::from("Monday"),
			Day::Tuesday   => String::from("Tuesday"),
			Day::Wednesday => String::from("Wednesday"),
			Day::Thursday  => String::from("Thursday"),
			Day::Friday    => String::from("Friday"),
			Day::Saturday  => String::from("Saturday"), 
		};
		get_week_day // return get_week_day
	}
}


enum Weather // currently holds 3 types of weather, maybe more will be added
{
	Sunny,
	Cloudy,
	Rainy,
}

impl Weather // function connected to Weather
{
	fn get_str(&self) -> String // turns current Weather into a string
	{
		let get_weather = match self
		{
			Weather::Sunny  => String::from("Sunny"),
			Weather::Cloudy => String::from("Cloudy"),
			Weather::Rainy  => String::from("Rainy"),
		};
		get_weather //return get_weather
	}
}


enum Location // holds currently 3 locations, more may be added
{
	MainStreet,
	StateStreet,
	Home,
}

impl Location // functions connected to Location
{
	fn get_str(&self) -> String // converts current location to a string
	{
		let get_location = match self
		{
			Location::MainStreet  => String::from("Main Street"),
			Location::StateStreet => String::from("State Street"),
			Location::Home        => String::from("Home"),
		};
		get_location //return get_location
	}
}


enum State // holds the multiple states the program can be in, there are no transitions
{
	Quit,
	Start,
	MainMenu,
	Upgrades,
	Save,
	Load,
	DaySetup,
	SetLocation,
	SetPrice,
	SetQuantity,
	DaySession,
	DayEnd,
}



// =======================================================
// main
// =======================================================
fn main()
{
	let mut player = Player
	{
		name: String::new(),
		money: 10.0, // in $
		customer_interest: 20,
		base_customer_interest: 20, // starting customer interest, used in the upgrade screen
	};

	let mut world = World
	{
		month_day: 0, // day of the month
		week_day: Day::Sunday,
		weather: Weather::Sunny,
		season: Season::Summer,
	};

	let mut session = Session
	{
		price: 1.0, // in $
		location: Location::Home,
		quantity: 1.0, // lemonade quantity in gallons
		customer_count: 0,
		starting_money: player.money, // used to gauge how much money was earned per day in $
	};

	let mut upgrades = Upgrades
	{
		lvl_advertising: 1,
		cost_advertising: 10, // in $
		lvl_quality: 1,
		cost_quality: 15, // in $
		lvl_cost_efficiency: 1,
		cost_cost_efficiency: 20, // in $
	};

	let constants = Constants
	{
		base_price_per_gal: 2.5, // in $
		error_wait_time: 1500, // in milliseconds
	};

	let mut current_state = State::Start;

	loop // main loop
	{
		match current_state // this match statement gets the current_state from the function returns, which in turn changes the state at the next loop
		{
			State::Quit        => break,
			State::Start       => current_state = screen_start(&mut player),
			State::MainMenu    => current_state = screen_main_menu(&mut player, &mut world, &constants),
			State::Upgrades    => current_state = screen_upgrades(&mut player, &mut upgrades, &constants),
			State::Save        => current_state = screen_save(&player, &world, &upgrades, &constants),
			State::Load        => current_state = screen_load(&mut player, &mut world, &mut upgrades, &constants),
			State::DaySetup    => current_state = screen_day_setup(&mut player, &mut world, &mut session, &mut upgrades, &constants),
			State::SetLocation => current_state = screen_set_location(&mut session, &constants),
			State::SetPrice    => current_state = screen_set_price(&mut session, &constants),
			State::SetQuantity => current_state = screen_set_quantity(&mut session, &mut upgrades, &constants),
			State::DaySession  => current_state = screen_day_session(&mut player, &mut world, &mut session),
			State::DayEnd      => // launches the Day End Screen and launched the next_day function
			{ 
				current_state = screen_day_end(&mut player, &mut world, &mut session);
				next_day(&mut world);
			}
		}
	}
}



// =======================================================
// start screen
// =======================================================
fn screen_start(local_player: &mut Player) -> State
{
	let mut user_input = String::new();

	clearscreen::clear().expect("Failed to clear screen"); // clears terminal
	println!("|----------------------------------------|");
	println!("|             Lemonade Stand             |");
	println!("|              By : Brayden              |");
	println!("|----------------------------------------|");
	println!("To proceed, name your Lemonade Stand:");
	println!("========================================================================");

	// user input
	io::stdin().read_line(&mut user_input).expect("Failed to read line");
	local_player.name = user_input.trim().to_string();

	State::MainMenu // return MainMenu
}



// =======================================================
// main menu
// =======================================================
fn screen_main_menu(local_player: &mut Player, local_world: &mut World, local_constants: &Constants) -> State
{
	let mut user_input = String::new();
	local_player.money = (local_player.money * 100.0).round() / 100.0; // rounds local_player.money to the 2s place

	clearscreen::clear().expect("Failed to clear screen"); // clears terminal
	println!("Welcome to your Lemonade Stand: {}", local_player.name);
	println!("--------------------------------------------------");
	println!("The current day is {}, Day {}", local_world.week_day.get_str(), local_world.month_day + 1);
	println!("The current season is {}", local_world.season.get_str());
	println!("The current weather is {}", local_world.weather.get_str());
	println!("Your funds: ${:.2}", local_player.money);
	println!("--------------------------------------------------");
	println!("Type the following commands to go to them:");
	println!("1 - Quit");
	println!("2 - Save");
	println!("3 - Load");
	println!("4 - Upgrades");
	println!("5 - Start the day");
	println!("========================================================================");

	// main menu user input
	io::stdin().read_line(&mut user_input).expect("Failed to read line");
	// convert 'user_input' into an int with error handling
	// 'user_input' in this case is an int version of the previous string 'user_input' with error handling
	let user_input: u32 = match user_input.trim().parse()
	{
		Ok(num) => num,
		Err(_)  => 0,
	};

	if user_input == 1 // quit
	{
		State::Quit // return Quit
	}
	else if user_input == 2 // go to upgrades screen
	{
		State::Save // return Save
	}
	else if user_input == 3 // go to day setup screen
	{
		State::Load // return Load
	}
	else if user_input == 4 // go to day setup screen
	{
		State::Upgrades // return Upgrades
	}
	else if user_input == 5 // go to day setup screen
	{
		State::DaySetup // return DaySetup
	}
	else
	{
		println!("Error, please enter a correct command number.");
		thread::sleep(time::Duration::from_millis(local_constants.error_wait_time)); // wait for milliseconds
		State::MainMenu // returns back to MainMenu
	}
}



// =======================================================
// upgrades
// =======================================================
fn screen_upgrades(local_player: &mut Player, local_upgrades: &mut Upgrades, local_constants: &Constants) -> State
{
	loop
	{
		let mut user_input = String::new();
		local_player.money = (local_player.money * 100.0).round() / 100.0; // rounds local_player.money to the 2s place

		clearscreen::clear().expect("Failed to clear screen"); // clears terminal
		println!("Welcome to the Upgrade Screen!");
		println!("--------------------------------------------------");
		println!("Your current funds: ${:.2}", local_player.money);
		println!("--------------------------------------------------");
		println!("Please select an upgrade to buy:");
		println!("1 - Go Back");
		println!("2 - Level {} -> Level {} Advertising - ${:.2}", local_upgrades.lvl_advertising, local_upgrades.lvl_advertising + 1, local_upgrades.cost_advertising * local_upgrades.lvl_advertising);
		println!("3 - Level {} -> Level {} Quality - ${:.2}", local_upgrades.lvl_quality, local_upgrades.lvl_quality + 1, local_upgrades.cost_quality * local_upgrades.lvl_quality);
		println!("4 - Level {} -> Level {} Cost Efficiency - ${:.2}", local_upgrades.lvl_cost_efficiency, local_upgrades.lvl_cost_efficiency + 1, local_upgrades.cost_cost_efficiency * local_upgrades.lvl_cost_efficiency);
		println!("========================================================================");

		// user input
		io::stdin().read_line(&mut user_input).expect("Failed to read line");
		let user_input: u32 = match user_input.trim().parse()
		{
			Ok(num) => num,
			Err(_)  => 0,
		};

		if user_input == 1 // leave
		{
			local_player.customer_interest = local_player.base_customer_interest + (5 * local_upgrades.lvl_advertising) + (10 * local_upgrades.lvl_quality);
			break;
		}
		else if user_input == 2 // buy advertising
		{
			if local_player.money >= (local_upgrades.lvl_advertising * local_upgrades.cost_advertising) as f32 // check if player has enough money
			{
				local_player.money -= (local_upgrades.lvl_advertising * local_upgrades.cost_advertising) as f32; // subtract the cost of the upgrade from the players wallet
				local_upgrades.lvl_advertising += 1; // increment the upgrade level by 1
			}
			else // if the player doesn't have enough cash
			{
				println!("Sorry, you don't have enough funding to purchase that upgrade.");
				thread::sleep(time::Duration::from_millis(local_constants.error_wait_time)); // wait for milliseconds
			}
		}
		else if user_input == 3 // buy quality
		{
			if local_player.money >= (local_upgrades.lvl_quality * local_upgrades.cost_quality) as f32 // check if player has enough money
			{
				local_player.money -= (local_upgrades.lvl_quality * local_upgrades.cost_quality) as f32; // subtract the cost of the upgrade from the players wallet
				local_upgrades.lvl_quality += 1; // increment the upgrade level by 1
			}
			else // if the player doesn't have enough cash
			{
				println!("Sorry, you don't have enough funding to purchase that upgrade.");
				thread::sleep(time::Duration::from_millis(local_constants.error_wait_time)); // wait for milliseconds
			}
		}
		else if user_input == 4 // buy cost efficiency
		{
			if local_player.money >= (local_upgrades.lvl_cost_efficiency * local_upgrades.cost_cost_efficiency) as f32 // check if player has enough money
			{
				local_player.money -= (local_upgrades.lvl_cost_efficiency * local_upgrades.cost_cost_efficiency) as f32; // subtract the cost of the upgrade from the players wallet
				local_upgrades.lvl_cost_efficiency += 1; // increment the upgrade level by 1
			}
			else // if the player doesn't have enough cash
			{
				println!("Sorry, you don't have enough funding to purchase that upgrade.");
				thread::sleep(time::Duration::from_millis(local_constants.error_wait_time)); // wait for milliseconds
			}
		}
		else
		{
			println!("Error, please enter a correct command number.");
			thread::sleep(time::Duration::from_millis(local_constants.error_wait_time)); // wait for milliseconds
		}
	}
	State::MainMenu // return MainMenu
}



// =======================================================
// save
// =======================================================
fn screen_save(local_player: &Player, local_world: &World, local_upgrades: &Upgrades, local_constants: &Constants) -> State
{
	let mut user_input = String::new();

	clearscreen::clear().expect("Failed to clear screen"); // clears terminal
	println!("Please enter your the name of your save file you would like to be saved:");
	println!("========================================================================");

	// user input
	io::stdin().read_line(&mut user_input).expect("Failed to read line.");

	let mut file = match File::create(&user_input.trim()) // creates file under variable file with error handling
	{
		Ok(f) => f,
		Err(error) => 
		{
			println!("Error creating file: {:?}", error);
			thread::sleep(time::Duration::from_millis(local_constants.error_wait_time)); // wait for milliseconds
			return State::MainMenu;
		}
	};

	// creates a file with each variable on their own line
	let save_contents = format!
	(
		"{}\n{}\n{}\n\n{}\n{}\n{}\n{}\n\n{}\n{}\n{}", 
		local_player.name, 
		local_player.money, 
		local_player.customer_interest, 

		local_world.month_day,
		local_world.week_day.get_str(),
		local_world.weather.get_str(), // may not be necessary
		local_world.season.get_str(),

		local_upgrades.lvl_advertising,
		local_upgrades.lvl_quality,
		local_upgrades.lvl_cost_efficiency
	);

	match file.write_all(save_contents.as_bytes())
	{
		Ok(f) => f,
		Err(error) => 
		{
			println!("Error writing to file: {:?}", error);
			thread::sleep(time::Duration::from_millis(local_constants.error_wait_time)); // wait for milliseconds
			return State::MainMenu;
		},
	};
	println!("All files saved sucessfully!");
	thread::sleep(time::Duration::from_millis(local_constants.error_wait_time)); // wait for milliseconds
	State::MainMenu
}



// =======================================================
// load
// =======================================================
fn screen_load(local_player: &mut Player, local_world: &mut World, local_upgrades: &mut Upgrades, local_constants: &Constants) -> State
{
	let mut user_input = String::new();
	let mut contents = [String::new(), String::new(), String::new(), String::new(), String::new(), String::new(), String::new(), String::new(), String::new(), String::new(), String::new(), String::new()];
	let mut index = 0;

	clearscreen::clear().expect("Failed to clear screen"); // clears terminal
	println!("Please enter the name of your save file you would like to be loaded:");
	println!("====================================================================");

	// user input
	io::stdin().read_line(&mut user_input).expect("Failed to read line.");
	let file =  match File::open(user_input.trim()) // opens the file that user_input specified
	{
		Ok(f) => f,
		Err(error) => match error.kind()
		{
			ErrorKind::NotFound => 
			{
				println!("Error opening file: File not found.");
				thread::sleep(time::Duration::from_millis(local_constants.error_wait_time)); // wait for milliseconds
				return State::MainMenu; // returns back to main menu
			},
			other_error =>
			{
				println!("Error opening file: {:?}", other_error);
				thread::sleep(time::Duration::from_millis(local_constants.error_wait_time)); // wait for milliseconds
				return State::MainMenu; // returns back to main menu
			},
		},
	};

	let reader = BufReader::new(file); // creates a buffer reader off the new file

	println!("Loading...");

	// reads the file line by line and puts each line into a string array
	for line in reader.lines() 
	{
		contents[index] = match line
		{
			Ok(ln) => ln,
			Err(_) => String::new(),
		};
		index += 1;
    };

	// set variables to their loaded counterparts
	local_player.name              = contents[0].trim().to_string(); // default &str
	local_player.money             = match contents[1].trim().parse() { Ok(num) => num, Err(_) => 0.0 };
	local_player.customer_interest = match contents[2].trim().parse() { Ok(num) => num, Err(_) => 20 };
	local_world.month_day          = match contents[4].trim().parse() { Ok(num) => num, Err(_) => 0 };
	local_world.week_day           = match contents[5].trim()
	{
		"Sunday"    => Day::Sunday,
		"Monday"    => Day::Monday,
		"Tuesday"   => Day::Tuesday,
		"Wednesday" => Day::Wednesday,
		"Thursday"  => Day::Thursday,
		"Friday"    => Day::Friday,
		"Saturday"  => Day::Saturday,
		_           => Day::Sunday, // error, fixes itself after next_day is called
	};
	local_world.weather = match contents[6].trim()
	{
		"Sunny"  => Weather::Sunny,
		"Cloudy" => Weather::Cloudy,
		"Rainy"  => Weather::Rainy,
		_        => Weather::Sunny, // error, fixes itself after next_day is called
	};
	local_world.season = match contents[7].trim()
	{
		"Summer" => Season::Summer,
		"Autumn" => Season::Autumn,
		"Winter" => Season::Winter,
		"Spring" => Season::Spring,
		_        => Season::Summer, // error
	};
	local_upgrades.lvl_advertising     = match contents[9].trim().parse() { Ok(num) => num, Err(_) => 0 };
	local_upgrades.lvl_quality         = match contents[10].trim().parse() { Ok(num) => num, Err(_) => 0 };
	local_upgrades.lvl_cost_efficiency = match contents[11].trim().parse() { Ok(num) => num, Err(_) => 0 };

	println!("Files loaded successfully.");

	thread::sleep(time::Duration::from_millis(local_constants.error_wait_time)); // wait for milliseconds
	State::MainMenu
}



// =======================================================
// day setup
// =======================================================
fn screen_day_setup(local_player: &mut Player, local_world: &mut World, local_session: &mut Session, local_upgrades: &mut Upgrades, local_constants: &Constants) -> State
{
	let mut user_input = String::new();
	let current_cost = local_session.quantity * (local_constants.base_price_per_gal / (0.2 * (local_upgrades.lvl_cost_efficiency) as f32 + 0.8));
	local_player.money = (local_player.money * 100.0).round() / 100.0; // rounds local_player.money to the 2s place

	clearscreen::clear().expect("Failed to clear screen"); // clears terminal
	println!("Week Day: {}", local_world.week_day.get_str());
	println!("Day: {}", local_world.month_day + 1);
	println!("Season: {}", local_world.season.get_str());
	println!("Weather: {}", local_world.weather.get_str());
	println!("Your funds: ${:.2}", local_player.money);
	println!("--------------------------------------------------");
	println!("Current Lemonade Price: ${:.2}", local_session.price);
	println!("Current Lemonade Quantity: {:.1} gallon(s)", local_session.quantity);
	println!("Current Lemonade Total Cost: ${:.2}", current_cost);
	println!("Current Stand Location: {}", local_session.location.get_str()); // local_session.location
	println!("--------------------------------------------------");
	println!("Type the following commands to go to them:");
	println!("1 - Go Back");
	println!("2 - Change Lemonade Price");
	println!("3 - Change Lemonade Quantity");
	println!("4 - Change Location");
	println!("5 - Start Day");
	println!("========================================================================");

	// user input
	io::stdin().read_line(&mut user_input).expect("Failed to read line");
	let user_input: u32 = match user_input.trim().parse()
	{
		Ok(num) => num,
		Err(_)  => 0,
	};

	if user_input == 1 // go to back to main menu
	{
		State::MainMenu // return MainMenu
	}
	else if user_input == 2 // go to lemonade price screen
	{
		State::SetPrice // return SetPrice
	}
	else if user_input == 3	// go to the quantity screen
	{
		State::SetQuantity // return SetQuantity
	}

	else if user_input == 4 // go to location screen
	{
		State::SetLocation // return SetLocation
	}
	else if user_input == 5 // go to the day session screen
	{
		if local_player.money >= current_cost // checks if the player can afford the lemonade quantity
		{
			local_player.money -= current_cost;
			State::DaySession // return DaySession
		}
		else // cannot afford lemonade
		{
			println!("Not enought money to buy lemonade.");
			thread::sleep(time::Duration::from_millis(local_constants.error_wait_time)); // wait for milliseconds
			State::DaySetup // return back to DaySetup
		}
	}
	else
	{
		println!("Error, please enter a correct command number.");
		thread::sleep(time::Duration::from_millis(local_constants.error_wait_time)); // wait for milliseconds
		State::DaySetup // return to DaySetup
	}
}



// =======================================================
// set location
// =======================================================
fn screen_set_location(local_session: &mut Session, local_constants: &Constants) -> State
{
	let mut user_input = String::new();

	clearscreen::clear().expect("Failed to clear screen"); // clears terminal
	println!("Based on the police presence, there is a chance they will shut down your stand.");
	println!("Please select a Location:");
	println!("1 - Go Back");
	println!("2 - Home (Default) - No police presence");
	println!("3 - Main Street - Minor police presence");
	println!("4 - State Street - Moderate police presence");
	println!("========================================================================");

	// user input
	io::stdin().read_line(&mut user_input).expect("Failed to read line");
	let user_input: u32 = match user_input.trim().parse()
	{
		Ok(num) => num,
		Err(_)  => 0,
	};

	if user_input == 1 // go to back to main menu
	{
		State::MainMenu // return MainMenu
	}
	else if user_input == 2 // set location to Home, then return to DaySetup
	{
		local_session.location = Location::Home;
		State::DaySetup // return DaySetup
	}
	else if user_input == 3 // set location to MainStreet, then return to DaySetup
	{
		local_session.location = Location::MainStreet;
		State::DaySetup // return DaySetup
	}
	else if user_input == 4 // set location to StateStreet, then return to DaySetup
	{
		local_session.location = Location::StateStreet;
		State::DaySetup // return DaySetup
	}
	else
	{
		println!("Error, please enter a correct command number.");
		thread::sleep(time::Duration::from_millis(local_constants.error_wait_time)); // wait for milliseconds
		State::SetLocation // return Location
	}
}



// =======================================================
// set price
// =======================================================
fn screen_set_price(local_session: &mut Session, local_constants: &Constants) -> State
{
	let mut user_input = String::new();

	clearscreen::clear().expect("Failed to clear screen"); // clears terminal
	println!("Please enter your desired price (above 0.1) you will charge your customers:");
	println!("===========================================================================");

	// user input
	io::stdin().read_line(&mut user_input).expect("Failed to read line");
	let user_input: f32 = match user_input.trim().parse()
	{
		Ok(num) => num,
		Err(_)  => 1.0,
	};

	if user_input >= 0.1 // checks if the user has entered a quantity above 0.1, if not, error out
	{
		local_session.price = user_input;
		State::DaySetup // return DaySetup
	}
	else
	{
		println!("Error, please enter a correct value");
		thread::sleep(time::Duration::from_millis(local_constants.error_wait_time)); // wait for milliseconds
		State::SetPrice // return SetPrice
	}
}



// =======================================================
// set quantity
// =======================================================
fn screen_set_quantity(local_session: &mut Session, local_upgrades: &mut Upgrades, local_constants: &Constants) -> State
{
	let mut user_input = String::new();
	let price_per_gallon = local_constants.base_price_per_gal / (0.2 * (local_upgrades.lvl_cost_efficiency) as f32 + 0.8); // pricing equation based on current level of cost efficiency

	clearscreen::clear().expect("Failed to clear screen"); // clears terminal
	println!("The current price of lemonade is {:.2} per gallon.", price_per_gallon);
	println!("Each cup of lemonade sold contains 0.2 gallons.");
	println!("Extra lemonade will be dumped at the end of the day.");
	println!("-----------------------------------------------------------------------------");
	println!("Type in the quantity (above 0.2) of lemonade you would like to make this day:");
	println!("=============================================================================");

	// user input
	io::stdin().read_line(&mut user_input).expect("Failed to read line");
	let user_input: f32 = match user_input.trim().parse()
	{
		Ok(num) => num,
		Err(_)  => 1.0,
	};

	if user_input >= 0.2
	{
		local_session.quantity = user_input;
		State::DaySetup // return DaySetup
	}
	else
	{
		println!("Error, please enter a correct value");
		thread::sleep(time::Duration::from_millis(local_constants.error_wait_time)); // wait for milliseconds
		State::SetQuantity // return SetQuantity
	}
}



// =======================================================
// day session
// =======================================================
fn screen_day_session(local_player: &mut Player, local_world: &mut World, local_session: &mut Session) -> State
{
	local_session.starting_money = local_player.money;
	local_session.customer_count = 0;
	let mut hour = 11.00; // 11:00 AM
	let mut police_probability = 0;
	let mut session_customer_interest = local_player.customer_interest as f32;
	session_customer_interest *= (local_session.price+0.2).powf(-1.1); // multiply customer interest based on price by (price+0.2)^(1.1)

	session_customer_interest *= match local_world.weather
	{
		Weather::Cloudy => 0.75,
		Weather::Rainy  => 0.5,
		_               => 1.0,
	};
	
	session_customer_interest *= match local_world.season
	{
		Season::Summer => 1.5,
		Season::Winter => 0.5,
		_              => 1.0,
	};

	session_customer_interest *= match local_world.week_day
	{
		Day::Saturday => 1.25,
		Day::Sunday   => 0.75,
		_             => 1.0,
	};
	
	match local_session.location
	{
		Location::MainStreet =>
		{
			session_customer_interest *= 1.5;
			police_probability = 2;
		},
		Location::StateStreet =>
		{
			session_customer_interest *= 2.0;
			police_probability = 5;
		},
		_                     =>
		{
			session_customer_interest *= 1.0;
		}
	};
	
	let compare_customer_interest = session_customer_interest as i32;

	while hour <= 17.00 // 5:00 PM
	{
		local_session.quantity = (local_session.quantity * 100.0).round() / 100.0;
		let customer_chance = rand::thread_rng().gen_range(1, 100); // generates a random number from 1 to 100
		let police_chance = rand::thread_rng().gen_range(1,100); // generates a random number from 1 to 100

		clearscreen::clear().expect("Failed to clear screen"); // clears terminal
		println!("Current Location: {}", local_session.location.get_str());
		println!("Current funds: ${:.2}", local_player.money);
		println!("Customers visited: {}", local_session.customer_count);
		println!("The current hour is: {:.2}", hour);
		println!("Amount of lemonade remaining: {:.1} gallon(s)", local_session.quantity);
		println!("------------------------------------------------------------------------");
		//println!("Customer interest: {}", session_customer_interest);
		
		if customer_chance <= compare_customer_interest // if the random number falls within customer interest, then sell lemonade
		{
			println!("You got a customer!");
			if local_session.quantity >= 0.2
			{
				println!("Customer bought lemonade for ${:.2}", local_session.price);
				local_player.money += local_session.price; // adds the price of the lemonade the customer bought to the player's wallet
				local_session.quantity -= 0.2; // subtract the quantity the customer drinks from the total quantity
				local_session.customer_count += 1; // increments the customer count by 1
			}
			else
			{
				println!("However, there was not enough lemonade to give them.");
			}
		}
		else if police_chance <= police_probability // checks if the police probability falls within the chance of police officer appearing
		{
			println!("Uh Oh, the police saw you and you had to end the day early.");
			thread::sleep(time::Duration::from_millis(3000)); // wait for 3 seconds
			return State::DayEnd; // return Day::End
		}
		
		thread::sleep(time::Duration::from_millis(1000)); // wait for 1 second
		hour += 0.25; // increment hour by 15 minutes
	}
	State::DayEnd // return State::DayEnd
}



// =======================================================
// day end
// =======================================================
fn screen_day_end(local_player: &mut Player, local_world: &mut World, local_session: &mut Session) -> State
{
	let money_earned = ((local_player.money - local_session.starting_money) * 100.0).round() / 100.0; // totals the money earned and rounds it to the 2s place
	let mut user_input = String::new();
	local_session.quantity = 1.0;

	clearscreen::clear().expect("Failed to clear screen"); // clears terminal
	println!("Day {} Over", local_world.month_day + 1);
	println!("{} Customers visited your stand", local_session.customer_count);
	println!("You made ${:.2}", money_earned);
	println!("========================================================================");
	println!("1 - Continue");

	// user input
	io::stdin().read_line(&mut user_input).expect("Failed to read line");

	State::MainMenu // no matter what the user inputs, go to the next screen
}



// =======================================================
// next day function
// =======================================================
fn next_day(local_world: &mut World)
{
	local_world.month_day += 1; // increment the month day
	
	if local_world.month_day >= 28 // 28 is 4 weeks of 7 days
	{
		local_world.month_day = 1;
		local_world.season = match local_world.season // changes to new season based on the previous season
		{
			Season::Summer => Season::Autumn,
			Season::Autumn => Season::Winter,
			Season::Winter => Season::Spring,
			Season::Spring => Season::Summer,
		};
	}
	
	let new_week_day = local_world.month_day % 7; // set equal to 0-6 depending on the month day
	
	local_world.week_day = match new_week_day
	{
		0 => Day::Sunday,
		1 => Day::Monday,
		2 => Day::Tuesday,
		3 => Day::Wednesday,
		4 => Day::Thursday,
		5 => Day::Friday,
		_ => Day::Saturday,
	};

	let weather_chance = rand::thread_rng().gen_range(0, 2); // generates a random number from 0-2 with 3 possibilities
	
	local_world.weather = match weather_chance
	{
		0 => Weather::Sunny,
		1 => Weather::Cloudy,
		_ => Weather::Rainy,
	};
}