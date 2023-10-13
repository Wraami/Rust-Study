use std::cell::RefCell;
use std::sync::{Arc, Mutex}; //mutex blocks thread that are waiting for lock to be available. //atomic reference counter, that shares the reference of the value.
pub fn main() {
    pub struct Bank {
        balance: f32,
        name: String,
    }

    // fn withdraw(the_bank: &mut Bank, amount: f32) {
    //     the_bank.balance -= amount;
    // }
    // let mut bank = Bank { balance: 100.0 };
    // withdraw(&mut bank, 5.00);
    // println!("Balance : {}", bank.balance);

    // //We want to create a bunch of customers that want monies

    // fn customer(the_bank: &mut Bank) {
    //     withdraw(the_bank, 5.00);
    // }

    // thread::spawn(|| customer(&mut bank)).join().unwrap();

    //the problem with this is that, we cant have a situation where the closure may outlive the current function and especially because we borrow, which is owned by the current function.
    //one of the fixes for this is a smart pointer, that allows multiple owners and blocks access whenever these different pointers are needed.

    fn withdraw(the_bank: &Arc<Mutex<Bank>>, amount: f32) {
        let mut bank_ref = the_bank.lock().unwrap();
        if bank_ref.balance < 5.00 {
            println!(
                "Current Balance: {} Withdrawal a smaller amount",
                bank_ref.balance
            );
        } else {
            bank_ref.balance -= amt;
            println!(
                "Customer withdrew {} Current Balance : {} ",
                amt, bank_ref.balance
            );
        }
    }
    fn customer(the_bank: &Arc<Mutex<Bank>>) {
        withdraw(&the_bank, 10.00);
    }

    let bank: &Arc<Mutex<Bank>> = Arc::new(Mutex::new(Bank { balance: 60.00 }));

    let handles = (0..10).map(|_| {
        let bank_ref = bank.clone();
        thread::spawn(|| customer(bank_ref))
    });
    for handle in handles {
        handle.join().unwrap();
    }
    println!("Total {}", bank.lock().unwrap().balance);
}
