# outline

Rustの画像処理学習用プロジェクト

## [Rustによる画像処理の基礎と応用](https://www.amazon.co.jp/Rust%E3%81%AB%E3%82%88%E3%82%8B%E7%94%BB%E5%83%8F%E5%87%A6%E7%90%86%E3%81%AE%E5%9F%BA%E7%A4%8E%E3%81%A8%E5%BF%9C%E7%94%A8-%E3%82%AB%E3%83%96%E3%83%88-ebook/dp/B0CQVNX6FD)

利用crateバージョン

+ image: 0.23.14

### トラブルシューティング

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