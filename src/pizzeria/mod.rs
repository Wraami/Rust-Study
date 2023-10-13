mod order {
    use rand::Rng;
    use std::io;
    use std::io::{BufRead, BufReader, ErrorKind, Write};

    pub struct Pizza {
        pub dough: String,
        pub cheese: String,
        pub topping: String,
        pub size: String,
        pub extra_crust_topping: String,
    }

    impl Pizza {
        pub fn lunch(topping: &str, extra_crust_topping: &str, size: &str) -> Pizza {
            Pizza {
                dough: String::from("neopolitan dough"),
                cheese: String::from("mozzarella"),
                topping: String::from(topping),
                size: String::from(size),
                extra_crust_topping: String::from(extra_crust_topping),
            }
        }
    }

    pub struct Drink {
        pub drink_type: String,
        pub size: String,
    }
    impl Drink {
        pub fn side_order(drink_type: &str, size: &str) -> Drink {
            Drink {
                drink_type: String::from(drink_type),
                size: String::from(size),
            }
        }
    }

    pub mod help_customer {
        use rand::Rng;

        pub fn seat_customer() {
            println!("Greet the customer to Lio's Pizza Spot");
            let reservation_status =
                super::get_user_input("Does customer have a reservation?").to_lowercase();
            if reservation_status == "yes" {
                println!("The customer has a reservation.");
                println!("guide them to their allocated seat!")
            } else if reservation_status == "no" {
                println!("The customer does not have a reservation.");
                println!("but that's ok, we have spare tables anyways :)")
            } else {
                println!("Invalid input. Please enter 'yes' or 'no'.");
                seat_customer();
            }
            take_order();
        }

        pub fn take_order() {
            let topping = super::get_user_input("Enter their pizza topping(s): ");
            let size = super::get_user_input("Enter their desired pizza size: ");
            let extra_crust_topping = super::get_user_input(
                "Enter extra crust topping (or press Enter for regular crust): ",
            );

            let cust_pizza: super::Pizza = if extra_crust_topping.is_empty() {
                super::Pizza::lunch(&topping, "regular", &size)
            } else {
                super::Pizza::lunch(&topping, &extra_crust_topping, &size)
            };

            let drink_needed =
                super::get_user_input("Would the customer like a drink?").to_lowercase();
            if drink_needed == "yes" {
                let drink_type = super::get_user_input("What drink would they like?: ");
                let size = super::get_user_input("Enter the size they'd like: ");
                let cust_drink = super::Drink::side_order(&drink_type, &size);
                takeaway_or_dine_in();
                serve_customer(Some(cust_pizza), Some(cust_drink));
            } else if drink_needed == "no" {
                takeaway_or_dine_in();
                serve_customer(Some(cust_pizza), None);
            } else {
                println!("Invalid input. Please enter 'yes' or 'no'.");
                take_order();
            }

            // let cust_drink: super::Drink = { super::Drink::side_order(&drink_type, &size) };
            // serve_customer(cust_drink);
        }

        fn takeaway_or_dine_in() {
            let seating_plan_for_customers =
                super::get_user_input("Would the customer like their order to go or to eat in?")
                    .to_lowercase();
            if seating_plan_for_customers == "to go" {
                println!(
                    "Ok, their order will be prepared and bagged to collect to-go after payment"
                );
            } else if seating_plan_for_customers == "eat in" {
            } else {
                println!("Invalid input. Please enter 'to go' or 'eat in'");
                takeaway_or_dine_in();
            }
            process_customer_payment_method();
        }

        fn process_customer_payment_method() {
            let payment_type =
                super::get_user_input("would the customer like to pay with cash or card?")
                    .to_lowercase();
            if payment_type == "card" {
                println!("tell the customer to present card and press to card machine");
            } else if payment_type == "cash" {
                println!("count cash for customer");
            } else {
                println!(
                    "Invalid input. We will never accept leetcoin! Please enter 'card' or 'cash'"
                );
                process_customer_payment_method();
            }
            process_customer_payment();
        }

        fn process_customer_payment() {
            let random_number = rand::thread_rng().gen_range(0..100); //Doing decline logic with random number for funnies xP

            if random_number <= 5 {
                println!("payment declined, please try again");
                process_customer_payment_method();
            } else {
                println!("Payment successful :) Thank you!");
                println!("Tell the customer their food will be with them very shortly :)");
            }
        }

        fn serve_customer(pizza: Option<super::Pizza>, drink: Option<super::Drink>) {
            if let Some(pizza) = pizza {
                println!(
                    "The customer is served a {} Pizza with {} and a {} crust",
                    pizza.size, pizza.topping, pizza.extra_crust_topping
                );
            }
            if let Some(drink) = drink {
                println!(
                    "The customer is served a {} {}",
                    drink.size, drink.drink_type
                );
            }
            customer_goodbye();
        }

        fn customer_goodbye() {
            println!(
                "Thank the customer for dining at Lio's Pizzeria, tell them to come again soon!"
            );
        }
    }

    pub fn get_user_input(prompt: &str) -> String {
        println!("{}", prompt);
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");
        input.trim().to_string()
    }
}

pub fn order_food() {
    crate::pizzeria::order::help_customer::seat_customer();
}
