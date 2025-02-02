あなたにAtCoder Heuristic Contestのビジュアライザ・入力ジェネレーターの作成をお願いしたいです。
システムはReact + Rustによるwasmで構成されていて、概ね以下のような担当分けになっています:
React側: seed値・outputをtextareaから受け付けて、Rustに送る・Rustから受け取った入力ファイルをTextAreaに表示・Rustから受け取ったsvgを表示
Rust側: Reactから渡されたものに対して処理を行う: 
具体的には、
- seedの値に基づいて入力ファイルの作成
- 与えられた出力に基づいてビジュアライザの作成(svgの描画)、ターンごと
- 入力・出力を受け取って、最大のターン数を返す
ことを行なっています。
以下のコードはRust側の例で、インターフェースを変えずに(つまり、lib.rsの内容をほぼ変えずに)、別のコンテスト用のビジュアライザシステムの開発を行いたいです:

```rust
use wasm_bindgen::prelude::*;
mod util;

#[wasm_bindgen]
pub fn gen(seed: i32) -> String {
    util::gen(seed as u64).to_string()
}

#[wasm_bindgen(getter_with_clone)]
pub struct Ret {
    pub score: i64,
    pub err: String,
    pub svg: String,
}

#[wasm_bindgen]
pub fn vis(_input: String, _output: String, turn: usize) -> Ret {
    let input = util::parse_input(&_input);
    let output = util::parse_output(&_output);
    let (score, err, svg) = util::vis(&input, &output, turn);
    Ret {
        score: score as i64,
        err,
        svg,
    }
}

#[wasm_bindgen]
pub fn get_max_turn(_input: String, _output: String) -> usize {
    let output = util::parse_output(&_output);
    output.q
}

[util.rs]
#![allow(non_snake_case, unused_macros)]
use proconio::input;
use rand::prelude::*;
use std::collections::VecDeque;
use svg::node::element::{Rectangle, Style};
use web_sys::console::log_1;

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

#[derive(Clone, Debug)]
pub struct Input {
    pub id: usize,
    pub n: usize,
    pub k: usize,
    pub s: Vec<String>,
}

impl std::fmt::Display for Input {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{} {} {}", self.id, self.n, self.k)?;
        for i in 0..self.n {
            writeln!(f, "{}", self.s[i])?;
        }
        Ok(())
    }
}

pub fn parse_input(f: &str) -> Input {
    let f = proconio::source::once::OnceSource::from(f);
    input! {
        from f,
        id:usize,
        n: usize,
        k: usize,
        s: [String; n]
    }
    Input { id, n, k, s }
}

pub struct Output {
    pub q: usize,
    pub yxc: Vec<(usize, usize, usize)>,
}

pub fn parse_output(f: &str) -> Output {
    let f = proconio::source::once::OnceSource::from(f);
    input! {
        from f,
        q: usize,
        yxc: [(usize, usize, usize); q]
    }
    Output { q, yxc }
}

pub fn gen(seed: u64) -> Input {
    let mut rng = rand_chacha::ChaCha20Rng::seed_from_u64(seed);
    let id = seed;
    let n = 100;
    let k = 9;
    let s = (0..n)
        .map(|_| {
            (0..n)
                .map(|_| rng.gen_range(1..k + 1).to_string())
                .collect::<String>()
        })
        .collect::<Vec<_>>();
    Input { id: 0, n, k, s }
}

fn calculate_score(input: &Input, yxc: &Vec<(usize, usize, usize)>) -> (usize, Vec<Vec<usize>>) {
    let mut state = vec![vec![0; input.n]; input.n];
    input.s.iter().enumerate().for_each(|(y, s)| {
        s.chars()
            .enumerate()
            .for_each(|(x, c)| state[y][x] = c.to_digit(10).unwrap() as usize)
    });

    let x_vec: Vec<i32> = vec![0, 1, 0, -1];
    let y_vec: Vec<i32> = vec![-1, 0, 1, 0];

    for (y, x, c) in yxc {
        // state[*y][*x] = *c;
        let selected_color = state[*y - 1][*x - 1];

        let mut visited = vec![vec![false; input.n]; input.n];
        let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
        queue.push_back((*y - 1, *x - 1));

        let mut count = 0;

        while queue.len() > 0 {
            let (ypos, xpos) = queue.pop_front().unwrap();
            if visited[ypos][xpos] {
                continue;
            }
            visited[ypos][xpos] = true;
            state[ypos][xpos] = *c;

            count = count + 1;
            for i in 0..4 {
                let nx = xpos as i32 + x_vec[i];
                let ny = ypos as i32 + y_vec[i];
                if nx < 0 || ny < 0 || nx >= input.n as i32 || ny >= input.n as i32 {
                    continue;
                }
                let nx = nx as usize;
                let ny = ny as usize;
                if visited[ny][nx] {
                    continue;
                }

                if state[ny][nx] != selected_color {
                    continue;
                }
                queue.push_back((ny, nx));
            }
        }
    }

    let mut score = 0;
    for color in 1..(input.k + 1) {
        let mut tmp_score = 0;
        for y in 0..input.n {
            for x in 0..input.n {
                if state[y][x] == color {
                    tmp_score += 100;
                }
            }
        }
        score = score.max(tmp_score);
    }
    score -= yxc.len();

    return (score, state);
}

fn generate_dark_color(code: usize) -> String {
    // 入力値に基づいてHue（色相）を計算
    let hue = (code as f32 * 36.0) % 360.0;

    // Saturation（彩度）を低めに、Lightness（明度）を固定値で低く設定
    let saturation = 30.0;
    let lightness = 30.0;

    // HSL to RGB 変換
    let hue_normalized = hue / 360.0;
    let q = if lightness < 0.5 {
        lightness * (1.0 + saturation)
    } else {
        lightness + saturation - (lightness * saturation)
    };

    let p = 2.0 * lightness - q;

    let r = hue_to_rgb(p, q, hue_normalized + 1.0 / 3.0);
    let g = hue_to_rgb(p, q, hue_normalized);
    let b = hue_to_rgb(p, q, hue_normalized - 1.0 / 3.0);

    // RGB を 16 進数に変換して文字列を返す
    format!(
        "#{:02X}{:02X}{:02X}",
        (r * 255.0) as u8,
        (g * 255.0) as u8,
        (b * 255.0) as u8
    )
}

fn generate_color(code: usize) -> String {
    // 入力値に基づいてHue（色相）を計算
    let hue = (code as f32 * 36.0) % 360.0;

    // Saturation（彩度）とLightness（明度）を固定値で設定
    let saturation = 10.0;
    let lightness = 0.1;

    // HSL to RGB 変換
    let hue_normalized = hue / 360.0;
    let q = if lightness < 0.5 {
        lightness * (1.0 + saturation)
    } else {
        lightness + saturation - (lightness * saturation)
    };

    let p = 2.0 * lightness - q;

    let r = hue_to_rgb(p, q, hue_normalized + 1.0 / 3.0);
    let g = hue_to_rgb(p, q, hue_normalized);
    let b = hue_to_rgb(p, q, hue_normalized - 1.0 / 3.0);

    // RGB を 16 進数に変換して文字列を返す
    format!(
        "#{:02X}{:02X}{:02X}",
        (r * 255.0) as u8,
        (g * 255.0) as u8,
        (b * 255.0) as u8
    )
}

fn hue_to_rgb(p: f32, q: f32, t: f32) -> f32 {
    let t = if t < 0.0 {
        t + 1.0
    } else if t > 1.0 {
        t - 1.0
    } else {
        t
    };

    if t < 1.0 / 6.0 {
        p + (q - p) * 6.0 * t
    } else if t < 1.0 / 2.0 {
        q
    } else if t < 2.0 / 3.0 {
        p + (q - p) * (2.0 / 3.0 - t) * 6.0
    } else {
        p
    }
}

pub fn rect(x: usize, y: usize, w: usize, h: usize, fill: &str) -> Rectangle {
    Rectangle::new()
        .set("x", x)
        .set("y", y)
        .set("width", w)
        .set("height", h)
        .set("fill", fill)
}

pub fn vis(input: &Input, output: &Output, turn: usize) -> (i64, String, String) {
    let (score, state) =
        calculate_score(input, &output.yxc[0..turn].into_iter().cloned().collect());

    let W = 800;
    let H = 800;
    let w = 8;
    let h = 8;
    let mut doc = svg::Document::new()
        .set("id", "vis")
        .set("viewBox", (-5, -5, W + 10, H + 10))
        .set("width", W + 10)
        .set("height", H + 10)
        .set("style", "background-color:white");

    doc = doc.add(Style::new(format!(
        "text {{text-anchor: middle;dominant-baseline: central; font-size: {}}}",
        6
    )));
    for y in 0..input.n {
        for x in 0..input.n {
            doc = doc.add(
                rect(
                    x * w,
                    W - (y + 1) * h,
                    w,
                    h,
                    &generate_dark_color(state[y][x]),
                )
                .set("stroke", "black")
                .set("stroke-width", 1)
                .set("class", "box"),
            );
        }
    }

    (score as i64, "".to_string(), doc.to_string())
}
```

