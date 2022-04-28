<p align="center">
  <a href="https://scott-hamilton.mit-license.org/"><img alt="MIT License" src="https://img.shields.io/badge/License-MIT-525252.svg?labelColor=292929&logo=creative%20commons&style=for-the-badge" /></a>
  <a href="https://github.com/SCOTT-HAMILTON/qrup/actions"><img alt="Build Status" src="https://img.shields.io/github/workflow/status/SCOTT-HAMILTON/qrup/CI?logo=github-actions&style=for-the-badge" /></a>
</p>
<h1 align="center">qrup - Local P2P File transfer from your mobile to your computer</h1>

Transfer files over LAN from your mobile device to a computer by scanning a QR code without leaving the terminal.

 > qrup was inspired by [qrcp](https://github.com/claudiodangelis/qrcp) and is just a Rust rewrite of their receive file feature.

## How does it work?
![Screenshot](https://user-images.githubusercontent.com/24496705/165276600-92eb56da-1afc-416a-a74f-59034553084e.png)

`qrup` binds a web server to the address of your Wi-Fi network interface on port 27177 and creates a handler for it. The default handler serves a file upload page and exits the program when the transfer is completed.

The tool prints a QR code that encodes the text:

```
http://{address}:{port}
```

Most QR apps can detect URLs in decoded text and act accordingly (i.e. open the decoded URL with the default browser), so when the QR code is scanned the content will begin downloading by the mobile browser.

## Building
This project is configured with cargo.
```sh
cargo build --release
```

## Usage

```sh
qrup
```
## License
qrup is delivered as it is under the well known MIT License.

## Credits
 - [Upload Page by Julian-Nash](https://gist.github.com/Julian-Nash/e94e181621e41f002c5848e2787c3a36)

**References that helped**
 - [poem's doc] : <https://docs.rs/poem/1.3.29/poem/>
 - [poem's examples] : <https://github.com/poem-web/poem/tree/master/examples>
 - [tokio's doc] : <https://docs.rs/tokio/latest/tokio/>
 - [qrcode-rust's README] : <https://github.com/kennytm/qrcode-rust>

[//]: # (These are reference links used in the body of this note and get stripped out when the markdown processor does its job. There is no need to format nicely because it shouldn't be seen. Thanks SO - http://stackoverflow.com/questions/4823468/store-comments-in-markdown-syntax)

   [poem's doc]: <https://docs.rs/poem/1.3.29/poem/>
   [poem's examples]: <https://github.com/poem-web/poem/tree/master/examples>
   [tokio's doc]: <https://docs.rs/tokio/latest/tokio/>
   [qrcode-rust's README]: <https://github.com/kennytm/qrcode-rust>


