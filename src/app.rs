use crate::*;

pub struct App {
    pub editor: Editor,
}

impl Drop for App {
    fn drop(&mut self) {
        log!("app dropped");
    }
}

impl App {
    pub fn new(parent: &mut HtmlElement) -> Result<Self, JsValue> {
        log!("starting app");
        let editor = Editor::new(parent)?;
        Ok(Self {
            editor,
        })
    }
    pub fn on_code_change(&mut self) -> Result<(), JsValue> {
        let code = self.editor.get_code();
        match code.parse::<Program>() {
            Ok(program) => {
                log!("program ok");
                let svg = program.compile();
                self.editor.display_svg(svg);
                self.editor.set_error(None);
            }
            Err(e) => {
                warn!("error in program:", e.to_string());
                self.editor.set_error(Some(e.to_string()));
            }
        }
        Ok(())
    }
}
