use gtk4::prelude::*;
use gtk4::{Fixed, GestureDrag, Widget};

/// Attach a drag-to-move gesture to `target`.
/// On drag, moves the widget within its parent `Fixed` container.
/// `on_drag_end` is called with the final (x, y) position when the drag finishes.
pub fn attach_drag_move(target: &Widget, on_drag_end: Option<Box<dyn Fn(f64, f64)>>) {
    let drag = GestureDrag::new();
    drag.set_button(1);

    let drag_target = target.clone();
    drag.connect_drag_begin(move |_, _, _| {
        drag_target.add_css_class("dragging");
    });

    let drag_target2 = target.clone();
    drag.connect_drag_update(move |_, offset_x, offset_y| {
        if let Some(parent) = drag_target2.parent() {
            if let Some(fixed) = parent.downcast_ref::<Fixed>() {
                let alloc = drag_target2.allocation();
                fixed.move_(
                    &drag_target2,
                    alloc.x() as f64 + offset_x,
                    alloc.y() as f64 + offset_y,
                );
            }
        }
    });

    let drag_target3 = target.clone();
    drag.connect_drag_end(move |_, _, _| {
        drag_target3.remove_css_class("dragging");
        if let Some(cb) = &on_drag_end {
            let alloc = drag_target3.allocation();
            cb(alloc.x() as f64, alloc.y() as f64);
        }
    });

    target.add_controller(drag);
}
