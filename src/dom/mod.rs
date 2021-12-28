use {
    gloo::{
        utils::*,
    },
    web_sys::*,
    wasm_bindgen::{
        JsCast,
    },
};

pub trait DomElement {
    fn with_class(self, class: &str) -> Self;
}
impl DomElement for HtmlDivElement {
    fn with_class(self, class: &str) -> Self {
        self.class_list().add_1(class).unwrap();
        self
    }
}

pub trait Append {
    fn append<E: AsRef<Node>>(&self, child: E);
}
pub trait Clear {
    fn clear(&self);
}
pub trait Attrs {
    fn set_attr<S: AsRef<str>>(&self, name: &str, value: S);
}

macro_rules! html_tag {
    ($tag: ident, $type: ty) => {
        impl Clear for $type {
            fn clear(&self) {
                while let Some(child) = self.first_element_child() {
                    child.remove();
                }
            }
        }
        impl Append for $type {
            fn append<E: AsRef<Node>>(&self, child: E) {
                self.append_child(child.as_ref()).unwrap();
            }
        }
        impl Attrs for $type {
            fn set_attr<S: AsRef<str>>(&self, name: &str, value: S) {
                self.set_attribute(name, value.as_ref()).unwrap();
            }
        }
        pub fn $tag() -> $type {
            document()
                .create_element(stringify!($tag))
                .unwrap()
                .dyn_into::<$type>()
                .unwrap()
        }
    }
}
macro_rules! svg_tag {
    ($tag: ident, $type: ty) => {
        impl Append for $type {
            fn append<E: AsRef<Node>>(&self, child: E) {
                self.append_child(child.as_ref()).unwrap();
            }
        }
        impl Attrs for $type {
            fn set_attr<S: AsRef<str>>(&self, name: &str, value: S) {
                self.set_attribute(name, value.as_ref()).unwrap();
            }
        }
        pub fn $tag() -> $type {
            document()
                .create_element_ns(Some("http://www.w3.org/2000/svg"), stringify!($tag))
                .unwrap()
                .dyn_into::<$type>()
                .unwrap()
        }
    }
}

html_tag!(div, HtmlDivElement);

svg_tag!(svg, SvgElement);
svg_tag!(line, SvgLineElement);
svg_tag!(polyline, SvgPolylineElement);
