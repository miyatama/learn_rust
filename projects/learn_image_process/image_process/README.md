# outline

Rustの画像処理学習用プロジェクト

## [Rustによる画像処理の基礎と応用](https://www.amazon.co.jp/Rust%E3%81%AB%E3%82%88%E3%82%8B%E7%94%BB%E5%83%8F%E5%87%A6%E7%90%86%E3%81%AE%E5%9F%BA%E7%A4%8E%E3%81%A8%E5%BF%9C%E7%94%A8-%E3%82%AB%E3%83%96%E3%83%88-ebook/dp/B0CQVNX6FD)

利用crateバージョン

+ image: 0.23.14

フォント利用

+ [しっぽり明朝](https://fontdasu.com/shippori-mincho/)

## will

+ [ ] [imageproc](https://docs.rs/imageproc/0.25.0/imageproc/index.html)を使いこなす
  + [x] binary_descriptors 使い方が謎
  + [x] contours のエラー対応
  + [x] template_matching の結果が謎

# imageproc

全モジュールのfunctionsを呼び出す

| module | progress | description |
| :----- | :-----: | :----- |
| binary_descriptors | completed | 呼び出せたけど結果が謎 |
| contours | completed |  |
| contrast | completed | |
| corners | completed | |
| definitions | none | functions定義なし |
| distance_transform | completed | |
| drawing | completed | |
| edges | completed | functionsはcannyのみ |
| filter | completed | |
| geometric_transformations | completed | |
| geometry | completed | |
| gradients | completed | |
| haar | completed | |
| hog | completed | 諸々謎 |
| hough | completed | |
| integral_image | completed | 諸々謎 |
| local_binary_patterns | completed | |
| map | completed | |
| math | completed | |
| morphology | completed  | |
| noise | completed | |
| pixelops | completed | |
| point | none | functions定義なし |
| property_testing | none | functions定義なし |
| rect | none | functions定義なし |
| region_labelling | completed | |
| seam_carving | completed | |
| stats | completed | |
| suppress | completed | |
| template_matching | complete | 本家exampleを見ても使い処が謎 |
| union_find | none | functions定義なし |
| utils | completed | |

## imageproc::contours::find_contours()のエラー

https://docs.rs/imageproc/0.25.0/imageproc/contours/fn.find_contours.html

```text
thread 'main' panicked at C:\Users\nmiya\.cargo\registry\src\index.crates.io-6f17d22bba15001f\imageproc-0.25.0\src\contours.rs:151:59:
called `Option::unwrap()` on a `None` value
```

[num::cast](https://docs.rs/num/latest/num/cast/index.html)でNoneが返っている

imageprocのコードで確認した結果、下記コードの`u8`指定が問題と発覚。`u32`なら通る。

```rust
let result = imageproc::contours::find_contours::<u8>(&img_gray);
```


# トラブルシューティング

## ImageBuffter作成時の型エラー

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


## マルチスレッド化のエラー

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

## imageproc::filter::filter3x3

[定義](https://docs.rs/imageproc/0.25.0/imageproc/filter/fn.filter3x3.html)

```rust
pub fn filter3x3<P, K, S>(
    image: &Image<P>,
    kernel: &[K],
) -> Image<ChannelMap<P, S>>
where
    P::Subpixel: Into<K>,
    S: Clamp<K> + Primitive,
    P: WithChannel<S>,
    K: Num + Copy,
```

kernelの`Num + Copy` is 何？ -> とりあえず`&vec![100, 110, 130]`で指定してみる。

```rust
error[E0308]: mismatched types
   --> src\use_image_proc\filter.rs:17:51
    |
17  |     let img_result = imageproc::filter::filter3x3(&img, &vec![100, 110, 130]);
    |                      ---------------------------- ^^^^ expected `&ImageBuffer<_, Vec<_>>`, found `&DynamicImage`
    |                      |
    |                      arguments to this function are incorrect
    |
    = note: expected reference `&ImageBuffer<_, Vec<_>>`
               found reference `&DynamicImage`
```

[ImageBuffer](https://docs.rs/image/latest/image/struct.ImageBuffer.html)が必要。imgは[image::open()](https://docs.rs/image/latest/image/fn.open.html)で作成してるのでDynamicImage。[into_rgba16()](https://docs.rs/image/latest/image/enum.DynamicImage.html#method.into_rgba16)を使う。

下記の通り記述を変更

```rust
    let k = [
        1.0f32 / 16.0f32,
        2.0f32 / 16.0f32,
        1.0f32 / 16.0f32,
        2.0f32 / 16.0f32,
        4.0f32 / 16.0f32,
        2.0f32 / 16.0f32,
        1.0f32 / 16.0f32,
        2.0f32 / 16.0f32,
        1.0f32 / 16.0f32,
    ];
    let image_buffer = img.into_rgb16();
    let _img_result = imageproc::filter::filter3x3(&image_buffer, &k);
```

謎エラー発生

```rust
error[E0283]: type annotations needed
   --> src\use_image_proc\filter.rs:29:23
    |
29  |     let _img_result = imageproc::filter::filter3x3(&image_buffer, &k);
    |                       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ cannot infer type of the type parameter `S` declared on the function `filter3x3`
    |
    = note: multiple `impl`s satisfying `_: Clamp<f32>` found in the `imageproc` crate:
            - impl Clamp<f32> for f32;
            - impl Clamp<f32> for u16;
            - impl Clamp<f32> for u8;
note: required by a bound in `imageproc::filter::filter3x3`
   --> C:\Users\nmiya\.cargo\registry\src\index.crates.io-6f17d22bba15001f\imageproc-0.25.0\src\filter\mod.rs:349:8
    |
346 | pub fn filter3x3<P, K, S>(image: &Image<P>, kernel: &[K]) -> Image<ChannelMap<P, S>>
    |        --------- required by a bound in this function
...
349 |     S: Clamp<K> + Primitive,
    |        ^^^^^^^^ required by this bound in `filter3x3`
help: consider specifying the generic arguments
    |
29  |     let _img_result = imageproc::filter::filter3x3::<Rgb<u16>, f32, S>(&image_buffer, &k);
    |                                                   ++++++++++++++++++++
```

とりあえず`S: Clamp<K> + Primitive,` & `K = f32`なので、

+ [Clamp](https://docs.rs/imageproc/0.25.0/imageproc/definitions/trait.Clamp.html)
+ [Primitive](https://docs.rs/image/0.25.0/image/trait.Primitive.html)

Primitive -> u8, u16, f32
Clamp -> f32

とりあえずf32で合わせればよさげ。

```rust
    let image_buffer = img.into_rgb32f();
    // type: image::buffer_::ImageBuffer<image::color::Rgb<f32>, alloc::vec::Vec<f32>>
    let filter_result =
        imageproc::filter::filter3x3::<image::Rgb<f32>, f32, f32>(&image_buffer, &kernel);
    let filter_result = image::DynamicImage::ImageRgb32F(filter_result);
    filter_result
        .into_rgb8()
        .save("filter3x3_result.png")
        .expect("failed to save filter3x3 image");
```

## imageprocの挙動確認

`cargo test`たたいたらエラー

```text
$ cargo test
   Compiling imageproc v0.26.0 (C:\work\001_rust\imageproc)
error[E0554]: `#![feature]` may not be used on the stable release channel
 --> src\lib.rs:3:19
  |
3 | #![cfg_attr(test, feature(test))]
  |                   ^^^^^^^^^^^^^

For more information about this error, try `rustc --explain E0554`.
error: could not compile `imageproc` (lib test) due to 1 previous error
warning: build failed, waiting for other jobs to finish...
```

nightlyに切り替えて実行してみる

```shell
rustup install nightly
rustup run nightly rustc --version
rustup default nightly
```

+ [error[E0554]: #![feature] may not be used on the stable release channel Couldn't install racer using cargo](https://stackoverflow.com/questions/53136717/errore0554-feature-may-not-be-used-on-the-stable-release-channel-couldnt)
+ [rustupでrustをセットアップ](https://hnakamur.github.io/blog/2018/01/29/setup-rust-with-rustup/)

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


