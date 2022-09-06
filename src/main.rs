use chrono::prelude::*;

fn main() {
    println!("Hello, world!");
}

trait AccountService {
    fn deposit(&mut self, amount: u32);
    fn withdraw(&mut self, amount: u32);
    fn print(&self) -> String;
}

trait Repository {
    fn add(&self, account: Account);
    fn retrieve(&self);
}

struct Transaction {
    date: Date<Utc>,
    amount: u32,
    balance: u32,
}

impl Transaction {
    fn new(amount: u32, balance: u32) -> Transaction {
        Transaction {
            date: Utc.ymd(2022, 8, 30),
            amount,
            balance
        }
    }
}

struct Account {
    transactions: Vec<Transaction>,
}

impl Account {
    fn new() -> Account {
        Account {
            transactions: vec![],
        }
    }

    fn balance(&self) -> u32 {
        self.transactions.last().map(|t| t.balance).unwrap_or(0)
    }
}

impl AccountService for Account {
    fn deposit(&mut self, amount: u32) {
        self.transactions.push(Transaction::new(amount, self.balance() + amount));
    }

    fn withdraw(&mut self, amount: u32) {
        todo!()
    }

    fn print(&self) -> String {
        let mut statement = "Date || Amount || Balance\n".to_string();
        let mut balance = 0;
        for entry in self.transactions.iter() {
            let date = entry.date.format("%Y-%m-%d");
            let amount = entry.amount;
            balance += amount;
            statement.push_str(format!("{date} || {amount} || {balance}\n").as_ref());
        }
        statement
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn call_print_function_no_actions() {
        let account = Account::new();

        let result = account.print();

        assert_eq!(result, "Date || Amount || Balance\n");
    }

    #[test]
    fn call_print_function_adding_deposit() {
        let mut account = Account::new();

        account.deposit(20);
        let result = account.print();

        assert_eq!(
            result,
            "Date || Amount || Balance\n\
             2022-08-30 || 20 || 20\n"
        );
    }

    #[test]
    fn call_print_function_adding_deposit_twice() {
        let mut account = Account::new();

        account.deposit(20);
        account.deposit(30);
        let result = account.print();

        assert_eq!(
            result,
            "Date || Amount || Balance\n\
             2022-08-30 || 20 || 20\n\
             2022-08-30 || 30 || 50\n"
        );
    }

    #[test]
    fn given_no_transactions_balance_returns_0() {
        let mut account = Account::new();

        let result = account.balance();

        assert_eq!(result,0);
    }
}