上記の情報を参考にして、この次に与えるAtCoder Heuristic Contestの問題のビジュアライザのためのutil.rsを書いてください。
ただし、元々のutil.rsの構造を大きく変えないで欲しいです:
- Input, Output構造体を作る
- Input,Outputに実装したトレイトは必ず実装する(特にDisplayを忘れがち)
- 適切にコメントを入れる
- 入力生成方法は簡易化せずに厳密に指定に従う必要があります
- 同じlib.rsを使うので、util.rsのインターフェースを変えることは禁止
- Rustのクレートは以下のバージョンのものを使用する:
wasm-bindgen = "0.2.89"
getrandom = {version="0.2", features=["js"]}
rand = { version = "=0.8.5", features = ["small_rng", "min_const_gen"] }
rand_chacha = "=0.3.1"
rand_distr = "=0.4.3"
itertools = "=0.11.0"
proconio = { version = "=0.4.5", features = ["derive"] }
clap = { version = "4.0.22", features = ["derive"] }
svg = "0.17.0"
delaunator = "1.0.1"
web-sys = {"version" = "0.3.44", features=['console']}

ただし、以下のコードを踏襲してInput, Output, genなどを書いてください。

## ツール類
```rust
#![allow(non_snake_case, unused_macros)]

use itertools::Itertools;
use proconio::{input, marker::Chars};
use rand::prelude::*;
use std::ops::RangeBounds;

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

#[derive(Clone, Debug)]
pub struct Input {
    pub ty: u64,
    pub n: usize,
    pub a: Vec<Vec<i32>>,
    pub vs: Vec<Vec<char>>,
    pub hs: Vec<Vec<char>>,
}

impl std::fmt::Display for Input {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{} {}", self.ty, self.n)?;
        for i in 0..self.n {
            writeln!(f, "{}", self.vs[i].iter().collect::<String>())?;
        }
        for i in 0..self.n - 1 {
            writeln!(f, "{}", self.hs[i].iter().collect::<String>())?;
        }
        for i in 0..self.n {
            writeln!(f, "{}", self.a[i].iter().join(" "))?;
        }
        Ok(())
    }
}

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

pub fn parse_input_fixed(f: &str) -> Input {
    let f = proconio::source::once::OnceSource::from(f);
    input! {
        from f,
        ty: u64, n: usize,
        vs: [Chars; n],
        hs: [Chars; n - 1],
    }
    for i in 0..n {
        assert_eq!(vs[i].len(), n - 1);
    }
    for i in 0..n - 1 {
        assert_eq!(hs[i].len(), n);
    }
    Input {
        ty,
        n,
        a: vec![],
        vs,
        hs,
    }
}

pub fn read<T: Copy + PartialOrd + std::fmt::Display + std::str::FromStr, R: RangeBounds<T>>(
    token: Option<&str>,
    range: R,
) -> Result<T, String> {
    if let Some(v) = token {
        if let Ok(v) = v.parse::<T>() {
            if !range.contains(&v) {
                Err(format!("Out of range: {}", v))
            } else {
                Ok(v)
            }
        } else {
            Err(format!("Parse error: {}", v))
        }
    } else {
        Err("Unexpected EOF".to_owned())
    }
}

const DIRS: [char; 4] = ['U', 'D', 'L', 'R'];
const DIJ: [(usize, usize); 4] = [(!0, 0), (1, 0), (0, !0), (0, 1)];

pub struct Output {
    pub start: (usize, usize, usize, usize),
    pub out: Vec<(bool, usize, usize)>,
}

pub fn parse_output(input: &Input, f: &str) -> Result<Output, String> {
    let mut out = vec![];
    let mut ss = f.split_whitespace();
    let start = (
        read(ss.next(), 0..input.n)?,
        read(ss.next(), 0..input.n)?,
        read(ss.next(), 0..input.n)?,
        read(ss.next(), 0..input.n)?,
    );
    while let Some(mv) = ss.next() {
        let do_swap = if mv == "1" {
            true
        } else if mv != "0" {
            return Err(format!("Invalid action: {}", mv));
        } else {
            false
        };
        let dir1 = read(ss.next(), '.'..='Z')?;
        let dir2 = read(ss.next(), '.'..='Z')?;
        let dir1 = if dir1 == '.' {
            !0
        } else if let Some(dir1) = DIRS.iter().position(|&d| d == dir1) {
            dir1
        } else {
            return Err(format!("Invalid direction: {}", dir1));
        };
        let dir2 = if dir2 == '.' {
            !0
        } else if let Some(dir2) = DIRS.iter().position(|&d| d == dir2) {
            dir2
        } else {
            return Err(format!("Invalid direction: {}", dir2));
        };
        out.push((do_swap, dir1, dir2));
    }
    if out.len() > 4 * input.n * input.n {
        return Err("Too many actions".to_owned());
    }
    Ok(Output { start, out })
}

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

pub fn gen(seed: u64) -> Input {
    let mut rng = rand_chacha::ChaCha20Rng::seed_from_u64(seed);
    let ty = seed % 20;
    let mut input = parse_input_fixed(FIXED[ty as usize]);
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

fn can_move(N: usize, h: &Vec<Vec<char>>, v: &Vec<Vec<char>>, i: usize, j: usize, dir: usize) -> bool {
    let (di, dj) = DIJ[dir];
    let i2 = i + di;
    let j2 = j + dj;
    if i2 >= N || j2 >= N {
        return false;
    }
    if di == 0 {
        v[i][j.min(j2)] == '0'
    } else {
        h[i.min(i2)][j] == '0'
    }
}

pub fn compute_score(input: &Input, out: &Output) -> (i64, String) {
    let (mut score, err, _) = compute_score_details(input, out.start, &out.out);
    if err.len() > 0 {
        score = 0;
    }
    (score, err)
}

fn compute_diff(input: &Input, a: &Vec<Vec<i32>>) -> i64 {
    let mut diff = 0;
    for i in 0..input.n {
        for j in 0..input.n {
            for dir in 1..=2 {
                if can_move(input.n, &input.hs, &input.vs, i, j, dir) {
                    let d = (a[i][j] - a[i + DIJ[dir].0][j + DIJ[dir].1]) as i64;
                    diff += d * d;
                }
            }
        }
    }
    diff
}

pub fn compute_score_details(
    input: &Input,
    start: (usize, usize, usize, usize),
    out: &[(bool, usize, usize)],
) -> (i64, String, (Vec<Vec<i32>>, (usize, usize), (usize, usize))) {
    let mut a = input.a.clone();
    let mut p1 = (start.0, start.1);
    let mut p2 = (start.2, start.3);
    let before = compute_diff(&input, &a);
    for &(do_swap, dir1, dir2) in out {
        if do_swap {
            let tmp = a[p1.0][p1.1];
            a[p1.0][p1.1] = a[p2.0][p2.1];
            a[p2.0][p2.1] = tmp;
        }
        if dir1 != !0 {
            if !can_move(input.n, &input.hs, &input.vs, p1.0, p1.1, dir1) {
                return (0, format!("Invalid move: {}", DIRS[dir1]), (a, p1, p2));
            }
            p1.0 += DIJ[dir1].0;
            p1.1 += DIJ[dir1].1;
        }
        if dir2 != !0 {
            if !can_move(input.n, &input.hs, &input.vs, p2.0, p2.1, dir2) {
                return (0, format!("Invalid move: {}", DIRS[dir2]), (a, p1, p2));
            }
            p2.0 += DIJ[dir2].0;
            p2.1 += DIJ[dir2].1;
        }
    }
    let after = compute_diff(&input, &a);
    let score = ((1e6 * (f64::log2(before as f64) - f64::log2(after as f64))).round() as i64).max(1);
    (score, String::new(), (a, p1, p2))
}

```

