# Colorist

新しい彩色ソフト。

## コマンド

- **clone/fetch後必ず実行**
    ```bash
    pnpm tauri icon
    cd src-tauri
    cargo run
    cd ..
    ```
  > [!NOTE]
  > 壊れたウィンドウが開きますが仕様です。

- 開発モード実行
    ```bash
    pnpm tauri dev
    ```

- リリースビルド
    ```bash
    pnpm tauri build
    ```
  > [!NOTE]
  > かなり時間かかるので注意。

- コードの一括フォーマット
    ```bash
    pnpm fmt
    cd src-tauri
    cargo fmt
    cargo sort
    cd ..
    ```
  > [!NOTE]
  > 事前に `cargo install cargo-sort` が必要です。
