use chrono::{DateTime, NaiveDate, Utc};

pub fn structs() {
    struct Musician {
        name: String,
        description: String,
        main_genre: String,
        last_release: DateTime<Utc>,
        most_popular_record: String,
    }
    let release_date = NaiveDate::from_ymd(2023, 10, 11);

    let last_release = DateTime::<Utc>::from_utc(release_date.and_hms_opt(0, 0, 0).unwrap(), Utc);

    let mut grimes = Musician {
        name: String::from("Grimes"),
        description: String::from(
            "Artsy indie girl recently known for her previous marriage to elon musk.",
        ),
        main_genre: String::from("Art Pop"),
        last_release,
        most_popular_record: String::from("Art Angels"),
    };

    grimes.description =String::from("eccentric pop artist with a grammy winning history and critical acclaim for her influence on the 2010s pop scene");

    println!("{}", grimes.name);
    println!("{}", grimes.description);
    println!("{}", grimes.main_genre);
    println!("{}", grimes.last_release);
    println!("{}", grimes.most_popular_record);
}

pub fn generic_structs() {
    const PI: f32 = 3.141592;
    // struct Rectangle<T, U> {
    //     length: T,
    //     height: U,
    // }
    // let rec = Rectangle {
    //     length: 4,
    //     height: 10.5,
    // };
    trait Shape {
        fn new(length: f32, width: f32) -> Self;
        fn area(&self) -> f32;
    }

    struct Rectangle {
        length: f32,
        width: f32,
    };
    struct Circle {
        length: f32,
        width: f32,
    };
    struct Pentagon {
        length: f32,
        width: f32,
    };
    struct Square {
        length: f32,
        width: f32,
    };

    impl Shape for Rectangle {
        fn new(length: f32, width: f32) -> Rectangle {
            return Rectangle { length, width };
        }
        fn area(&self) -> f32 {
            return self.length * self.width;
        }
    }

    impl Shape for Circle {
        fn new(length: f32, width: f32) -> Circle {
            return Circle { length, width };
        }

        fn area(&self) -> f32 {
            return (self.length / 2.0).powf(2.0) * PI;
        }
    }

    impl Shape for Pentagon {
        fn new(length: f32, width: f32) -> Pentagon {
            return Pentagon { length, width };
        }
        fn area(&self) -> f32 {
            return (self.length / 2.0).powf(2.0) * PI;
        }
    }

    impl Shape for Square {
        fn new(length: f32, width: f32) -> Square {
            return Square { length, width };
        }
        fn area(&self) -> f32 {
            return self.length * self.width;
        }
    }

    let rec: Rectangle = Shape::new(10.0, 10.0);
    let circle: Circle = Shape::new(24.0, 24.0);
    let pent: Pentagon = Shape::new(20.0, 20.0);
    let square: Square = Shape::new(5.0, 5.0);

    println!("Rectangle Area: {}", rec.area());
    println!("Circle Area : {}", circle.area());
    println!("Pentagon Area : {}", pent.area());
    println!("Square Area : {}", square.area());
}
