# outline

Rustの画像処理学習用プロジェクト

## [Rustによる画像処理の基礎と応用](https://www.amazon.co.jp/Rust%E3%81%AB%E3%82%88%E3%82%8B%E7%94%BB%E5%83%8F%E5%87%A6%E7%90%86%E3%81%AE%E5%9F%BA%E7%A4%8E%E3%81%A8%E5%BF%9C%E7%94%A8-%E3%82%AB%E3%83%96%E3%83%88-ebook/dp/B0CQVNX6FD)

利用crateバージョン

+ image: 0.23.14

## will

+ [ ] [imageproc](https://docs.rs/imageproc/0.25.0/imageproc/index.html)を使いこなす

## トラブルシューティング

### ImageBuffter作成時の型エラー

```rust
let result_img = image::ImageBuffer::from_raw(width, height, result_pixels).unwrap();
// expected `ImageBuffer<Rgba<u8>, Vec<u8>>`, found `ImageBuffer<_, Vec<Rgba<u8>>>`
DynamicImage::ImageRgba8(result_img)
```

+ [ImageBuffer::from_raw](https://docs.rs/image/latest/image/struct.ImageBuffer.html#method.from_raw)
+ [ImageRgba8](https://docs.rs/image/latest/image/enum.DynamicImage.html#variant.ImageRgba8)
+ [RgbaImage](https://docs.rs/image/latest/image/type.RgbaImage.html)

RgbaImageは`pub type RgbaImage = ImageBuffer<Rgba<u8>, Vec<u8>>;`
from_rawの戻り値は`Option<ImageBuffer<P, Container>>`

解決策: from_rawに`Vec<u8>`を指定する


### マルチスレッド化のエラー

```rust
   |
42 | fn apply_filter_multi(img: DynamicImage, filter: [[f32; 3]; 3]) -> DynamicImage {
   |                       --- captured outer variable
...
46 |         .flat_map(|y| {
   |                   --- captured by this `FnMut` closure
...
49 |                 .map(move |x| apply_filter_at_pixel(&img, x, y, &filter))
   |                      ^^^^^^^^                        ---
   |                      |                               |
   |                      |                               variable moved due to use in closure
   |                      `img` is moved here             move occurs because `img` has type `DynamicImage`, which does not implement the `Copy` trait 
   |
help: clone the value before moving it into the closure
   |
49 ~                 .map({
50 +                 let value = img.clone();
51 ~                 move |x| apply_filter_at_pixel(&value, x, y, &filter)
52 ~                 })
   |

```

+ [Moved values and captured in Fn closure](https://users.rust-lang.org/t/moved-values-and-captured-in-fn-closure/38243)


ひとまず、rayon利用

```rust
use rayon::iter::IntoParallelIterator;
use rayon::iter::ParallelIterator;
use rayon::prelude::*;
```

+ [[Help]: Why is my concurrent code slower than the single-threaded version?](https://www.reddit.com/r/rust/comments/wtpix6/help_why_is_my_concurrent_code_slower_than_the/?rdt=64369)

pixel配列から生成するように変更(予定)


# reference

+ [Rustによる画像処理の基礎と応用](https://www.amazon.co.jp/Rust%E3%81%AB%E3%82%88%E3%82%8B%E7%94%BB%E5%83%8F%E5%87%A6%E7%90%86%E3%81%AE%E5%9F%BA%E7%A4%8E%E3%81%A8%E5%BF%9C%E7%94%A8-%E3%82%AB%E3%83%96%E3%83%88-ebook/dp/B0CQVNX6FD)
+ [Rustで画像処理 メリットと注意点](https://zenn.dev/kacky/articles/101fc3d90364f4)
+ crate
  + [image](https://crates.io/crates/image)
  + [imageproc](https://crates.io/crates/imageproc)
  + [rayon](https://crates.io/crates/rayon)
+ blog.ojisan.io
  + [Rust でモザイク加工を実装し、それを WebAssembly として Web アプリから利用する](https://blog.ojisan.io/rust-mosaic-web-app/)
+ [yavuzceliker / sample-images](https://github.com/yavuzceliker/sample-images)