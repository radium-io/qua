use cursive::{
    align::HAlign,
    backend, backends,
    traits::Nameable,
    views::{self, LinearLayout, Panel, ResizedView, TextArea, TextView},
    CursiveRunnable,
};
use cursive_buffered_backend::BufferedBackend;

fn main() {
    let mut siv = CursiveRunnable::new(
        || -> Result<Box<dyn backend::Backend>, crossterm::ErrorKind> {
            let crossterm_backend = backends::crossterm::Backend::init()?;
            Ok(Box::new(BufferedBackend::new(crossterm_backend)))
        },
    );

    siv.add_global_callback('q', |s| s.quit());

    let select_panel = Panel::new(TextArea::new().with_name("select"))
        .title("SELECT")
        .title_position(HAlign::Left);

    let from_panel = Panel::new(TextArea::new().with_name("from"))
        .title("FROM")
        .title_position(HAlign::Left);

    let where_panel = Panel::new(TextArea::new().with_name("where"))
        .title("WHERE")
        .title_position(HAlign::Left);

    let parts_layout = LinearLayout::vertical()
        .child(ResizedView::with_full_screen(select_panel))
        .child(ResizedView::with_full_screen(from_panel))
        .child(ResizedView::with_full_screen(where_panel));

    let output_view = ResizedView::with_full_height(TextView::new("(OUTPUT)"));

    let screen_layout = LinearLayout::horizontal()
        .child(parts_layout)
        .child(output_view);

    siv.add_fullscreen_layer(screen_layout);

    siv.run();

    print!(
        "SELECT {} FROM {}",
        siv.call_on_name("select", move |v: &mut views::TextArea| {
            String::from(v.get_content())
        })
        .unwrap(),
        siv.call_on_name("from", move |v: &mut views::TextArea| {
            String::from(v.get_content())
        })
        .unwrap(),
    );

    if let Some(where_clause) = siv.call_on_name("where", move |v: &mut views::TextArea| {
        String::from(v.get_content())
    }) {
        if where_clause.len() > 0 {
            print!(" WHERE {}", where_clause);
        }
    }

    print!(";")
}
