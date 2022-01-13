Author: Brayden Olsen

Date Modified: 1/12/2022

Version: 1.0.1


A CLI Lemonade Stand Simulator for Linux & Windows built in Rust


Instructions
 
 Launching
 
  To launch Lemonade Stand Simulator, go into the appropriate directory for your OS and launch either 
  
  lemonadestand.exe (Windows) or run ./lemonadestand in the terminal (Linux)
  
 
 Playing
 
  To start, name your Lemonade Stand
  
  from the Main Menu you can do several things
  
   1. Quit:          Quits the game
   
   2. Save:          Goes to the save screen
   
   3. Load:          Goes to the load screen
   
   4. Upgrades:      Goes to the upgrades screen where you can upgrade your Lemonade Stand
   
   5. Start the day: Goes to the day setup screen where you can plan your strategy for the day
   
   Type in the corresponding number for the menu you want to go to
   
  Save
   
   Opening the Save menu prompts you asking what you would like to name your save file, this
   
   save file is stored in the same directory as the game executable
   
  Load
  
   Opening the Load menu prompts you asking what file you would like to load. Type in the file
   
   name of your save file to load it
   
  Upgrades
  
   From here you can select one of several upgrades for your stand
   
    1. Go Back:         Returns to Main Menu
	
	2. Advertising:     Upgrades your advertising, increasing the chances of a customer visiting your stand
	
    3. Quality:         Upgrades your lemonade quality, vastly increasing the chances of a customer visiting your stand
	
	4. Cost Efficiency: Decreases the cost of your lemonade
   
  Day Setup
  
   In this screen you can adjust your strategy for your lemonade stand
   
    1. Go Back:                  Returns to Main Menu
	
	2. Change Lemonade Price:    Brings you to the Lemonade Price screen
	
	3. Change Lemonade Quantity: Brings you to the Lemonade Quantity screen
	
	4. Change Location:          Brings you to the Location screen
	
	5. Start Day:                Brings you to the Day screen
	
  Change Lemonade Price
  
   In this screen you can adjust the price you charge customers for your lemonade, based on the price, 
   the chance of customers visiting your stand change
   
  Change Lemonade Quantity
  
   In this screen you can change the quantity of lemonade that you want to make for the day, keep in mind
   that lemonade costs money. Try to balance the amount of lemonade you make so that you end the day with none left
   
  Change Location
  
   In this screen there are 3 different locations you can choose from 
   
    1. Go Back:      Returns to Day Setup screen
	
	2. Home:         You will get less customer interest at home, but the chances of you being caught are nothing
	
	3. Main Street:  You will get more customer interest at the cost of an increased police presence
	
	4. State Street: The most dangerous place to sell lemonade, but more customers will see you
	
  Start Day
  
   This screen is where you actually make money, every 15 minutes there is a chance a customer will come that is calculated
   
   based on your upgrades, price, location, season, weather, and day. Each customer drinks a certain amount of lemonade, so make sure you have enough!
   
   Also calculated every 15 minutes is the chance you will get caught by the police, this is based on the location that you have
   
   chosen. When you are caught by the police the day is over and all your lemonade is dumped
   
   At the end of the day, your excess lemonade is dumped, and you are ready to repeat the process all over again
   
  
  
   
  
   