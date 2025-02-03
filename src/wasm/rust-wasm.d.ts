/* tslint:disable */
/* eslint-disable */

/*
Rust の関数を javascript から呼び出すインターフェイスとなる関数群。

これらの関数は問題ごとに新規に実装する必要があるため、型を合わせるために
テンプレとしては declare で宣言だけしておく。

wasm にコンパイルしたものを src/wasm/* に配置し、こいつは消す。
*/

// 

// seed から入力を生成する関数。
export declare function gen(seed: number): string;

// 入力(_input), 出力(_output), ターン(turn) から、
// - スコアを計算
// - (エラーが発生した場合は) エラー文
// - 出力をビジュアライズした画像(svg)
// を返す関数。
export declare function vis(_input: string, _output: string, turn: number): Ret;

export declare class Ret {
  private constructor();
  free(): void;
  score: bigint;
  err: string;
  svg: string;
};

// 入力(_input), 出力(_output) から出力が全部で何ターンあるかを返す関数。 
export declare function get_max_turn(_input: string, _output: string): number;

// 上記関数を呼び出す前に呼ぶ必要がある関数
// 正直良くわかってないが、こいつを先に呼び出しておかないとRustの関数を呼び出せない

export declare function init(module_or_path?: any): Promise<any>;