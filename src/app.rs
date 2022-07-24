use egui_extras::{TableBuilder,Size};

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    // Example stuff:
    label: String,

    // this how you opt-out of serialization of a member
    #[serde(skip)]
    value: f32,
}

struct Character{
    name: String,
    race: String,
    class: String,
    alignment: String,
}

impl Default for Character{
    fn default() -> Self {
        Self { name: "".to_string(), race: "".to_string(), class: "".to_string(), alignment: "".to_string() }
    }
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            // Example stuff:
            label: "".to_owned(),
            value: 2.7,
        }
    }
}

impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customized the look at feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl eframe::App for TemplateApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        let Self { label, value } = self;

        // Examples of how to create different panels and windows.
        // Pick whichever suits you.
        // Tip: a good default choice is to just keep the `CentralPanel`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        frame.quit();
                    }
                });
            });
        });

        // egui::SidePanel::left("side_panel").show(ctx, |ui| {
        //     ui.heading("Side Panel");

        //     ui.horizontal(|ui| {
        //         ui.label("Write something: ");
        //         ui.text_edit_singleline(label);
        //     });

        //     ui.add(egui::Slider::new(value, 0.0..=10.0).text("value"));
        //     if ui.button("Increment").clicked() {
        //         *value += 1.0;
        //     }

        //     ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
        //         ui.horizontal(|ui| {
        //             ui.spacing_mut().item_spacing.x = 0.0;
        //             ui.label("powered by ");
        //             ui.hyperlink_to("egui", "https://github.com/emilk/egui");
        //             ui.label(" and ");
        //             ui.hyperlink_to("eframe", "https://github.com/emilk/egui/tree/master/eframe");
        //         });
        //     });
        // });

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's

            let table = TableBuilder::new(ui)
            .striped(true)
            .columns(Size::initial(50.0), 5)
            .resizable(true)
            .header(20.0 , |mut header|{
                header.col(|ui|{
                    ui.heading("Name");
                });
                header.col(|ui|{
                    ui.heading("Race");
                });
                header.col(|ui|{
                    ui.heading("Class");
                });
                header.col(|ui|{
                    ui.heading("Alignment");
                });
                header.col(|ui|{
                    ui.heading("Stats");
                });
            })
            .body(|mut body|{
                body.row(30.0, |mut row| {
                    let char = Character {
                        name: String::from("tank").to_owned(),
                        race: String::from("human"),
                        class: String::from("fighter"),
                        alignment: String::from("dog"),
                    };
                    row.col(|ui|{
                        let mut name = char.name.to_owned();
                        ui.text_edit_singleline(&mut name);
                    });
                    row.col(|ui| {
                        ui.label("fafaf");
                        ui.text_edit_singleline(label);
                    });
                    row.col(|ui| {
                        ui.label("");
                        ui.text_edit_singleline(label);
                    });
                    row.col(|ui| {
                        ui.label("");
                        ui.text_edit_singleline(label);
                    });
                    row.col(|ui| {
                        ui.label("");
                        ui.text_edit_singleline(label);
                    });
                })
            });
            // .column(Size::exact(50.0))
            // .header(20.0, |mut header| {
            //     header.col(|ui| {
            //         ui.heading("Growing");
            //     });
            //     header.col(|ui| {
            //         ui.heading("Fixed");
            //     });
            // });
            // table.body(body)
            // .body(|mut body| {
            //     body.row(30.0, |mut row| {
            //         row.col(|ui| {
            //             ui.label("first row growing cell");
            //         });
            //         row.col(|ui| {
            //             ui.button("action");
            //         });
            //     });
            // });
        });

        if false {
            egui::Window::new("Window").show(ctx, |ui| {
                ui.label("Windows can be moved by dragging them.");
                ui.label("They are automatically sized based on contents.");
                ui.label("You can turn on resizing and scrolling if you like.");
                ui.label("You would normally chose either panels OR windows.");
            });
        }
    }
}