## 問題文
問題文
N×N マスの盤面がある。 一番左上のマスの座標を 
(0,0) とし、そこから下方向に 
i マス、右方向に 
j マス進んだ先のマスの座標を 
(i,j) とする。 
N×N マスの外周は壁で囲われており、マス間にも壁がある場合がある。 辺を共有する2マスは、間に壁がない時に「隣接マス」であると定義する。(13:15に追記)

各マス 
(i,j) には 
1 から 
N 
2
  の数字 
a 
(i,j)
​
  がそれぞれ1つずつ書かれている。 高橋君と青木君の二人は、グリッド上の好きな初期位置からスタートし、以下の一連の行動を最大で 
4N 
2
  回行う。

高橋君と青木君の現在位置に書かれている数字を交換したいならば交換する。
高橋君は自身の現在位置に隣接するマスへ移動したいならば移動する。
青木君は自身の現在位置に隣接するマスへ移動したいならば移動する。
一連の行動は 
1→2→3 の順で行われ、全体で1回の行動とみなす。 数字を交換せずに隣接マスへ移動してもよいし、数字の交換後に移動せず同じ位置に留まっても良い。 二人の現在位置が同じマスとなっても構わない。

最終的に隣接するマス間の数字が出来るだけ近くなるように二人の行動を決定して欲しい。

