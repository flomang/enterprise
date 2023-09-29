# coinbase 
A coinbase pro market feed

Usage: coinbase --market <MARKET>

Options:
  -m, --market <MARKET>  The market to connect to e.g. 'BTC=USD'
  -h, --help             Print help
  -V, --version          Print version


## Building
```
cargo build
```

## Running
From the workspace:
```
cargo run -p coinbase -- --market BTC-USD
```


From the project:
```
cargo run -- -m ETH-USD
```

