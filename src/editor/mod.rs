use {
    crate::*,
    dom::{Append, Attrs, Clear},
};

pub struct Editor {
    code_editor: HtmlDivElement,
    canvas: HtmlDivElement,
    error: HtmlDivElement,
    coords: HtmlDivElement,
    #[allow(dead_code)]
    handlers: Vec<EventListener>,
}

static CODE: &str = r#"
move to 25 33
color red
line to 40 21
line to 20 80
color blue
line to 10 10
move to 20 20
line to 80 20
"#;

impl Editor {
    pub fn new(
        parent: &mut HtmlElement,
    ) -> Result<Self, JsValue> {
        log!("starting editor");
        let mut handlers = Vec::new();
        let editor = dom::div().with_class("editor");
        parent.append_child(editor.as_ref()).unwrap();

        let code_editor = dom::div().with_class("code-editor");
        code_editor.set_inner_text(CODE);
        code_editor.set_attr("spellcheck", "false");
        code_editor.set_attr("contenteditable", "true");
        handlers.push(EventListener::new(&code_editor, "input", move |_event| {
            log!("code changed");
            app().on_code_change().unwrap();
        }));
        editor.append(&code_editor);

        let player = dom::div().with_class("player");
        let canvas = dom::div().with_class("canvas");
        handlers.push(EventListener::new(&canvas, "mousemove", move |event| {
            let app = app();
            let rect = app.editor.canvas.get_bounding_client_rect();
            let mouse_event: &MouseEvent = event.dyn_ref::<MouseEvent>().unwrap();
            let x = 100.0 * (mouse_event.client_x() as f32 - rect.left() as f32) / (rect.width() as f32);
            let y = 100.0 * (mouse_event.client_y() as f32 - rect.top() as f32) / (rect.height() as f32);
            let coords = format!("{} {}", x.round(), y.round());
            app.editor.coords.set_inner_text(&coords);
        }));
        handlers.push(EventListener::new(&canvas, "mouseleave", move |_event| {
            app().editor.coords.set_inner_text("");
        }));
        player.append(&canvas);
        editor.append(&player);

        let infos = dom::div().with_class("infos");
        player.append(&infos);

        let error = dom::div().with_class("error");
        infos.append(&error);
        let coords = dom::div().with_class("coords");
        infos.append(&coords);

        Ok(Self {
            code_editor,
            canvas,
            error,
            coords,
            handlers,
        })
    }
    pub fn get_code(&self) -> String {
        self.code_editor.inner_text()
    }
    pub fn display_svg(&self, svg: SvgElement) {
        self.canvas.clear();
        self.canvas.append(svg);
    }
    pub fn set_error(&self, error: Option<String>) {
        match error {
            Some(s) => self.error.set_inner_text(&s),
            None => self.error.set_inner_text(""),
        }
    }
}
