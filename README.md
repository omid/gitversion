# gitversion

[![MIT licensed][mit-badge]][mit-url]

Semantic Versioning for Git projects

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
