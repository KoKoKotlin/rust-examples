#[derive(Debug)]
struct Rect {
    width: u32,
    height: u32
}

fn main() {
    
    let rect = Rect {
        width: 30,
        height: 20,
    };

    let rect2 = Rect { 
        width: 15, 
        height: 10,
    };

    let rect3 = Rect {
        width: 20,
        height: 60,
    };
    
    println!("Width: {}, Height: {}, Area: {}", rect.width, rect.height, calc_area(&rect));
    println!("{:?}", rect);

    println!("Area of rect with method: {}", rect.area());

    println!("Rect can hold rect2: {}", rect.can_hold(&rect2));
    println!("Rect can hold rect3: {}", rect.can_hold(&rect3));

    let square = Rect::square(10);
    println!("{:?}", square);
    
}

// outside function
fn calc_area(rect: &Rect) -> u32 {
    rect.width * rect.height
}

// method
impl Rect {
    fn area(&self) -> u32 { // borrow self immutably
        self.width * self.height
    }

    fn can_hold(&self, other: &Rect) -> bool {
        self.width > other.width && self.height > other.height
    }
    
    fn square(size: u32) -> Rect {  // associated function because no self argument
        Rect { 
            width: size,
            height: size,
        }
    }
}