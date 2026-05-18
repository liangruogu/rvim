mod editor;
use editor::Editor;
fn main() {
    let mut editor = Editor::new();
    let _ = editor.run();
}
