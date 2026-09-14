// ==========================================
// Lab Session 3: Structs, Enums, & Methods
// ==========================================

#[derive(Debug, PartialEq, Clone)]
enum StudentStatus {
    Active,
    Graduated,
    OnLeave,
    Suspended,
}

#[derive(Debug, Clone)]
struct Student {
    id: u32,
    name: String,
    status: StudentStatus,
    gpa: f64,
}

impl Student {
    fn new(id: u32, name: &str, gpa: f64) -> Self {
        Student {
            id,
            name: String::from(name),
            status: StudentStatus::Active,
            gpa,
        }
    }

    fn update_gpa(&mut self, new_gpa: f64) {
        if (0.0..=4.0).contains(&new_gpa) {
            self.gpa = new_gpa;
        } else {
            println!("Invalid GPA: {}. Must be between 0.0 and 4.0", new_gpa);
        }
    }

    fn set_status(&mut self, new_status: StudentStatus) {
        self.status = new_status;
    }

    fn is_eligible_for_honors(&self) -> bool {
        self.status == StudentStatus::Active && self.gpa >= 3.5
    }
}

// Transaction Enum for Bank Account
#[derive(Debug)]
enum Transaction {
    Deposit(f64),
    Withdrawal(f64),
    Interest(f64),
}

// BankAccount Struct
struct BankAccount {
    account_number: String,
    balance: f64,
    history: Vec<Transaction>,
}

impl BankAccount {
    fn new(account_number: &str, initial_balance: f64) -> Self {
        BankAccount {
            account_number: String::from(account_number),
            balance: initial_balance,
            history: vec![],
        }
    }

    fn deposit(&mut self, amount: f64) {
        if amount > 0.0 {
            self.balance += amount;
            self.history.push(Transaction::Deposit(amount));
        }
    }

    fn withdraw(&mut self, amount: f64) -> Result<f64, String> {
        if amount <= 0.0 {
            Err(String::from("Withdrawal amount must be positive"))
        } else if amount > self.balance {
            Err(String::from("Insufficient funds"))
        } else {
            self.balance -= amount;
            self.history.push(Transaction::Withdrawal(amount));
            Ok(self.balance)
        }
    }

    fn apply_interest(&mut self, rate_percent: f64) {
        let interest = self.balance * (rate_percent / 100.0);
        self.balance += interest;
        self.history.push(Transaction::Interest(interest));
    }

    fn balance(&self) -> f64 {
        self.balance
    }

    fn statement(&self) {
        println!("=== Statement for Account {} ===", self.account_number);
        for tx in &self.history {
            match tx {
                Transaction::Deposit(amt) => println!("  Deposit:    +${:.2}", amt),
                Transaction::Withdrawal(amt) => println!("  Withdrawal: -${:.2}", amt),
                Transaction::Interest(amt) => println!("  Interest:   +${:.2}", amt),
            }
        }
        println!("  Current Balance: ${:.2}\n", self.balance);
    }
}

fn main() {
    println!("=== Student Management System ===");
    let mut alice = Student::new(101, "Alice Smith", 3.8);
    let mut bob = Student::new(102, "Bob Jones", 3.2);

    println!("Alice honors eligible? {}", alice.is_eligible_for_honors());

    alice.update_gpa(3.4);
    println!("Alice new GPA: {}, honors eligible? {}", alice.gpa, alice.is_eligible_for_honors());

    bob.set_status(StudentStatus::Graduated);
    bob.update_gpa(3.9);
    println!("Bob honors eligible? {} (Status: {:?})\n", bob.is_eligible_for_honors(), bob.status);

    println!("=== Bank Account System ===");
    let mut acc = BankAccount::new("ACC-1001", 500.0);
    acc.deposit(200.0);

    match acc.withdraw(150.0) {
        Ok(bal) => println!("Withdrawal successful. New balance: ${:.2}", bal),
        Err(msg) => println!("Error: {}", msg),
    }

    match acc.withdraw(1000.0) {
        Ok(bal) => println!("Withdrawal successful. New balance: ${:.2}", bal),
        Err(msg) => println!("Error: {}", msg),
    }

    acc.apply_interest(2.5);
    acc.statement();
    println!("Final balance: ${:.2}\n", acc.balance());
}