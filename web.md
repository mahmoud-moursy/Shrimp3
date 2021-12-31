A library to quickly construct webpages in Shrimp3, and eventually, a library to host web applications.

# The `html_` component

I've implemented a few HTML tags, but not all of them. For minimal use-cases, you can use them to create a webpage

e.g:
```
con(
	# The format is the body of the tag, and then any
		attributes can be optionally specified later.
		This applies to all tags, even if they do not usually
		have a body, such as <img /> tags. #
	el_head(
		con(
			el_comment("Create HTML comments using the el_comment function.")
			el_title("Test page")
			# Not quite sure how a stylesheet is linked.
				Haven't touched HTML in a long time! #
			link("" "rel=\"stylesheet\" href=\"master.css\"")
		)
	)
	el_body(
		el_p("Hello world" "style=\"color: red\"")
	)
)
```