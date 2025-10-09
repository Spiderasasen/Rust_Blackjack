use std::io;
use rand::{rng, Rng};
use std::collections::linked_list::LinkedList;
fn main() {
    let mut money :i32 = 1000;

    //making the main menu
    loop {
        println!("Blackjack!!");
        println!("money: {}", money);

        //making the player chose what to do
        println!("1. Play the Game\n2. Exit");
        let mut choice: String = String::new(); //no matter what you need to read make the mut be a string

        //calling the scanner?
        io::stdin()
            .read_line(&mut choice) //letting the choice section be used as a reader
            .expect("Failed to read line"); //just in case the code cant read the input

        let choice: i32 = choice.trim().parse().expect("Please enter a number"); //you have to type this to call in a int

        //checking what the player chosed
        if choice == 1 {
            //checking if the player can bet
            if check_money(money) {
                println!("Sorry lad, you can't bet today");
                continue;
            }

            //running the function for the game
            money = blackjack(money);
        } else if choice == 2 {
            break;
        }else if choice == 3{
                testing_shit();
        } else {
            println!("Please, enter a valid option");
        }
    }
}

//checking how much money is gained or lost
fn changing_money(original_money: i32,changed_money: i32) -> i32{
    if changed_money < 0{ //losing money
        let total_money = original_money + changed_money;
        total_money
    }
    else{ //gaining money
        let total_money = original_money + changed_money;
        total_money
    }
}

//checking the player has enough money to bet
fn check_money(money: i32) -> bool{
    if money < 0{ //if the player has no money, then it will return true
        return true;
    }
    //otherwise it will return false
    false
}

//TODO, make blackjack
fn blackjack(money: i32) -> i32{
    //making a empty linked list
    let mut cards:LinkedList<String> = LinkedList::new();

    //making the bot cards
    let mut bot_cards:LinkedList<String> = LinkedList::new();

    //asking the user how much they want to bet
    println!("How much do you want to bet lad?");
    let mut amount = String::new();
    io::stdin()
        .read_line(&mut amount)
        .expect("Please enter a number");
    //making the input into a valid int
    let mut amount:i32 = amount.trim().parse().expect("Please enter a number");

    //making and printing the cards
    for _ in 0..2{
        cards.push_back(pretty_number(random_number()));
    }

    //making the bots cards
    bot_cards.push_back(pretty_number(random_number()));

    let clone_cards = cards.clone();

    //intal count
    let mut total:i32 =  total_value_of_cards(clone_cards, 0);

    //if 21, the user automatically wins
    if is_21(total){
        println!("YOU WIN!!!");
        amount =  amount * 5;
    }

    //looping
    loop{
        //printing the cards and the options of the player
        println!("Your Cards: {:?}", cards);
        println!("Bot's Cards: {:?}", bot_cards);
        println!("Total Value: {}", total);

        //checking if you got 21
        if is_21(total){
            println!("YOU WIN!!!");
            amount = amount * 2;
            break;
        }

        //checking if the player went over 21
        if over_21(total){
            println!("YOU LOST!!!");
            amount = amount * -2;
            break;
        }

        //asking what the player wants
        println!("What do you want to do?\n1.Hit\n2. Stand");

        //asking what the user wants to do
        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Please enter a valid option");

        let choice :i32 = choice.trim().parse().expect("Please enter a number");

        if choice == 1{ //player hits
            cards.push_back(pretty_number(random_number()));
            let cards_clone = cards.clone();
            total = total_value_of_cards(cards_clone, 0);
            continue;
        }
        else if choice == 2 { //player stands
            //establishing a boolean if it is 21 or over 21
            let mut is_value_21:bool = false;
            let mut is_value_over:bool = false;
            let mut is_total_16:bool = false;

            //establishing the bots total count after 1
            let bot_cards_clone :LinkedList<String> = bot_cards.clone();
            let mut total_bot_count:i32 = total_value_of_cards(bot_cards_clone, 0);

            //will countine looping unless the value is greater than 16
            loop{
                //printing the values
                println!("Bot Cards: {:?}", bot_cards);
                println!("Total Bot Count: {}", total_bot_count);

                //checking if the value is 16 or higher
                if total_bot_count >= 16{
                    is_total_16 = true;
                    break;
                }

                //counting if the bot got a 21
                if is_21(total_bot_count){
                    println!("YOU LOST!!!");
                    amount = amount * -2;
                    is_value_21 = true;
                    break;
                }

                //if the bot got over 21
                if over_21(total_bot_count){
                    println!("YOU WIN!!!");
                    amount = amount * 2;
                    is_value_over = true;
                    break;
                }

                //adding another card to the bot and also counting the cards
                bot_cards.push_back(pretty_number(random_number()));
                let bot_cards_clone: LinkedList<String> = bot_cards.clone();
                total_bot_count = total_value_of_cards(bot_cards_clone, 0);
            }

            //checking if either small values is true
            if is_value_21{ //bot got 21
                break;
            }

            if is_value_over{ //bot got over 21
                break;
            }

            if is_total_16{ // bot got a total value of 16 or higher
                //if the bot got a bigger value then the player, the bot wins
                if total < total_bot_count{
                    println!("YOU LOST!!!");
                    amount = amount * -2;
                    break;
                }

                //if the player got a bigger number then the bot, the player wins
                if total > total_bot_count{
                    println!("YOU WIN!!!");
                    amount = amount * 2;
                    break;
                }
            }
        }
        else{ //player put something else then 1 or 2
            println!("Please enter a valid option");
        }
    }

    //changing the amount of the money
    let new_money = changing_money(money, amount);
    new_money
}

