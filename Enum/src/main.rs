

enum Countries {
    Brasil,
    Italia,
    Canada,
}

enum Coins {
    Real(Countries),
    Centavo,
}

#[derive(Debug)]
enum IpAddr {
    V4(String),
    V6(String),
}

fn main() {
    let home = IpAddr::V4(String::from("127.0.0.1"));
    
    let loopback = IpAddr::V6(String::from("::1"));

    println!("Loop back {:?}", loopback);

    let moeda = Coins::Real(Countries::Brasil);

    println!("Moeda {}", conver_moeda(moeda));
}

fn conver_moeda(coin: Coins) -> u32 {
    match coin {
        Coins::Real(countries) => {
            if let Countries::Brasil = countries {
                println!("Coin real from Brasil");
            }
            13
        }
        Coins::Centavo => 100,
    }
}
