## Internet library

Eventually, a way to create a server will be added, but that is not yet implemented.

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