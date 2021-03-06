#![allow(unused_imports)]
use proconio::{input, fastout, marker::Chars};
use std::cmp::*;

#[fastout]
fn main() {
    //! 外周が白と保証されているので
    //! 縦横二マスに注目して考える
    //! 1)
    //! .. .. ## #. .# ##
    //! .. ## .. #. .# ##
    //! この時真ん中の頂点は角にならない
    //! 
    //! 2)
    //! .. #. .# .. ## ## .# #.
    //! .# .. .. #. #. .# ## ##
    //! この時真ん中の頂点は角になる
    //! 
    //! 3)
    //! .# #.
    //! #. .#
    //! 自己交叉がないことが保証されているのでこれはない
    input!(h: usize, w: usize, s: [Chars; h]);
    let mut ans = 0;
    for y in 0..h-1 {
        for x in 0..w-1 {
            let mut dots = 0;
            let v = [s[y][x], s[y][x+1], s[y+1][x], s[y+1][x+1]];
            v.iter().for_each(|&v| if v == '.' { dots += 1; });
            match dots {
                1 | 3 => ans += 1,
                _ => {},
            };
        }
    }
    println!("{}", ans);
}
