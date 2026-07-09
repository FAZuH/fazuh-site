use dioxus::prelude::*;

use crate::components::*;

#[component]
pub fn App() -> Element {
    rsx! {
        Head {}
        Header {}
        main {
            Hero {}
            About {}
            Skills {}
            Projects {}
            Contact {}
        }
        Footer {}
    }
}
