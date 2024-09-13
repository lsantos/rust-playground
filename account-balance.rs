struct Account {
    balance: i32
}

fn main() {
    let accounts: Vec<Account> = vec![
        Account { balance: 0 },
        Account { balance: 10 }
    ];
    
    let balances = accounts
        .iter()
        .map(|account| account.balance)
        .collect::<Vec<_>>();
        
    println!("Balances: {:#?}", balances);
}
