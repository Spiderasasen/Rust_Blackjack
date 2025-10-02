fn main() {
    let money = 10;
    let money_lost = -5;
    let money_won = 5;
    println!("Blackjack!!");
    println!("money: {}", money);
    let money = check_money(money, money_lost);
    println!("money: {}", money);
    let money = check_money(money, money_won);
    println!("money: {}", money);
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