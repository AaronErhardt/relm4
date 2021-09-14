use adw::traits::ApplicationWindowExt;
use gtk::prelude::{BoxExt, ButtonExt, GtkWindowExt, OrientableExt};
use relm4::{send, AppUpdate, Model, RelmApp, Sender, WidgetPlus, Widgets};

#[derive(Default)]
struct AppModel {
    counter: u8,
}

enum AppMsg {
    Increment,
    Decrement,
}

impl Model for AppModel {
    type Msg = AppMsg;
    type Widgets = AppWidgets;
    type Components = ();
    type Settings = ();
}

impl AppUpdate for AppModel {
    fn update(&mut self, msg: AppMsg, _components: &(), _sender: Sender<AppMsg>) -> bool {
        match msg {
            AppMsg::Increment => {
                self.counter = self.counter.wrapping_add(1);
            }
            AppMsg::Decrement => {
                self.counter = self.counter.wrapping_sub(1);
            }
        }
        true
    }
}

#[relm4_macros::widget]
impl Widgets<AppModel, ()> for AppWidgets {
    view! {
        main_window = adw::ApplicationWindow {
            GtkWindowExt::set_default_width: 300,
            GtkWindowExt::set_default_height: 200,

            ApplicationWindowExt::set_child: main_box = Some(&gtk::Box) {
                set_orientation: gtk::Orientation::Vertical,

                append = &adw::HeaderBar {
                    set_title_widget = Some(&gtk::Label) {
                        set_label: "Title!",
                    }
                },
                append = &gtk::Box {
                    set_orientation: gtk::Orientation::Vertical,
                    set_margin_all: 5,
                    set_spacing: 5,

                    append = &gtk::Button {
                        set_label: "Increment",
                        connect_clicked(sender) => move |_| {
                            send!(sender, AppMsg::Increment);
                        },
                    },
                    append = &gtk::Button {
                        set_label: "Decrement",
                        connect_clicked(sender) => move |_| {
                            send!(sender, AppMsg::Decrement);
                        },
                    },
                    append = &gtk::Label {
                        set_margin_all: 5,
                        set_label: watch! { &format!("Counter: {}", model.counter) },
                    }
                }
            },
        }
    }
}

fn main() {
    let model = AppModel::default();
    let app = RelmApp::new(model, &());
    app.run();
}
