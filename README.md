# wikipedia-ime-dictionary

## インストール
Rustがインストール済みでcargoコマンドが動作する環境であれば、このプロジェクトをローカルにダウンロードするだけでOK。

## 使用方法
1. このプロジェクトをgit cloneまたはアーカイブでダウンロードする
1. Wikipediaの概要データを[このリンク先](https://dumps.wikimedia.org/jawiki/latest/jawiki-latest-abstract.xml.gz)からダウンロード（数百MB。所要時間10分程度）
1. 適当な展開ツールでファイルを解凍。jawiki-latest-abstract.xmlをプロジェクトのルートに配置
1. ツールを実行(cargo run --release)
1. 5分ほどで同じディレクトリにMS-IME用のオープン辞書(*.dctx)が出力される。
1. ダブルクリックで開き、辞書をインポートすれば使用可能

## 既知の問題
* 自前でgzip解凍しようとしたが、ダウンロードできるデータが壊れておりうまくいかなかったため断念（7zipでは解凍できた）

## 注意
出力されたファイルはWikipediaのライセンスに準拠するため、再配布する場合は注意すること。
