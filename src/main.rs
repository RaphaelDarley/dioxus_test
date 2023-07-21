use dioxus::prelude::*;

fn main() {
    dioxus_web::launch(app);
}

fn app(cx: Scope) -> Element {
    let test_state = use_state(cx, || String::from(""));
    cx.render(rsx! {
        h1 { "Test Form" }
        p { "{test_state}" } // this line causes an error at runtime
    })
}
