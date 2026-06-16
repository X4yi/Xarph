use gtk4::glib;
fn main() {
    let (tx, rx) = glib::MainContext::channel(glib::Priority::DEFAULT);
}
