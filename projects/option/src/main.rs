#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    //...
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(Option<UsState>),
}

fn value_in_cents(coin: &Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            // match state {
            //     Some(state) => {
            //         println!("A Quarter from {:?}", state);
            //     },
            //     None => {
            //         println!("A generic Quarter");
            //     },
            // }
            if let Some(state) = state {
                println!("A Quarter from {:?}", state);
            }
            else {
                println!("A generic Quarter");
            }
            25
        },
    }
}

fn main() {
    let coin1 = Coin::Quarter(Some(UsState::Alabama));
    let coin2 = Coin::Quarter(None);
    println!("{}", value_in_cents(&coin1));
    println!("{}", value_in_cents(&coin2));

    //let config_max = Some(3u8);
    // let config_max: Option<u8> = None;
    // if let Some(max) = config_max {
    //     println!("The maximum is condigured to be {max}");
    // }
}