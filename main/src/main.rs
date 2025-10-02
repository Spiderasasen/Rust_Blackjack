use std::io;

fn main() {
    let money :i32 = 10;
    println!("Blackjack!!");
    println!("money: {}", money);
    println!("Please enter a value");
    let mut choice :String = String::new(); //no matter what you need to read make the mut be a string

    //calling the scanner?
    io::stdin()
        .read_line(&mut choice) //letting the choice section be used as a reader
        .expect("Failed to read line"); //just in case the code cant read the input

    let number: i32 = choice.trim().parse().expect("Please enter a number"); //you have to type this to call in a int

    let money :i32 = check_money(money, number);

    println!("Money: {}", money);
}

//checking how much money is gained or lost
fn check_money(original_money: i32,changed_money: i32) -> i32{
    if changed_money < 0{ //losing money
        let total_money = original_money + changed_money;
        total_money
    }
    else{ //gaining money
        let total_money = original_money + changed_money;
        total_money
    }
}