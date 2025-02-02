#![allow(non_snake_case, unused_macros)]

use itertools::Itertools;
use proconio::{input, marker::Chars};
use rand::prelude::*;
use std::ops::RangeBounds;

//=========== 共通で使うトレイト・マクロなど ==========//

/// setmin, setmax を提供するトレイト
pub trait SetMinMax {
    fn setmin(&mut self, v: Self) -> bool;
    fn setmax(&mut self, v: Self) -> bool;
}
impl<T> SetMinMax for T
where
    T: PartialOrd,
{
    fn setmin(&mut self, v: T) -> bool {
        *self > v && {
            *self = v;
            true
        }
    }
    fn setmax(&mut self, v: T) -> bool {
        *self < v && {
            *self = v;
            true
        }
    }
}

#[macro_export]
macro_rules! mat {
    ($($e:expr),*) => { Vec::from(vec![$($e),*]) };
    ($($e:expr,)*) => { Vec::from(vec![$($e),*]) };
    ($e:expr; $d:expr) => { Vec::from(vec![$e; $d]) };
    ($e:expr; $d:expr $(; $ds:expr)+) => { Vec::from(vec![mat![$e $(; $ds)*]; $d]) };
}

//=========== 入力を保持する構造体 ==========//

/// Input 構造体: 問題文に対応した入力情報を保持する
///  - ty: テストケースの種類 (0〜19)
///  - n: 盤面サイズ (N×N)
///  - a: 各マスに書かれている数字
///  - vs: 各行における「右方向」の壁情報 ( '0' なら壁なし, '1' なら壁あり )
///  - hs: 各列における「下方向」の壁情報 ( '0' なら壁なし, '1' なら壁あり )
#[derive(Clone, Debug)]
pub struct Input {
    pub ty: u64,
    pub n: usize,
    pub a: Vec<Vec<i32>>,
    pub vs: Vec<Vec<char>>,
    pub hs: Vec<Vec<char>>,
}

/// Input 構造体の表示 (デバッグ用 / gen などで生成したものの出力に使う)
impl std::fmt::Display for Input {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // 問題文にある形式に準じて出力する
        writeln!(f, "{} {}", self.ty, self.n)?;
        for i in 0..self.n {
            // vs[i] は N-1 個の文字
            writeln!(f, "{}", self.vs[i].iter().collect::<String>())?;
        }
        for i in 0..self.n - 1 {
            // hs[i] は N 個の文字
            writeln!(f, "{}", self.hs[i].iter().collect::<String>())?;
        }
        for i in 0..self.n {
            writeln!(f, "{}", self.a[i].iter().join(" "))?;
        }
        Ok(())
    }
}

//=========== 入力のパース関数 ==========//

/// 提出時に与えられる本番の入力をパースする
pub fn parse_input(f: &str) -> Input {
    let f = proconio::source::once::OnceSource::from(f);
    input! {
        from f,
        ty: u64, n: usize,
        vs: [Chars; n],
        hs: [Chars; n - 1],
        a: [[i32; n]; n],
    }
    Input { ty, n, a, vs, hs }
}

/// vs, hsのみを持った固定のマップ (in_fixed/t.txt相当) を読み込み、
/// 数字 a は空配列のままにしておくためのパース関数
pub fn parse_input_fixed(f: &str) -> Input {
    let f = proconio::source::once::OnceSource::from(f);
    input! {
        from f,
        ty: u64, n: usize,
        vs: [Chars; n],
        hs: [Chars; n - 1],
    }
    // vs[i].len() == n-1, hs[i].len() == n を念のため assert
    for i in 0..n {
        assert_eq!(vs[i].len(), n - 1);
    }
    for i in 0..n - 1 {
        assert_eq!(hs[i].len(), n);
    }
    // a はあとで乱数で埋め込むので空のまま返す
    Input {
        ty,
        n,
        a: vec![],
        vs,
        hs,
    }
}

//=========== 出力を保持する構造体 ==========//

/// Output 構造体: 解答(行動列など)を保持する
///  - start: (p_i, p_j, q_i, q_j) の初期位置
///  - out: (do_swap, dir1, dir2) の列
///    - do_swap == true のとき数字を交換 (1)
///    - dir1: 高橋君の移動(U/D/L/R/.) をDIRSの添字で表現している
///    - dir2: 青木君の移動(U/D/L/R/.) をDIRSの添字で表現している
pub struct Output {
    pub start: (usize, usize, usize, usize),
    pub out: Vec<(bool, usize, usize)>,
}

//=========== 出力のパース関数 ==========//