得点
隣接するマスのペア全体の集合を 
E とし、隣接マスの数字の差の二乗和 
∑ 
u,v∈E
​
 (a 
u
​
 −a 
v
​
 ) 
2
  を考える。 初期状態における二乗和の値を 
D 
′
 、最終状態における二乗和の値を 
D としたとき、以下の得点が得られる。

max(1,round(10 
6
 ×log 
2
​
  
D
D 
′
 
​
 ))

下記で述べる 
t の値毎に 
10 個ずつ、合計で 200 個のテストケースがあり、各テストケースの得点の合計が提出の得点となる。 
t の値毎に別のテストセット(test_t)に別れており、不正な出力や制限時間超過をした場合、対応するテストセットのみが 
0 点となる。 コンテスト時間中に得た最高得点で最終順位が決定され、コンテスト終了後のシステムテストは行われない。 同じ得点を複数の参加者が得た場合、提出時刻に関わらず同じ順位となる。

入力
入力は以下の形式で標準入力から与えられる。

t 
N
v 
0,0
​
 ⋯v 
0,N−2
​
 
⋮
v 
N−1,0
​
 ⋯v 
N−1,N−2
​
 
h 
0,0
​
 ⋯h 
0,N−1
​
 
⋮
h 
N−2,0
​
 ⋯h 
