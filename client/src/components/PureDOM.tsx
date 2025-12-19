import { createEffect, createSignal, onCleanup } from "solid-js";
import DOMPurify, { Config } from "dompurify";

interface PureDOMProps {
	html: string;
	class?: string; // Tailwind or custom classes for iframe
}

// This component renders sanitized HTML inside a sandboxed iframe for isolation
export function PureDOM(props: PureDOMProps) {
	const [cleanhtml, setCleanhtml] = createSignal("");

	createEffect(() => {
		const config: Config = {
			ALLOWED_TAGS: [
			  "a", "abbr", "b", "blockquote", "br", "code", "div", "em", "h1", "h2", "h3", "h4", "h5", "h6",
			  "hr", "i", "img", "li", "ol", "p", "pre", "s", "small", "span", "strong", "sub", "sup", "table",
			  "tbody", "td", "th", "thead", "tr", "u", "ul"
			],
			ADD_ATTR: ["target"]
		};
		// Sanitize the HTML
		setCleanhtml(DOMPurify.sanitize(props.html, config));
	});

	return (
		<div class={props.class ?? ""} innerHTML={cleanhtml()}></div>
	);
}