/// 誤りを文字列で返す形にしておく(Score計算時にチェック)
pub fn parse_output(input: &Input, f: &str) -> Result<Output, String> {
    let mut out = vec![];
    let mut ss = f.split_whitespace();

    // 最初に初期位置4つを読む
    let start = (
        read(ss.next(), 0..input.n)?,
        read(ss.next(), 0..input.n)?,
        read(ss.next(), 0..input.n)?,
        read(ss.next(), 0..input.n)?,
    );

    // 残りは行動
    while let Some(mv) = ss.next() {
        let do_swap = if mv == "1" {
            true
        } else if mv == "0" {
            false
        } else {
            return Err(format!("Invalid action (swap=?): {}", mv));
        };
        let dir1 = read(ss.next(), '.'..='Z')?;
        let dir2 = read(ss.next(), '.'..='Z')?;

        // '.' は !0 (存在しない添字) で表現し、U/D/L/R は対応するDIRSのindexに変換
        let dir1 = if dir1 == '.' {
            !0
        } else if let Some(d1) = DIRS.iter().position(|&c| c == dir1) {
            d1
        } else {
            return Err(format!("Invalid move direction: {}", dir1));
        };
        let dir2 = if dir2 == '.' {
            !0
        } else if let Some(d2) = DIRS.iter().position(|&c| c == dir2) {
            d2
        } else {
            return Err(format!("Invalid move direction: {}", dir2));
        };
        out.push((do_swap, dir1, dir2));
    }
    if out.len() > 4 * input.n * input.n {
        return Err("Too many actions".to_owned());
    }
    Ok(Output { start, out })
}

//=========== 一部固定のマップ (in_fixed/t.txt) ==========//
// テスト生成に使う
const FIXED: [&'static str; 20] = [
    include_str!("../in_fixed/0.txt"),
    include_str!("../in_fixed/1.txt"),
    include_str!("../in_fixed/2.txt"),
    include_str!("../in_fixed/3.txt"),
    include_str!("../in_fixed/4.txt"),
    include_str!("../in_fixed/5.txt"),
    include_str!("../in_fixed/6.txt"),
    include_str!("../in_fixed/7.txt"),
    include_str!("../in_fixed/8.txt"),
    include_str!("../in_fixed/9.txt"),
    include_str!("../in_fixed/10.txt"),
    include_str!("../in_fixed/11.txt"),
    include_str!("../in_fixed/12.txt"),
    include_str!("../in_fixed/13.txt"),
    include_str!("../in_fixed/14.txt"),
    include_str!("../in_fixed/15.txt"),
    include_str!("../in_fixed/16.txt"),
    include_str!("../in_fixed/17.txt"),
    include_str!("../in_fixed/18.txt"),
    include_str!("../in_fixed/19.txt"),
];

//=========== 入力生成 (gen) ==========//

/// 与えられたseedに応じて、固定マップ + ランダムに並べた a を生成
pub fn gen(seed: u64) -> Input {
    let mut rng = rand_chacha::ChaCha20Rng::seed_from_u64(seed);

    // t は 0..19 で決まっている
    let ty = seed % 20;
    // まずは固定部分を読み込む
    let mut input = parse_input_fixed(FIXED[ty as usize]);

    // 1..=N^2 までの数字をランダムにシャッフルして a に割り当てる
    let mut nums = (1..=input.n * input.n).collect_vec();
    nums.shuffle(&mut rng);

    input.a = mat![0; input.n; input.n];
    for i in 0..input.n {
        for j in 0..input.n {
            input.a[i][j] = nums[i * input.n + j] as i32;
        }
    }
    input
}

//=========== スコア計算用の補助 ==========//

/// 入力トークンを任意の型 T にパースしてチェックする汎用関数
pub fn read<T: Copy + PartialOrd + std::fmt::Display + std::str::FromStr, R: RangeBounds<T>>(
    token: Option<&str>,
    range: R,
) -> Result<T, String> {
    if let Some(v) = token {
        if let Ok(parsed) = v.parse::<T>() {
            if !range.contains(&parsed) {
                Err(format!("Out of range: {}", parsed))
            } else {
                Ok(parsed)
            }
        } else {
            Err(format!("Parse error: {}", v))
        }
    } else {
        Err("Unexpected EOF".to_owned())
    }
}

/// U, D, L, R を文字で表す
const DIRS: [char; 4] = ['U', 'D', 'L', 'R'];

/// (di, dj): U->(-1,0), D->(1,0), L->(0,-1), R->(0,1) をusizeの計算で !0 を用いて実現
const DIJ: [(usize, usize); 4] = [
    (!0, 0), // U
    (1, 0),  // D
    (0, !0), // L
    (0, 1),  // R
];

//=========== 移動可否判定のヘルパー関数 ==========//

