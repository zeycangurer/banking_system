trait Account {
    fn deposit(&mut self, amount: f64);
    fn withdraw(&mut self, amount: f64);
    fn balance(&self) -> f64;
}

struct BankAccount {
    account_number: u32,
    holder_name: String,
    balance: f64,
}

impl Account for BankAccount {
    fn deposit(&mut self, amount: f64) {
        self.balance += amount;
    }

    fn withdraw(&mut self, amount: f64) {
        self.balance -= amount;
    }

    fn balance(&self) -> f64 {
        self.balance
    }
}

fn main() {
    let mut account1 = BankAccount {
        account_number: 12345,
        holder_name: String::from("Zeycan1"),
        balance: 1000.0,
    };

    let mut account2 = BankAccount {
        account_number: 67890,
        holder_name: String::from("Zeycan2"),
        balance: 2000.0,
    };

    account1.deposit(500.0);
    account2.withdraw(300.0);

    println!("{} adlı hesapta kalan bakiye: {}", account1.holder_name, account1.balance());
    println!("{} adlı hesapta kalan bakiye: {}", account2.holder_name, account2.balance());
}
