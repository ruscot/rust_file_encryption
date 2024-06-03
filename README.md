# rust_file_encryption

This tool can be used to encrypt file contents with a password using AES128.

## How to use this tool
Command line arguments
Encrypt file content
```shell
cargo run mypassword src/test.txt encrypt
```
Decrypt file content
```shell
cargo run mypassword src/test.txt decrypt
```