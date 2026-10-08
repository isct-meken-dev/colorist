mod api;

use tauri::Emitter;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, Layer};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    #[cfg(debug_assertions)]
    tauri_specta::Builder::<tauri::Wry>::new()
        .commands(api::commands())
        .types(&specta::collect())
        .export(
            specta_typescript::Typescript::default(),
            "../src/gen/bindings.ts",
        )
        .expect("バインディングの出力に失敗しました。");

    tauri::Builder::<tauri::Wry>::new()
        .setup(|app| {
            tracing_subscriber::registry()
                .with(TauriLogLayer {
                    app_handle: app.handle().clone(),
                })
                .init();
            Ok(())
        })
        .invoke_handler(api::invoke_handler())
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
