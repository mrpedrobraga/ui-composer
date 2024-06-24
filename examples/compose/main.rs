#![allow(non_snake_case)]

fn main() {
    let app = App();

    // render the app
    println!("{}", app);
}

fn App() -> String {
    FlexContainer(vec![
        Button("Click me once, shame on you".into()),
        Button("Click me twice, shame on me".into()),
        Button("Click me three times... I'm gonna MAUL YOU".into()),
    ])
}

fn FlexContainer(things: Vec<String>) -> String {
    things.iter().fold(String::new(), |mut s, a| {
        s.push_str(a.as_str());
        s.push('\n');
        s
    })
}

fn Button(label: String) -> String {
    format!("[ {} ]", label)
}