fn can_move(
    N: usize,
    h: &Vec<Vec<char>>,
    v: &Vec<Vec<char>>,
    i: usize,
    j: usize,
    dir: usize,
) -> bool {
    let (di, dj) = DIJ[dir];
    let i2 = i.wrapping_add(di);
    let j2 = j.wrapping_add(dj);
    if i2 >= N || j2 >= N {
        return false;
    }
    // 垂直方向の移動であれば hs[i or i2][j] を見る
    // 水平方向の移動であれば vs[i][j or j2] を見る
    if di == 0 {
        // 左右移動 => vs[i][min(j,j2)]
        v[i][j.min(j2)] == '0'
    } else {
        // 上下移動 => hs[min(i,i2)][j]
        h[i.min(i2)][j] == '0'
    }
}

//=========== スコア計算 ==========//

/// ある配置 a について、隣接マスの数字差の二乗和を計算
fn compute_diff(input: &Input, a: &Vec<Vec<i32>>) -> i64 {
    let mut diff = 0;
    for i in 0..input.n {
        for j in 0..input.n {
            // 下方向(DIR=1)と右方向(DIR=3)だけ見れば重複しない
            for dir in [1, 3] {
                if can_move(input.n, &input.hs, &input.vs, i, j, dir) {
                    let (ii, jj) = (i + DIJ[dir].0, j + DIJ[dir].1);
                    let d = (a[i][j] - a[ii][jj]) as i64;
                    diff += d * d;
                }
            }
        }
    }
    diff
}

/// スコアとエラーメッセージを返す (単純に行動が不正なら score=0, err!=0 など)
pub fn compute_score(input: &Input, out: &Output) -> (i64, String) {
    let (mut score, err, _) = compute_score_details(input, out.start, &out.out);
    // エラーがある場合は score=0 とする
    if !err.is_empty() {
        score = 0;
    }
    (score, err)
}

/// 部分的に行動を適用したときの状態(a, p1, p2)とスコア等を返す
fn compute_score_details(
    input: &Input,
    start: (usize, usize, usize, usize),
    actions: &[(bool, usize, usize)],
) -> (i64, String, (Vec<Vec<i32>>, (usize, usize), (usize, usize))) {
    // 盤面をコピー
    let mut a = input.a.clone();
    let mut p1 = (start.0, start.1);
    let mut p2 = (start.2, start.3);

    // 初期状態の差分
    let before = compute_diff(input, &a);

    // 行動を順番に適用
    for &(do_swap, dir1, dir2) in actions {
        // 1. 数字を交換
        if do_swap {
            let tmp = a[p1.0][p1.1];
            a[p1.0][p1.1] = a[p2.0][p2.1];
            a[p2.0][p2.1] = tmp;
        }
        // 2. 高橋君が移動
        if dir1 != !0 {
            if !can_move(input.n, &input.hs, &input.vs, p1.0, p1.1, dir1) {
                return (0, format!("Invalid move by T: {}", DIRS[dir1]), (a, p1, p2));
            }
            p1.0 = p1.0.wrapping_add(DIJ[dir1].0);
            p1.1 = p1.1.wrapping_add(DIJ[dir1].1);
        }
        // 3. 青木君が移動
        if dir2 != !0 {
            if !can_move(input.n, &input.hs, &input.vs, p2.0, p2.1, dir2) {
                return (0, format!("Invalid move by A: {}", DIRS[dir2]), (a, p1, p2));
            }
            p2.0 = p2.0.wrapping_add(DIJ[dir2].0);
            p2.1 = p2.1.wrapping_add(DIJ[dir2].1);
        }
    }

    // 最終状態の差分
    let after = compute_diff(input, &a);

    // スコア計算: max(1, round(1e6 * (log2(before) - log2(after))))
    // ただし before=0 のケースは本来起こり得ない(初期に必ず隣接差分は>0)と想定
    // 問題文に忠実に従うなら, after > 0 でログを取れるはず
    let score = if before == 0 || after == 0 {
        1
    } else {
        let val = (1e6 * (f64::log2(before as f64) - f64::log2(after as f64))).round() as i64;
        val.max(1)
    };

    (score, String::new(), (a, p1, p2))
}

//=========== ビジュアライザ ==========//

/// vis関数から呼び出される描画用矩形
use svg::node::element::{Line, Rectangle, Style, Text};
use svg::Node;

/// 矩形を作るためのヘルパー
fn rect(x: i32, y: i32, w: i32, h: i32, fill: &str) -> Rectangle {
    Rectangle::new()
        .set("x", x)
        .set("y", y)
        .set("width", w)
        .set("height", h)
        .set("fill", fill)
}

