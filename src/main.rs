use ui::Ui;

pub mod constants;
pub mod files;
pub mod ui;
pub mod word;

fn main() {
    let mut a = Ui::new(None);
    a.display();
    a.update();
}
