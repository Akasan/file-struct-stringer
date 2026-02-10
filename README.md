# file-struct-stringer

フォルダ構造を読みやすいテキスト形式で表示するRust製CLIツール。テックブログやドキュメントでプロジェクトの構造を示すのに便利です。

## 特徴

- 📁 階層的なツリー表示（ボックス描画文字を使用）
- 🎯 フォルダのみを表示するオプション
- 🔍 特定のファイル拡張子でフィルタリング（複数指定可能）
- 📏 カスタマイズ可能なブランチ文字のダッシュ数
- 🚫 一般的な不要ディレクトリを自動除外（.git, node_modules, target等）

## インストール

### ローカルビルド

```bash
# リポジトリをクローン
git clone <repository-url>
cd file-struct-stringer

# ローカルにインストール
cargo install --path .
```

### ビルドのみ

```bash
# デバッグビルド
cargo build

# リリースビルド（最適化あり）
cargo build --release
```

## 使用方法

### 基本的な使い方

```bash
# カレントディレクトリの構造を表示
file-struct-stringer

# 特定のディレクトリを表示
file-struct-stringer /path/to/directory
```

### オプション

```
Usage: file-struct-stringer [OPTIONS] [PATH]

Arguments:
  [PATH]  対象ディレクトリ（デフォルト: カレントディレクトリ）

Options:
  -f, --folders-only       フォルダのみを表示（ファイルは表示しない）
  -e, --format <FORMAT>    指定した拡張子のファイルのみ表示（カンマ区切りで複数指定可能）
  -d, --dashes <DASHES>    ブランチ文字のダッシュ数（デフォルト: 2）
  -h, --help               ヘルプを表示
```

## 使用例

### 基本表示

```bash
file-struct-stringer
```

出力例：
```
./
├── .gitignore
├── CLAUDE.md
├── Cargo.lock
├── Cargo.toml
├── README.md
└── src/
    └── main.rs
```

### フォルダのみ表示

```bash
file-struct-stringer --folders-only
# または
file-struct-stringer -f
```

出力例：
```
./
└── src/
```

### 特定の拡張子でフィルタリング

```bash
# .rsと.tomlファイルのみ表示
file-struct-stringer --format rs,toml
# または
file-struct-stringer -e rs,toml
```

出力例：
```
./
├── Cargo.toml
└── src/
    └── main.rs
```

### カスタムダッシュ数

```bash
# ダッシュ4つのブランチ文字（├──── や └────）
file-struct-stringer --dashes 4
# または
file-struct-stringer -d 4

# ダッシュ1つのブランチ文字（├─ や └─）
file-struct-stringer --dashes 1
```

### オプションの組み合わせ

```bash
# .mdファイルのみをダッシュ3つで表示
file-struct-stringer -e md -d 3

# 特定のディレクトリの.rsファイルのみ表示
file-struct-stringer --format rs ./src

# フォルダのみをダッシュ1つで表示
file-struct-stringer --folders-only --dashes 1
```

## 除外されるディレクトリ

以下のディレクトリは自動的に除外されます：

- `.git`
- `node_modules`
- `target`
- `.idea`
- `.vscode`

## 開発

### 必要な環境

- Rust 1.93.0以上

### 開発コマンド

```bash
# プロジェクトのチェック
cargo check

# コードのフォーマット
cargo fmt

# Lintチェック
cargo clippy

# テスト実行
cargo test

# 開発中の実行
cargo run -- [引数]
```

### 依存関係

- `clap` (v4.5) - CLIパーサー
- `walkdir` (v2.5) - ディレクトリトラバーサル

## ライセンス

このプロジェクトのライセンスについては、リポジトリのLICENSEファイルを参照してください。
