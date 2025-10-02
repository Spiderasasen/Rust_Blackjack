use std::io;

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
        } else {
            println!("Please, enter a valid option");
        }
    }


    //
    // println!("Money: {}", money);
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

fn blackjack(money: i32) -> i32{
    //asking the user how much they want to bet
    println!("How much do you want to bet lad?");
    let mut amount = String::new();
    io::stdin()
        .read_line(&mut amount)
        .expect("Please enter a number");

    let amount :i32 = amount.trim().parse().expect("Please enter a number");

    let new_money = changing_money(money, amount);
    new_money
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