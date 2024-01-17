![](https://github.com/rusty-image-color-analyzer/rusty-image-color-analyzer.github.io/blob/web/rusty.webp)

# Rusty Image color analyizer
A rust to wasm example for analyzing image colors client side in the browser with minial effort. 

This application is a client side wasm tool that allows you to select an image and then create a color swatch of unique colors based on multple attributes such as hue, saturation and brightness. 

## How to compile wasm pkg
```
cargo build --target wasm32-unknown-unknown
wasm-pack build --target web 
```
## How to test localy
```
python3 serve.py
```
