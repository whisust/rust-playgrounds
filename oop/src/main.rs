pub trait Draw {
    fn draw(&self);
}

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

#[derive(Debug)]
pub struct Button {
    width: u32,
    height: u32,
    label: String,
}

impl Draw for Button {
    fn draw(&self) {
        println!("{:?}", self);
    }
}


#[derive(Debug)]
pub struct TextField {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for TextField {
    fn draw(&self) {
        println!("{:?}", self);
    }
}

fn main() {
    let components: Vec<Box<dyn Draw>> = vec![
        Box::new(Button { width: 10, height: 10, label: "coucou".to_string() }),
        Box::new(TextField {width: 1, height: 1, options: vec!["this is a label".to_string()] })
    ];
    let screen = Screen {
        components,
    };

    screen.run();
}
