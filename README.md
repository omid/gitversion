# gitversion

[![MIT licensed][mit-badge]][mit-url]

#### Basic Semantic Versioning for Git projects

The download size of the docker image is only about *30MB* and the actual size of the docker image is only around *80MB*.

[mit-badge]: https://img.shields.io/badge/license-MIT-blue.svg
[mit-url]: LICENSE-MIT

## Usage
#### Docker
_Note: change **DIR** to the Git project dir._
```bash
docker run -v DIR:/repo omidmr/gitversion gitversion --help
```

#### Manual
```bash
cargo install gitversion
gitversion --help
```

## License

This project is licensed under the [MIT license](LICENSE).
