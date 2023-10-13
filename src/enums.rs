fn enums() {
    enum Month {
        January,
        February,
        March,
        April,
        May,
        June,
        July,
        August,
        September,
        October,
        November,
        December,
    }
    // you can also define functions for these enumerated types.
    impl Month {
        fn is_autumn(&self) -> bool {
            match self {
                Month::September | Month::October | Month::November => true,
                _ => false,
            }
        }
    }

    let current_month: Month = Month::October;
    let month_name = match current_month {
        Month::September => ("Chill Month"),
        Month::October => ("Spooky Month"),
        Month::November => ("Almost Christmas Month"),
        Month::December => ("XMAS Month"),
        Month::January => ("New years Month"),
        Month::February => ("thaiboy love month"),
        Month::March => ("St Patricks Month"),
        Month::April => ("Birthday + Easter Month"),
        Month::May => ("Summer Month"),
        Month::June => ("Summer Month"),
        Month::July => ("Summer Month"),
        Month::August => ("Summer Month"),
    };

    println!("Is this month in autumn? {}", current_month.is_autumn());

    println!("Month Type: {}", month_name);
}