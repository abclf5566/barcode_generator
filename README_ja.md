[日本語版はこちら](README_ja.md)
[Read this in English](README.md)
[Download the latest release here](https://github.com/abclf5566/barcode_generator/releases/download/release/barcode_generator.zip)

# Windows 10用デバイスIMEIバーコードジェネレーター

## 概要
このプログラムは、Windows 10の`netsh mbn show interface` から取得したデバイスIDからバーコードを生成するよう設計されています。
Code128バーコード規格を使用しており、生成されたバーコードはPNGファイルとして保存され、オプションでシステムのデフォルト画像ビューアで開くことができます。

簡単いうとWindows10デバイスIMEIを所得したらバーコード化し、画像ファイルを開くことです。
IMEIが搭載されていない場合は何も表示されません。

## 機能
- **デバイスIMEIの取得：** プログラムは `netsh mbn show interface` コマンドを使用して、数値のみで構成される最後のデバイスIDを見つけます。
- **バーコード生成：** Code128バーコードに必要な開始文字を追加し、デバイスIMEIからバーコード画像を生成します。
- **画像の保存と表示：** バーコードをPNGファイルとして保存し、システムのデフォルト画像ビューアで開くことを試みます。

## コード説明
- **コマンド実行：** `netsh mbn show interface | findstr /R "[0-9][0-9]*$"` コマンドを実行して、数値で終わる行のみを出力からフィルタリングします。
- **バーコード処理：** デバイスIMEIの始めに 'Ɓ' (Code128の文字セットB用の開始文字) を追加して、バーコード規格に準拠します。
- **画像生成：** バーコードデータに基づいてピクセルを設定することでバーコード画像を生成します。ここで「0」は白を、「1」は黒を表します。
- **エラー処理：** バーコードが生成または保存できないシナリオに対して適切なエラー処理を実装します。

## 要件
- **Rust環境：** ソースコードをコンパイルしたい場合は、マシンにRustがインストールされていることを確認してください。
- **外部クレート：** バーコード生成には `barcoders`、コマンド実行には `subprocess`、画像処理には `image` を使用しています。

## 実行方法
1. **プログラムのコンパイル：**
   ```bash
   cargo build --release
2. **または、コンパイル済みの.exeを実行：**
   ```bash
   ./target/release/barcode_generator.exe