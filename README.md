# Leptos Styling

Scoped CSS styling for leptos components.

Leptos Styling is an idiomatic, abstracted extension providing scoped css (and [others](#supported-css-extensions)) for leptos components. No need to worry about class conflicts, unnecessary `.css` files, or complex selectors. Just attach a `#[style]` macro to the component's definition.

## Defining a Style

```rust
use leptos::prelude::*;
use leptos_styling::prelude::*;

#[component]
#[style {
	h1 {
		font-style: italic;

		b {
			color: green;
		}
	}

	.my-class {
		background-color: red;
	}
}]
pub fn MyComponent -> impl IntoView {
	view! {
		<h1>
			"This is a"
			<b>"styled"</b>
			"title!"
		</h1>

		<p class="my-class">
			"This block has a "<b>"red"</b>" background."
		</p>
	}
}
```

## Supported CSS Extensions
TODO

## Inheriting From CSS Files

### Tailwind
TODO