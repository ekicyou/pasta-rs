# pasta-rs
パスタのレシピ

## ドキュメント
整備中です。

## ライセンス
MITです。詳細は後日記載。

## windowsでrust+vscode環境を揃えてニヤニヤする方法
いつの間にかrust+vscodeのインストールがとっても簡単になっていました。
windows環境であればこんな感じに設定すれば、だれのマシンでも同じrust環境が
設定できます。以下、インストールメモです。

1. VS2015をインストール
 * コミュニティ版でもOKです。
 * インストールオプションに注意してください。
   必ず、'C++ / Windows10SDK'の最新版を入れること。
   rustはVS2015のリンカーなどを利用します。

2. [rustup][rustup]をインストール
 * 上記サイトからでも選択できますが、[ダウンロード一覧][rustup_items]より、
   windows 64bitの場合は"x86_64-pc-windows-msvc"を導入することを強く推奨します。

3. コマンドラインで'rustup target list'と入力
 * インストール可能なbuild target一覧が表示されます。
   本能の赴くままに、インストールしたいターゲットをコピペしておいてください。

4. 一括インストール
 * 下記をコピペして、batファイルを作ってください。
 * 'rustup target add'のところはさっきコピペしておいたものをずらずら並べてください。
 * batファイルが出来たら実行してください。
   環境にもよりますが、小一時間はかかります。こっちは最小化しておいて、
   次の作業に進んでください。
 * もし途中で失敗している場合は、他のアプリを終了してリトライしてください。
   VS2015、C++、Win10SDKが正しくインストールされている場合は通るはずです。

```inst.bat
rustup target add i686-pc-windows-msvc
rustup target add x86_64-pc-windows-msvc
rustup update
cargo install racer
cargo install rustfmt
cargo install rustsym
cargo install mdbook
cargo install cargo-edit
cargo install cargo-script
cargo install cargo-update[^cargo-update]
cargo install-update -a
cargo update
```
[^cargo-update]:ビルド中に"rc.exe"が要求される場合は、事前にWindows SDKをインストールし、/bin/x64/ にPATHを通しておくこと。
                通常はVS2015のインストール時に最新のWindows10SDKを一緒にインストールしておけばよきに計らってくれる。

5. rustのソースコードリポジトリを取得
 * 必須ではありませんが、導入しておくとインテリセンスが有効になります。
 * 適当なフォルダに[rustリポジトリ][rust_src]よりgit cloneしてください。
 * 'stable'branchをcheck outしてください。masterブランチはrust開発用です。
 * インストールフォルダのURLは後で利用します。

6. [Visual Studio Code][vscode]のインストール
 * サイトに行って最新版をダウンロードし、インストールしてください。

7. Visual Studio Codeの拡張機能をインストール
 * バッチファイルの実行が終わるのを待ってください。
   rust関係以外の拡張機能をインストールするのは問題ありません。
 * 'ctrl + shift + x'を押してください。「拡張機能」欄が表示されます。
 * 'Rusty Code'を検索し、インストールしてください。

8. Visual Studio Codeの環境設定
 * 'ファイル -> ユーザー設定'より、JSONファイルを直接編集します。ワイルドですねｗ。
 * "rust.rustLangSrcPath"項目は、'rustフォルダ/src/'の場所を設定してください。
 * 設定晒しておきます。ご参考に。

 ```settings.json
// 既定の設定を上書きするには、このファイル内に設定を挿入します
{
    // エディター

    // フォント ファミリを制御します。
    "editor.fontFamily": "MoboGothic, Consolas, 'Courier New', monospace",

    // 1 つのタブに相当するスペースの数。`editor.detectIndentation` がオンの場合、この設定はファイル コンテンツに基づいて上書きされます。
    "editor.tabSize": 4,

    // エディターでインデントのガイドを表示する必要があるかどうかを制御します
    "editor.renderIndentGuides": true,

    // ファイルを保存するときにフォーマットする。
    "editor.formatOnSave": true,

    // 拡張機能

    // 拡張機能を自動的に更新します
    "extensions.autoUpdate": true,


    // Rusty Code configuration

    // Specifies path to /src directory of local copy of Rust sources
    "rust.rustLangSrcPath": "D:/home/maz/rust/rust/src",


    // テレメトリ

    // 利用状況データとエラーを Microsoft に送信できるようにします。
    "telemetry.enableTelemetry": true,

    // クラッシュ レポートを Microsoft に送信するように設定します。
    // このオプションを有効にするには、再起動が必要です。
    "telemetry.enableCrashReporter": true
}
 ```

9. rust+vscodeでニヤニヤする
 * これでrust+vscodeの素晴らしいエコシステムの恩恵を受けられます。
 * 環境のアップグレードは下記コマンドで可能です。バッチファイルを作っておくと便利ですよ。

 ```update.bat
rustup update
cargo update
pushd path/to/rust/src
  git pull
  popd
```

以上、後は存分にコードを作ってニヤニヤしてください！



[rustup]: https://www.rustup.rs/ "rustup"
[rustup_items]: https://github.com/rust-lang-nursery/rustup.rs/#other-installation-methods "ダウンロード一覧"
[rust_src]: https://github.com/rust-lang/rust "rustリポジトリ"
[vscode]: https://code.visualstudio.com/ "Visual Studio Code"
