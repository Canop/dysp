use {
    gloo::{
        console::*,
        utils::*,
    },
    web_sys::*,
    wasm_bindgen::{
        prelude::*,
        JsCast,
    },
};

pub trait DomElement {
    fn with_class(self, class: &str) -> Self;
}
impl DomElement for HtmlDivElement {
    fn with_class(self, class: &str) -> Self {
        self.class_list().add_1(class);
        self
    }
}

pub trait Append {
    fn append<E: AsRef<Node>>(&self, child: E);
}

impl Append for HtmlDivElement {
    fn append<E: AsRef<Node>>(&self, child: E) {
        self.append_child(child.as_ref());
    }
}

pub fn div() -> HtmlDivElement {
    document()
        .create_element("div")
        .unwrap()
        .dyn_into::<HtmlDivElement>()
        .unwrap()
}
