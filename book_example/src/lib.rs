// src/lib.rs がライブラリクレートのデフォルトのルートとなる。
// src/chapter7_2 ディレクトリ内のモジュールを読み込むには、ここでネストする必要がある。
// 仕組みについて詳しくは chapter7_5
pub mod chapter7_2 {
    pub mod garden;
}

pub mod chapter7_3 {
    pub mod restaurant;
}

pub mod chapter7_4 {
    pub mod restaurant;
}

// src/chapter7_5.rs を読み込む。
pub mod chapter7_5;

pub mod chapter11_1;
pub mod chapter11_3;
pub mod chapter12_3;
pub mod chapter12_4;
pub mod chapter12_5;
pub mod chapter13_3;
pub mod chapter14_2;
