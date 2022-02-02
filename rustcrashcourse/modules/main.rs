mod house{
    pub fn bedroom() {
        println!("Bedroom");
    }

    pub fn bathroom() {
        println!("Bathroom");
    }

    pub fn kitchen() {
        println!("Kitchen");
    }
}

fn main() {
    house::bedroom();
    house::bathroom();
    house::kitchen();
}
