fn main() {
    println!("Hello, world!");
    // userTest();
    // areaTesting();
    // secondAreaTesting();
    // thirdAreaTesting();

    let rect1 = Rectangle { width: 30, height: 50 };
    let rect2 = Rectangle { width: 10, height: 40 };
    let rect3 = Rectangle { width: 60, height: 45 };
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}

struct User {
    user: String,
    email: String,
    sign_in: u64,
    active: bool
}

fn userTest() {
    let user1 = User {
        user: "name".to_string(),
        email: "email@email.com".to_string(),
        sign_in: 1,
        active: true
    };
}

fn areaTesting() {
    let widthTest = 30;
    let heightTest = 50;

    println!("the area of the test is {} uints", area(widthTest, heightTest));
}
fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn secondAreaTesting() {
    
    let rect1 = (30,50);
    println!("the area of the rectangle is {}", secondArea(rect1));

}

fn secondArea(dimensions: (u32,u32)) -> u32 {
    dimensions.0 * dimensions.1
}

struct Rectangle {
    width: u32,
    height: u32,
}

fn thirdArea(rectangle: &Rectangle) -> u32 {
 rectangle.width * rectangle.height
}

fn thirdAreaTesting() {
    let rect1 = Rectangle{width: 30, height: 50};

    println!("the area of the rectangle is {}", thirdArea(&rect1));
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}
impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width * self.height > other.width * other.height
    }
}