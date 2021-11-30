## Internet library

How to import:

```
@main() {
	# Put this inside of the main function. #
	use internet
}
```

### Functions

#### `get(String) -> String`
Performs a `GET` HTTP request and returns the result.

#### `post(String, String) -> String`
Performs a `POST` HTTP request and returns the result.
Takes in URL, then the body.