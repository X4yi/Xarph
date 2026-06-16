use std::cell::RefCell;
use std::rc::Rc;

pub mod objects;
pub mod wallpaper;

use gtk4::gdk;
use gtk4::prelude::*;
use gtk4::{
    Align, Application, Box as GtkBox, Button, Fixed, GestureClick, Label, Orientation, Popover,
    Separator, glib,
};
use xarph_sdk::config::XarphConfig;
use xarph_sdk::socket::Socket;

use self::objects::DesktopObject;
use self::objects::ObjectData;
use self::objects::WidgetObject;
use self::objects::drag;
use self::objects::persistence::ObjectPersistence;
use self::wallpaper::WallpaperRenderer;

pub struct Desktop {
    renderer: WallpaperRenderer,
    objects: Rc<RefCell<Vec<std::boxed::Box<dyn DesktopObject>>>>,
    object_container: Fixed,
    widget_container: Fixed,
    persistence: ObjectPersistence,
    current_workspace: u8,
}

impl Desktop {
    pub fn new(app: &Application, use_layer_shell: bool) -> Self {
        let renderer = WallpaperRenderer::new(app, use_layer_shell);

        // Layer 1: Desktop Widgets (below objects)
        let widget_container = Fixed::new();
        widget_container.set_hexpand(true);
        widget_container.set_vexpand(true);
        renderer.add_overlay_widget(&widget_container);

        // Layer 2: Desktop Objects (above widgets)
        let object_container = Fixed::new();
        object_container.set_hexpand(true);
        object_container.set_vexpand(true);
        renderer.add_overlay_widget(&object_container);

        let persistence = ObjectPersistence::new();
        let objects_data = persistence.load();
        let objects = ObjectPersistence::deserialize_all(objects_data);

        let objects = Rc::new(RefCell::new(objects));

        let mut desktop = Self {
            renderer,
            objects,
            object_container,
            widget_container,
            persistence,
            current_workspace: 1,
        };

        if desktop.objects.borrow().is_empty() {
            desktop.add_default_widgets();
        }

        let config = XarphConfig::load();
        let wallpaper = config.get_wallpaper_for_workspace(desktop.current_workspace);
        desktop.renderer.set_config(wallpaper.clone());

        desktop.build_widgets();
        desktop.build_object_widgets();
        desktop.attach_desktop_context_menu();
        desktop.spawn_workspace_watcher();
        desktop.spawn_config_watcher();
        desktop
    }

    pub fn present(&self) {
        self.renderer.present();
    }

    fn build_widgets(&self) {
        while let Some(child) = self.widget_container.first_child() {
            self.widget_container.remove(&child);
        }

        let objects = self.objects.clone();
        let persistence = self.persistence.clone();
        for obj in self.objects.borrow().iter() {
            if obj.object_type() == "widget" {
                let widget = obj.build();
                let obj_id = obj.id().to_string();
                let objects = objects.clone();
                let persistence = persistence.clone();
                drag::attach_drag_move(
                    &widget,
                    Some(Box::new(move |x, y| {
                        for o in objects.borrow_mut().iter_mut() {
                            if o.id() == obj_id {
                                o.set_position(x, y);
                                break;
                            }
                        }
                        let data: Vec<ObjectData> =
                            objects.borrow().iter().map(|o| o.data()).collect();
                        let _ = persistence.save(&data);
                    })),
                );
                self.widget_container
                    .put(&widget, obj.data().x(), obj.data().y());
            }
        }

        self.widget_container.set_visible(true);
    }

    fn build_object_widgets(&self) {
        while let Some(child) = self.object_container.first_child() {
            self.object_container.remove(&child);
        }

        let objects = self.objects.clone();
        let persistence = self.persistence.clone();
        for obj in self.objects.borrow().iter() {
            if obj.object_type() != "widget" {
                let widget = obj.build();
                let obj_id = obj.id().to_string();
                let objects = objects.clone();
                let persistence = persistence.clone();
                drag::attach_drag_move(
                    &widget,
                    Some(Box::new(move |x, y| {
                        for o in objects.borrow_mut().iter_mut() {
                            if o.id() == obj_id {
                                o.set_position(x, y);
                                break;
                            }
                        }
                        let data: Vec<ObjectData> =
                            objects.borrow().iter().map(|o| o.data()).collect();
                        let _ = persistence.save(&data);
                    })),
                );
                self.object_container
                    .put(&widget, obj.data().x(), obj.data().y());
            }
        }

        self.object_container.set_visible(true);
    }

