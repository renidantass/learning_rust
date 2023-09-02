struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, rect: &Rectangle) -> bool {
         rect.width <= self.width && rect.height <= self.height
    }

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size
        }
    }
}

struct Person(String, u8);

struct AlwaysEqual;

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

fn main() {
    let mut user1: User = build_user(
                                    String::from("renidantas0@gmail.com"), 
                                    String::from("renidantass")
    );

    let user2: User = User {
        active: false,
        ..user1
    };

    let p1: Person = Person(String::from("Reni"), 23);

    println!("{}", p1.1);

    println!("{}", user2.email);

    let subject = AlwaysEqual;

    let rect1 = Rectangle {
        width: 30,
        height: 50
    };

    let rect2 = Rectangle {
        width: 10,
        height: 40
    };

    let rect3 = Rectangle {
        width: 60,
        height: 45
    };

    println!("rect1 is {:?}", rect1);
    println!("pretty print rect1 is {:#?}", rect1);

    println!("The area of rect is {}", rect1.area());

    println!("Can rect1 hold rect2? {:?}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {:?}", rect1.can_hold(&rect3));

    let square = Rectangle::square(32);

    println!("{:?}", square);
}