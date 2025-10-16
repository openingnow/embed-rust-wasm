Embed the WASM file in HTML to simplify distribution.

```sh
wasm-pack build --target web
python -c 'print(open("index_template.html").read().replace("`GLUE-JS`;",open("pkg/rf.js").read()))' > index.html
sed -i "s|GZIP-BASE64|$(gzip -c pkg/rf_bg.wasm | base64 -w 0)|" index.html
```