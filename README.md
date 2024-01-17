![](https://github.com/rusty-image-color-analyzer/rusty-image-color-analyzer.github.io/blob/web/rusty.webp)

# Rusty Image color analyizer
A rust to wasm example for analyzing image colors client side in the browser with minial effort. 

## How to compile wasm pkg
```
cargo build --target wasm32-unknown-unknown
wasm-pack build --target web 
```
## How to test localy
```
python3 serve.py
```