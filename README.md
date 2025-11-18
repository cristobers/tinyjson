# tinyjson - A Small JSON Validator

Given a JSON file as input, the program will make sure that it complies with the JSON 
spec.

JSON rules implemented from [JSON.org](https://www.json.org/json-en.html)

```
# Build and run
cargo build --release && <path_to_built_binary>/./tinyjson <input_file>
```

## Issues

- Doesn't handle non-ASCII characters when constructing strings.
- Only understands decimal numbers when parsing numbers at the moment.
