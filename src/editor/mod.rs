use crate::*;

pub struct Editor {
    code_editor: HtmlDivElement,
    code_change_listener: EventListener,
    canvas_click_listener: EventListener,
}

impl Editor {
    pub fn new(
        parent: &mut HtmlElement,
    ) -> Result<Self, JsValue> {
        log!("starting editor");
        let editor = dom::div().with_class("editor");
        parent.append_child(editor.as_ref());

        let code_editor = dom::div().with_class("code-editor");
        code_editor.set_inner_text("bla bla");
        code_editor.set_attribute("spellcheck", "false");
        code_editor.set_attribute("contenteditable", "true");
        let code_change_listener = EventListener::new(&code_editor, "input", move |_event| {
            log!("code changed");
            app().on_code_change();
        });
        editor.append(&code_editor);

        let player = dom::div().with_class("player");
        let canvas = dom::div().with_class("canvas");
        let canvas_click_listener = EventListener::new(&canvas, "click", move |_event| {
            log!("click in canvas");
        });
        player.append(canvas);
        editor.append(player);

        Ok(Self {
            code_editor,
            code_change_listener,
            canvas_click_listener,
        })
    }
    pub fn get_code(&self) -> String {
        self.code_editor.inner_text()
    }
}
