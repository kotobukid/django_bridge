# Protocol Buffers コンパイラ (protoc) のインストール方法

このプロジェクトは Protocol Buffers を使用しており、ビルドするには `protoc` コンパイラが必要です。以下の手順に従って、お使いのオペレーティングシステムに `protoc` をインストールしてください。

## Debian/Ubuntu

```bash
sudo apt-get update
sudo apt-get install -y protobuf-compiler
```

インストール後、以下のコマンドでバージョンを確認できます：

```bash
protoc --version
```

## macOS

Homebrew を使用してインストールできます：

```bash
brew install protobuf
```

## Windows

1. [Protocol Buffers のリリースページ](https://github.com/protocolbuffers/protobuf/releases) から最新の Windows 用バイナリをダウンロードします（例：`protoc-24.4-win64.zip`）
2. ダウンロードしたZIPファイルを解凍します
3. 解凍したフォルダ内の `bin` ディレクトリを PATH 環境変数に追加します

## 手動インストール（すべてのプラットフォーム）

1. [Protocol Buffers のリリースページ](https://github.com/protocolbuffers/protobuf/releases) から、お使いのプラットフォーム用の最新バージョンをダウンロードします
2. ダウンロードしたアーカイブを解凍します
3. 解凍したディレクトリ内の `bin` フォルダにある `protoc` バイナリを、PATH が通っている場所に配置します

## 環境変数の設定

`protoc` がインストールされているにもかかわらず見つからない場合は、`PROTOC` 環境変数を設定することで、明示的にパスを指定できます：

```bash
export PROTOC=/path/to/protoc
```

Windows の場合：

```cmd
set PROTOC=C:\path\to\protoc.exe
```

## トラブルシューティング

インストール後も問題が解決しない場合は、以下を確認してください：

1. `protoc` が PATH に含まれているか
2. インストールしたバージョンが最新か
3. 必要に応じて `PROTOC` 環境変数を設定する

詳細については、[Protocol Buffers のドキュメント](https://developers.google.com/protocol-buffers) を参照してください。