    fn attach_desktop_context_menu(&self) {
        let rclick = GestureClick::new();
        rclick.set_button(3);

        let container = self.object_container.clone();
        rclick.connect_pressed(move |_, _, x, y| {
            let popover = Popover::new();
            popover.set_autohide(true);
            popover.add_css_class("context-menu");

            let vbox = GtkBox::new(Orientation::Vertical, 0);
            vbox.add_css_class("menu-content");

            let title = Label::builder()
                .label("Desktop")
                .css_classes(["menu-info"])
                .xalign(0.0)
                .build();
            vbox.append(&title);
            vbox.append(&Separator::new(Orientation::Horizontal));

            let new_folder_btn = Button::builder()
                .label("New Folder")
                .css_classes(["menu-item"])
                .halign(Align::Fill)
                .build();
            vbox.append(&new_folder_btn);

            vbox.append(&Separator::new(Orientation::Horizontal));

            let change_wallpaper_btn = Button::builder()
                .label("Change Wallpaper…")
                .css_classes(["menu-item"])
                .halign(Align::Fill)
                .build();
            vbox.append(&change_wallpaper_btn);

            let settings_btn = Button::builder()
                .label("Desktop Settings…")
                .css_classes(["menu-item"])
                .halign(Align::Fill)
                .build();
            vbox.append(&settings_btn);

            popover.set_child(Some(&vbox));
            popover.set_parent(&container);
            popover.set_pointing_to(Some(&gdk::Rectangle::new(x as i32, y as i32, 1, 1)));
            popover.popup();

            popover.connect_closed(move |p| {
                p.unparent();
            });
        });

        self.object_container.add_controller(rclick);
    }

    fn add_default_widgets(&mut self) {
        use self::objects::widget_object::WidgetType;

        let widgets: Vec<std::boxed::Box<dyn DesktopObject>> = vec![
            std::boxed::Box::new(WidgetObject::new(
                "Clock".into(),
                WidgetType::MiniClock,
                20.0,
                20.0,
            )),
            std::boxed::Box::new(WidgetObject::new(
                "Calendar".into(),
                WidgetType::Calendar,
                240.0,
                20.0,
            )),
            std::boxed::Box::new(WidgetObject::new(
                "System".into(),
                WidgetType::SystemMonitor,
                540.0,
                20.0,
            )),
        ];
        self.objects.borrow_mut().extend(widgets);

        let data: Vec<ObjectData> = self.objects.borrow().iter().map(|o| o.data()).collect();
        let _ = self.persistence.save(&data);
    }

    fn spawn_config_watcher(&self) {
        let (tx, rx) = async_channel::unbounded::<()>();

        std::thread::spawn(move || {
            let _ = xarph_sdk::config::watch_config(move |_| {
                let _ = tx.send_blocking(());
            });
            // Keep thread alive
            loop {
                std::thread::sleep(std::time::Duration::from_secs(3600));
            }
        });

        let mut renderer = self.renderer.clone();
        let current_workspace = self.current_workspace;
        glib::spawn_future_local(async move {
            while let Ok(()) = rx.recv().await {
                let config = XarphConfig::load();
                let wallpaper = config.get_wallpaper_for_workspace(current_workspace);
                renderer.set_config(wallpaper.clone());
            }
        });
    }

    fn spawn_workspace_watcher(&self) {
        let (tx, rx) = async_channel::unbounded::<u8>();

        std::thread::spawn(move || {
            loop {
                match Socket::connect() {
                    Ok(mut socket) => {
                        if let Ok(Ok(xarph_sdk::Response::Handled)) =
                            socket.send(xarph_sdk::Request::EventStream)
                        {
                            let mut read_event = socket.read_events();
                            loop {
                                match read_event() {
                                    Ok(xarph_sdk::Event::WorkspaceActivated { id, focused }) => {
                                        if focused {
                                            let _ = tx.send_blocking(id as u8);
                                        }
                                    }
                                    Ok(_) => {}
                                    Err(_) => break,
                                }
                            }
                        }
                    }
                    Err(_) => {}
                }
                std::thread::sleep(std::time::Duration::from_secs(2));
            }
        });

        let mut renderer = self.renderer.clone();
        glib::spawn_future_local(async move {
            while let Ok(ws_idx) = rx.recv().await {
                let config = XarphConfig::load();
                let wallpaper = config.get_wallpaper_for_workspace(ws_idx);
                renderer.set_config(wallpaper.clone());
            }
        });
    }
}
