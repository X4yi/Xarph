use super::ShellWidget;
use crate::start_menu::StartMenu;
use gtk4::prelude::*;
use gtk4::{Button, Label, Widget};
use std::cell::RefCell;

pub struct StartButtonWidget;

impl ShellWidget for StartButtonWidget {
    fn build(&self) -> Widget {
        let icon = Label::new(Some(""));
        icon.set_markup("<span size='16000'>\u{f0c9}</span>");

        let btn = Button::builder()
            .child(&icon)
            .tooltip_text("Menú Inicio")
            .css_classes(vec!["start-btn".to_string(), "flat".to_string()])
            .build();

        let menu: RefCell<Option<StartMenu>> = RefCell::new(None);
        btn.connect_clicked(move |btn| {
            let mut m = menu.borrow_mut();
            if m.is_none() {
                *m = Some(StartMenu::new(btn));
            }
            if let Some(ref start_menu) = *m {
                start_menu.toggle();
            }
        });

        btn.upcast()
    }
}
