use std::sync::OnceLock;
use std::time::Duration;

use cushy::value::{Destination, Dynamic};
use cushy::widget::MakeWidget;
use cushy::widgets::progress::Progressable;
use cushy::Run;
use tokio::runtime;
use tokio::time::sleep;

// This function can be called safely from any thread to get a handle to the
// running tokio runtime. This is needed until Cushy has a way to initialize
// tokio in the threads it uses:
// <https://github.com/khonsulabs/cushy/issues/147>.
fn tokio_runtime() -> &'static runtime::Handle {
    static RUNTIME: OnceLock<runtime::Handle> = OnceLock::new();
    RUNTIME.get_or_init(|| {
        let rt = runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .expect("tokio initialization error");
        let handle = rt.handle().clone();
        std::thread::spawn(move || {
            // Replace with the async main loop, or some sync structure to
            // control shutting it down if desired.
            rt.block_on(async {
                loop {
                    sleep(Duration::from_secs(1000)).await
                }
            });
        });
        handle
    })
}

fn main() {
    let progress = Dynamic::new(0_u8);
    let progress_bar = progress.clone().progress_bar();
    "Press Me"
        .into_button()
        .on_click(move |()| {
            tokio_runtime().spawn(do_something(progress.clone()));
        })
        .and(progress_bar)
        .into_rows()
        .centered()
        .expand()
        .run()
        .expect("error starting Cushy");
}

async fn do_something(progress: Dynamic<u8>) {
    for i in 0..u8::MAX {
        progress.set(i);
        sleep(Duration::from_millis(10)).await
    }
}
