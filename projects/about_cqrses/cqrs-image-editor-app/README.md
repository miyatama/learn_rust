# outline

cqrs-esを利用した画像加工アプリ

想定イベント

+ create canvas(size)
+ add image(position, scale, rotate)
+ update image(position, scale, rotate)
+ delete image
+ clip canvas(position, size)

提供するViewとしては、画像のyaml(or json)
画像自体のダウンロードは別口とする。