N−2,N−1
​
 
a 
0,0
​
  
⋯ 
a 
0,N−1
​
 
⋮
a 
N−1,0
​
  
⋯ 
a 
N−1,N−1
​
 
t は入力の生成方法を表す整数で、
0≤t≤19 を満たす。詳細は入力生成方法の欄を参照せよ。
盤面の大きさ 
N は 
10≤N≤100 を満たす。
v 
i,0
​
 ⋯v 
i,N−2
​
  は 0 と 1 からなる 
N−1 文字の文字列であり、
v 
i,j
​
 =1 であればマス 
(i,j) とその右隣のマス 
(i,j+1) の間に壁があり、
v 
i,j
​
 =0 であれば壁がないことを表す。
h 
i,0
​
 ⋯h 
i,N−1
​
  は 0 と 1 からなる 
N 文字の文字列であり、
h 
i,j
​
 =1 であればマス 
(i,j) とその下隣のマス 
(i+1,j) の間に壁があり、
h 
i,j
​
 =0 であれば壁がないことを表す。
全てのマスは互いに到達可能であることが保証されている。
a 
i,j
​
  は初期状態でマス 
(i,j) に書かれている数字を表し、
1≤a 
i,j
​
 ≤N 
2
  を満たす。
出力
まず初めに、高橋君の初期位置 
(p 
i
​
 ,p 
j
​
 ) と青木君の初期位置 
(q 
i
​
 ,q 
j
​
 ) を決め、以下の形式で一行で標準出力に出力せよ。

p 
i
​
  
p 
j
​
  
q 
i
​
  
q 
j
​
 
初期位置は盤面の範囲内であれば、自由に決めて良い。

続いて、全体で 
k 回の行動を行う場合、1回の行動毎に以下の形式で1行に3文字を空白区切りで、合計 
k 行を標準出力に出力せよ。

s 
d 
e
s は数字の交換を行うかどうかを表す文字で、交換する場合は 1 、しない場合は 0 である。
d は高橋君の移動先を表す文字で、上下左右に隣接するマスに移動する場合は、それぞれ U D L R、移動せず現在位置にとどまる場合は . である。
e は青木君の移動先を表す文字で、高橋君の場合と同様である。
移動先との間に壁がある場合や、行動回数が 
4N 
2
  回を超えた場合は  と判定される。

入力生成方法
入力のうちの 
a を除いた部分は、入力を生成する seed 値を 20 で割った余り 
t に応じて固定であり、下記にリンクがある入力ジェネレータに含まれる in_fixed/t.txt がそのまま使われる。 テストケースには各 
t∈{0,1,⋯,19} の入力がそれぞれちょうど 10 個ずつ含まれる。

各マスの数字 
a 
i,j
​
  は 
1 から 
N 
2
  の値をランダムに並び替えることにより生成される。

## ビジュアライザの仕様
- N × N マスをグリッドで表現し、各マス (i, j) には数字 a_{(i, j)} を書く。
    - マスは基本的に赤色で塗る。マスに書かれている数字が大きいほど濃く、小さいほど薄い赤色で塗る。
- 壁が存在する場合は、該当する線を太く表現する。
- 高橋君の現在位置を"T"、青木君の現在位置を"A"とマスに上書きする。適切に透明度を付けて上書きする。
- ターンは1回の行動(1, 2, 3)を表現する。