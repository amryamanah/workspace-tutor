pub fn main() {
    println!("Chapter 17");
    chap17_1();
    chap17_2();
}

fn chap17_2() {
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No"),
                ],
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("OK"),
            }),
        ]
    };

    screen.run();
}

pub trait Draw {
    fn draw(&self);
}

//pub struct Screen<T: Draw> {
//    pub components: Vec<T>,
//}
//
//impl<T> Screen<T>
//    where T: Draw {
//    pub fn run(&self) {
//        for component in self.components.iter() {
//            component.draw();
//        }
//    }
//}

pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        unimplemented!()
    }
}

pub struct SelectBox {
    pub width: u32,
    pub height: u32,
    pub options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        unimplemented!()
    }
}

fn chap17_1() {
    let mut a = AveragedCollection {
        list: vec![],
        average: 0.0,
    };

    a.add(10);
    println!("average: {}", a.average);

    a.add(5);
    println!("average: {}", a.average);
}

#[derive(Debug)]
pub struct AveragedCollection {
    list: Vec<i32>,
    average: f64,
}

impl AveragedCollection {
    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average();
    }

    pub fn remove(&mut self) -> Option<i32> {
        let result = self.list.pop();
        match result {
            Some(value) => {
                self.update_average();
                Some(value)
            }
            None => None,
        }
    }

    pub fn average(&self) -> f64 {
        self.average
    }

    fn update_average(&mut self) {
        let total: i32 = self.list.iter().sum();
        self.average = total as f64 / self.list.len() as f64;
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
