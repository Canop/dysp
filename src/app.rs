use crate::*;

pub struct App {
    editor: Editor,
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
        log!("new code: {:?}", &code);
        Ok(())
    }
}