/// 壁を描画するためのヘルパー (線分)
fn line(x1: i32, y1: i32, x2: i32, y2: i32, stroke: &str, width: f32) -> Line {
    Line::new()
        .set("x1", x1)
        .set("y1", y1)
        .set("x2", x2)
        .set("y2", y2)
        .set("stroke", stroke)
        .set("stroke-width", width)
}

/// 赤系のグラデーション色を返す
///   val が大きいほど濃い赤, 小さいほど薄い赤
fn cell_color(val: i32, max_val: i32) -> String {
    // val / max_val が 0.0～1.0 となるようにして、そこから適当に RGBA を生成
    // 例: RGBA(255, 0, 0, alpha)
    // alpha を 0.2～1.0 の範囲で線形補間するなど
    let ratio = val as f32 / max_val as f32;
    let alpha = 0.2 + 0.8 * ratio; // 0.2～1.0 にマッピング
    format!("rgba(255,0,0,{:.3})", alpha)
}

/// ビジュアライザ: turn 回分の行動を適用した状態を描画する
/// 戻り値: (score, err, svg文字列)
pub fn vis(input: &Input, output: &Output, turn: usize) -> (i64, String, String) {
    // 部分的に行動を適用した状態を取得
    let (score, err, (a, p1, p2)) = compute_score_details(
        input,
        output.start,
        &output.out[..turn.min(output.out.len())],
    );

    // SVGサイズ
    let wcell = 20; // 1マスの幅
    let hcell = 20; // 1マスの高さ
    let W = (input.n as i32) * wcell;
    let H = (input.n as i32) * hcell;

    // SVG初期化
    let mut doc = svg::Document::new()
        .set("id", "vis")
        .set("viewBox", (0, 0, W, H))
        .set("width", W)
        .set("height", H)
        .set("style", "background-color:white");

    // テキストスタイル (数字の描画や T/A の表示)
    doc = doc.add(Style::new(format!(
        "text {{ text-anchor: middle; dominant-baseline: central; font-size: {}px; font-weight: bold; }}",
        10
    )));

    // 最大値 (色の濃淡に使う)
    let max_val = (input.n * input.n) as i32;

    // 1. 各マスを描画 (塗りつぶし + 数字)
    for i in 0..input.n {
        for j in 0..input.n {
            let x = (j as i32) * wcell;
            let y = (i as i32) * hcell;
            let val = a[i][j];

            // 塗りつぶし
            let color = cell_color(val, max_val);
            let r = rect(x, y, wcell, hcell, &color);
            doc = doc.add(r);

            // 数字(a[i][j])を表示
            let tx = x + wcell / 2;
            let ty = y + hcell / 2;
            let text_num = Text::new("")
                .set("x", tx)
                .set("y", ty)
                .add(svg::node::Text::new(val.to_string()))
                .set("fill", "black");
            doc = doc.add(text_num);
        }
    }

    // 2. 壁を描画
    //    vs[i][j] == '1' なら (i,j)-(i,j+1) の間に壁(縦線)
    //    hs[i][j] == '1' なら (i,j)-(i+1,j) の間に壁(横線)
    for i in 0..input.n {
        for j in 0..(input.n - 1) {
            if input.vs[i][j] == '1' {
                let x1 = (j + 1) as i32 * wcell;
                let y1 = (i as i32) * hcell;
                let x2 = x1;
                let y2 = y1 + hcell;
                doc = doc.add(line(x1, y1, x2, y2, "black", 2.0));
            }
        }
    }
    for i in 0..(input.n - 1) {
        for j in 0..input.n {
            if input.hs[i][j] == '1' {
                let x1 = (j as i32) * wcell;
                let y1 = (i + 1) as i32 * hcell;
                let x2 = x1 + wcell;
                let y2 = y1;
                doc = doc.add(line(x1, y1, x2, y2, "black", 2.0));
            }
        }
    }

    // 3. 高橋君 (T), 青木君 (A) の現在位置を上書き
    //    (p1.0, p1.1) -> 'T'
    //    (p2.0, p2.1) -> 'A'
    {
        let (ti, tj) = p1;
        let x = (tj as i32) * wcell + wcell / 2;
        let y = (ti as i32) * hcell + hcell / 2;
        let text_t = Text::new("")
            .set("x", x)
            .set("y", y)
            .add(svg::node::Text::new("T"))
            .set("fill", "blue")
            .set("font-size", 14);
        doc = doc.add(text_t);
    }
    {
        let (ai, aj) = p2;
        let x = (aj as i32) * wcell + wcell / 2;
        let y = (ai as i32) * hcell + hcell / 2;
        let text_a = Text::new("")
            .set("x", x)
            .set("y", y)
            .add(svg::node::Text::new("A"))
            .set("fill", "green")
            .set("font-size", 14);
        doc = doc.add(text_a);
    }

    (score, err, doc.to_string())
}

fn main() {}