//giving out a number out
fn random_number() -> i32{
    let mut rng = rand::thread_rng();

    let number = rng.gen_range(1..10);

    number
}

//pretty printing for the cards
fn pretty_number(card_num: i32) -> String{
    if card_num == 1{
        "Ace".to_string()
    }
    else if card_num == 10 {
        //making a array
        let bob = ["10", "Jack", "Queen", "King"]; //this is how to make an array

        //getting a random number to get the index
        let mut rng = rand::thread_rng();
        let index = rng.gen_range(0..3);

        //returning the string
        return bob[index].to_string(); //this is how to get the results
    }
    else{
        return card_num.to_string();
    }
}

//checking if you hit 21
fn is_21(card_amount: i32) -> bool{
    if card_amount == 21{
        return true;
    }
    false
}

//checking if you went over 21
fn over_21(card_amount: i32) -> bool{
    if card_amount > 21{
        return true;
    }
    false
}

//adding up the list to get the last value
fn total_value_of_cards(list: LinkedList<String>, previous_total_value: i32) -> i32{
    let mut num: i32 = previous_total_value;

    //if the privious value was 0, we just add it noramlly
    if num <= 0{
        for item in list{
            num += what_value(item, num);
        }
    }
    else{
        for item in list{
            num += what_value(item, num);
        }
    }
    num
}

//seeing what card you got
fn what_value(card: String, total_vale: i32) -> i32{
    if card == "Jack" || card == "Queen" || card == "King"{ //if jack, queen, or king, return 10
        10
    }
    else if card == "Ace"{ //checking on the ace
        return if over_21(total_vale + 11) { //if over 21, return 1
            1
        } else { //if not over 21 return 1
            11
        }
    }
    else{
        let safe_number: i32 = card.parse().expect("Please enter a number");
        return safe_number;
    }
}

// testing my code works
fn testing_shit(){
    println!("So you found this section of the code.\nWell this used to be a place for me to test shit.\n This is now a waste land, nothing is here.\n But hey you found a testing area, that im to lazy to remove so just enjoy the comment");
    if is_21(22){
        println!("Yes_21");
    }
    else{
        println!("No_21");
    }

    if over_21(22){
        println!("Yes_over");
    }
    else{
        println!("No_over");
    }

    let num: i32 = random_number();
    let num2: i32 = 10;
    let card:String = pretty_number(num);
    let card2: String = pretty_number(num2);
    println!("{}", card);
    println!("{}", card2);

    //checking the amount if what_value works
    println!("{}", what_value("Jack".to_string(), 10));
    println!("{}", what_value("Ace".to_string(), 11));
    println!("{}", what_value("Ace".to_string(), 10));
    println!("{}", what_value("15".to_string(), 11));

    //checking if total_value_of_cards works
    let mut cards:LinkedList<String> = LinkedList::new();
    cards.push_back("Jack".to_string());
    cards.push_back("Ace".to_string());
    let value = total_value_of_cards(cards, 0);
    println!("{}", value);
    // cards.push_back(4.to_string());
    // let old_values = value;
    // value = total_value_of_cards(cards, old_values);

}