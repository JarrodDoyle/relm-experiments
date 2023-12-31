use gtk::prelude::{BoxExt, ButtonExt, OrientableExt};
use rand::seq::IteratorRandom;
use relm4::{
    gtk, set_global_css, ComponentParts, ComponentSender, RelmApp, RelmWidgetExt, SimpleComponent,
};

const ICON_LIST: &[&str] = &[
    "bookmark-new-symbolic",
    "edit-copy-symbolic",
    "edit-cut-symbolic",
    "edit-find-symbolic",
    "starred-symbolic",
    "system-run-symbolic",
    "emoji-objects-symbolic",
    "emoji-nature-symbolic",
    "display-brightness-symbolic",
];

fn random_icon_name() -> &'static str {
    ICON_LIST
        .iter()
        .choose(&mut rand::thread_rng())
        .expect("Could not choose a random icon")
}

#[tracker::track]
struct AppModel {
    first_icon: &'static str,
    second_icon: &'static str,
    identical: bool,
}

#[derive(Debug)]
enum AppInput {
    UpdateFirst,
    UpdateSecond,
}

#[relm4::component]
impl SimpleComponent for AppModel {
    type Init = u8;
    type Input = AppInput;
    type Output = ();

    view! {
        #[root]
        gtk::ApplicationWindow {
            #[track = "model.changed(AppModel::identical())"]
            set_class_active: ("identical", model.identical),
            gtk::Box {
                set_orientation: gtk::Orientation::Horizontal,
                set_spacing: 10,
                set_margin_all: 10,
                gtk::Box {
                    set_orientation: gtk::Orientation::Vertical,
                    set_spacing: 10,
                    gtk::Image {
                        set_pixel_size: 50,
                        #[track = "model.changed(AppModel::first_icon())"]
                        set_icon_name: Some(model.first_icon),
                    },
                    gtk::Button {
                        set_label: "New random image",
                        connect_clicked[sender] => move |_| {
                            sender.input(AppInput::UpdateFirst)
                        }
                    }
                },
                append = &gtk::Box {
                    set_orientation: gtk::Orientation::Vertical,
                    set_spacing: 10,
                    gtk::Image {
                        set_pixel_size: 50,
                        #[track = "model.changed(AppModel::second_icon())"]
                        set_icon_name: Some(model.second_icon),
                    },
                    gtk::Button {
                        set_label: "New random image",
                        connect_clicked[sender] => move |_| {
                            sender.input(AppInput::UpdateSecond)
                        }
                    }
                },
            }
        }
    }

    fn init(
        _params: Self::Init,
        window: &Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        set_global_css(".identical { background: #00ad5c; }");

        let model = AppModel {
            first_icon: random_icon_name(),
            second_icon: random_icon_name(),
            identical: false,
            tracker: 0,
        };
        let widgets = view_output!();
        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, _sender: ComponentSender<Self>) {
        self.reset();

        match message {
            AppInput::UpdateFirst => {
                self.set_first_icon(random_icon_name());
            }
            AppInput::UpdateSecond => {
                self.set_second_icon(random_icon_name());
            }
        }
        self.set_identical(self.first_icon == self.second_icon);
    }
}

fn main() {
    let app = RelmApp::new("relm4.test.tracker_app");
    app.run::<AppModel>(0);
}
