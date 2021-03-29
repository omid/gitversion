# gitversion

[![MIT licensed][mit-badge]][mit-url]

#### Basic Semantic and Sequential Versioning for Git projects

The download size of the docker image is only around *9MB* and the actual size of the docker image is less than *19MB*.

[mit-badge]: https://img.shields.io/badge/license-MIT-blue.svg
[mit-url]: LICENSE

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
