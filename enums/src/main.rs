enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn main() {
    let c1 = Coin::Penny;

    println!("Value of c1: {}", value_in_cents(c1));

    let five = Some(5);
    let six = plus_one(five);

    println!("Value of six: {:?}", six);

    let some_u8_value = Some(0u8);
    if let Some(3u8) = some_u8_value {
        println!("three");
    } else {
        println!("Not three");
    }
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
