<p align="center">
  <h1 align="center">Cairo VM Codes</h1>
</p>
<p align="center">
  <strong><i>An interactive reference to Cairo Virtual Machine</i></strong>
  <img width="1408" alt="screenshot" src="https://user-images.githubusercontent.com/5113/142245431-08ad9922-9115-43fd-9572-8b33cde75bb0.png">
</p>

This is the backend source code that runs [cairovm.codes](http://cairovm.codes) web application. Repository with the frontend code can be found [here](https://github.com/walnuthq/cairovm.codes). Below you will find the docs on how to contribute to the project and get it up and running locally for further development.

cairovm.codes is brought to you by [Walnut](https://www.walnut.dev).

## ⚙️ Installation

The app requires the following dependencies:

- [Rust](https://www.rust-lang.org/) >= 1.75.0

## 👩‍💻 Local Development

For contributing to the project, you can quickly get the application running by following these steps:

Clone this repository:

    git clone git@github.com:walnuthq/cairovm.codes-server.git

Install the dependencies:

    make deps

Start up the app and see it running at http://localhost:3000/_ah/warmup

    cargo run --bin server

## 🚀 Deploying

Deployments are handled automatically, as soon as your PR is merged to `main`.

## 🤗 Contributing

For instructions see [cairovm.codes](https://github.com/walnuthq/cairovm.codes)

## License

[MIT](LICENSE)
