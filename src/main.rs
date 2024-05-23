use ui::Ui;

pub mod constants;
pub mod files;
pub mod ui;
pub mod word;

fn main() {
    let mut a = Ui::new(Some("/home/mikel/Escritorio/bisekzio_metodoa.c"));
    a.display();
    a.update();
}
