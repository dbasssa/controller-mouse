use ksni::Tray;
use ksni::menu::StandardItem;
use ksni::blocking::TrayMethods;
use std::sync::OnceLock;
use image::GenericImageView;

pub fn spawn() -> ksni::blocking::Handle<MyTray> {
    MyTray {}.spawn().expect("tray spawn failed")
}

static PIXMAP: OnceLock<ksni::Icon> = OnceLock::new();

fn pixmap() -> &'static ksni::Icon {
    PIXMAP.get_or_init(|| {
        // Decode the bundled PNG once (RGBA8)
        let img = image::load_from_memory_with_format(
            include_bytes!("assets/app-icon.png"),
            image::ImageFormat::Png,
        )
        .expect("valid image");
        let (width, height) = img.dimensions();
        let mut data = img.into_rgba8().into_vec();
        assert_eq!(data.len() % 4, 0);
        for pixel in data.chunks_exact_mut(4) {
            pixel.rotate_right(1)
        }
        ksni::Icon {
            width: width as i32,
            height: height as i32,
            data,
        }
    })
}

pub struct MyTray;

impl Tray for MyTray {
    fn id(&self) -> String { "gamepad-mouse".into() }
    fn title(&self) -> String { "gamepad-mouse".into() }
    fn icon_pixmap(&self) -> Vec<ksni::Icon> {
        vec![pixmap().clone()]
    }
    fn menu(&self) -> Vec<ksni::MenuItem<Self>> {
        vec![
            StandardItem {
                label: "quit".into(),
                activate: Box::new(|_tray: &mut MyTray| std::process::exit(0)),
                ..Default::default()
            }
            .into(),
        ]
    }
}
