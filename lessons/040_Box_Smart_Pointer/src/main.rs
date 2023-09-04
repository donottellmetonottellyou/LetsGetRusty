#![allow(unused)]

trait UIComponent {
    fn render(&self) {
        println!("Rendering component...");
    }
}

struct Button {
    text: String,
}
impl UIComponent for Button {}

struct Container {
    name: String,
    child: Option<Box<Container>>,
}
impl UIComponent for Container {}

fn main() {
    let button_a = Button {
        text: "button a".into(),
    };
    let button_b = Box::new(Button {
        text: "button b".into(),
    });

    let button_c = button_a;
    let button_d = button_b;

    let components: Vec<Box<dyn UIComponent>> = vec![Box::new(button_c), button_d];
}
