import config from "../config"

function Usage() {
    return (
        <>
            <p>
                問題文は
                <a 
                    href={config.usage.problemLink}
                    target="_blank"
                >
                    こちら
                </a>
                。
            </p>

            <p>
                使い方
                <br />
                <details open={config.usage.isDefaultOpen}>
                    <p>
                        Seed 欄の値を変えると対応する入力が Input 欄に生成されます。
                        <br />
                        Download ボタンを押すと、シード値が seed, seed+1, ..., seed+#cases-1 に対応する入力を一括ダウンロード出来ます。
                        <br />
                    </p>
                    <p>
                        生成された入力に対して解答プログラムをローカル実行し、プログラムの出力を Output 欄に貼り付けると、ビジュアライズ結果が表示されます。
                        <br />
                        ▶ ボタンを押すと、アニメーションが開始します。
                        <br />
                        Save as PNG ボタンを押すと、ビジュアライズ結果のダウンロードが出来ます。
                        <br />
                    </p>
                    <p>
                        上部の「ファイルを選択」ボタンを押して出力ファイルを含むディレクトリを選択することで、複数の出力を素早く切り替えることが出来るようになります。
                        <br />
                        出力ファイル名を 1234.txt もしくは abcd_1234.txt という形式にすることで、ファイルを選択時に自動的にseed番号(この場合は1234)が設定されます。
                        <br />
                    </p>
                </details>
            </p>

            <hr />
        </>
    );
};

export default Usage;