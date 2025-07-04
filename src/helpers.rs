use std::fs::read_to_string;
use cursive::align::HAlign;
use cursive::event::Key;
use cursive::view::{Nameable, Resizable};
use cursive::views::{
    Dialog, EditView, LinearLayout, OnEventView, ScrollView, SelectView,
};
use cursive::{Cursive, View};

pub fn get_locales() -> String {
    return read_to_string("/usr/share/i18n/SUPPORTED").expect("Failed to read locales file");
}

pub fn get_timezones() -> Vec<(String, String)> {
    let content =
        read_to_string("/usr/share/zoneinfo/zone.tab").expect("Failed to read timezones file");

    let mut timezones: Vec<(String, String)> = Vec::new();

    for line in content.lines() {
        if line.starts_with('#') || line.trim().is_empty() {
            continue;
        }
        if let Some(tz) = line.split_whitespace().nth(2) {
            let stringed = tz.to_string();
            timezones.push((stringed.clone(), stringed));
        }
    }

    timezones.sort();
    return timezones;
}

pub fn wrap_view_with_shortcuts<T: View>(view: T) -> OnEventView<T> {
    OnEventView::new(view).on_event(Key::Esc, |cursive| {
        cursive.pop_layer();
    })
}

pub fn single_select_menu<T, I, F>(cursive: &mut Cursive, dialog_title: &str, items: I, on_select: F)
where
    T: Clone + Send + Sync + 'static,
    I: IntoIterator<Item = (String, T)>,
    F: Fn(&mut Cursive, &T) + Send + Sync + 'static,
{
    let mut select_view = SelectView::<T>::new().h_align(HAlign::Center);
    for (label, value) in items {
        select_view.add_item(label, value);
    }
    let select_view = select_view.on_submit(move |cursive, selected_value| {
        on_select(cursive, selected_value);
        cursive.pop_layer();
    });

    cursive.add_layer(wrap_view_with_shortcuts(ScrollView::new(
        Dialog::new()
            .title(dialog_title)
            .content(select_view)
            .button("Cancel", |cursive| {
                cursive.pop_layer();
            }),
    )));
}

pub fn titled_input_row(label: &str, input_name: &str) -> LinearLayout {
    LinearLayout::horizontal()
        .child(Dialog::text(label))
        .child(EditView::new().with_name(input_name).fixed_width(20))
}

pub fn input_dialog<F>(cursive: &mut Cursive, dialog_title: &str, input_name: String, on_ok: F)
where
    F: Fn(&mut Cursive, String) + Send + Sync + 'static,
{
    let edit = EditView::new().on_submit(move |cursive, val| {
        if !val.is_empty() {
            on_ok(cursive, val.to_string());
        } else {
            cursive.add_layer(Dialog::info("Input cannot be empty."));
            return;
        }
        cursive.pop_layer();
    });

    cursive.add_layer(wrap_view_with_shortcuts(
        Dialog::new()
            .title(dialog_title)
            .content(edit.fixed_width(20).with_name(&input_name))
            .button("Cancel", |cursive| {
                cursive.pop_layer();
            }),
    ));
}