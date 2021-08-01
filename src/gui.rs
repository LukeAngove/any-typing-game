use orbtk::prelude::*;
use crate::backend::Doer;
use crate::ui_trait::UI;

pub struct Gui {
    doer : Doer,
}

impl UI for Gui {
    fn new(mut doer : Doer) -> Self {
        //input.handle(move |me, ev| match ev {
        //    Event::KeyDown => {
        //        println!("Stuff happened!");
        //        output.set_value("Luke");
        //        doer.check_and_do(&me.value()).unwrap();
        //        true
        //    },
        //    _ => false,
        //});

        Gui {
            doer,
        }
    }
        
    fn main_loop(&mut self) -> Result<!, Box<dyn std::error::Error>> {
        let _app = Application::new()
            .window(|ctx| {
                Window::new()
                    .title("Title!")
                    .position((100.0, 100.0))
                    .size(200, 100)
                    .child(
                        TextBlock::new()
                            .text("Hi!")
                            .v_align("center")
                            .h_align("center")
                            .build(ctx)
                    )
                    .build(ctx)
            })
            .run();
        loop {}
    }
}