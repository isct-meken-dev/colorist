mod api;

use tauri::Emitter;
use tauri_specta::collect_commands;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, Layer};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let specta_builder = tauri_specta::Builder::<tauri::Wry>::new().commands({
        use api::*;
        collect_commands![greet]
    });

    #[cfg(debug_assertions)]
    specta_builder
        .export(
            specta_typescript::Typescript::default(),
            "../node_modules/@specta/bindings.ts",
        )
        .expect("failed to export ts bindings");

    tauri::Builder::default()
        .setup(|app| {
            tracing_subscriber::registry()
                .with(TauriLogLayer {
                    app_handle: app.handle().clone(),
                })
                .init();
            Ok(())
        })
        .invoke_handler(specta_builder.invoke_handler())
        .run(tauri::generate_context!())
        .expect("アプリケーションの実行中にエラーが発生しました。");
}

struct TauriLogLayer {
    app_handle: tauri::AppHandle,
}
impl<S: tracing::Subscriber> Layer<S> for TauriLogLayer {
    fn on_event(
        &self,
        event: &tracing::Event<'_>,
        _ctx: tracing_subscriber::layer::Context<'_, S>,
    ) {
        let mut visitor = StringVisitor(String::new());
        event.record(&mut visitor);

        let level = event.metadata().level().to_string().to_lowercase();
        let message = visitor.0;

        self.app_handle.emit("rust-log", (level, message)).ok();
    }
}
struct StringVisitor(String);
impl tracing::field::Visit for StringVisitor {
    fn record_debug(&mut self, field: &tracing::field::Field, value: &dyn std::fmt::Debug) {
        if field.name() == "message" {
            self.0 = format!("{:?}", value);
        }
    }
}
