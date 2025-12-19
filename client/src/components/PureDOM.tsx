import { createEffect, createSignal, onCleanup } from "solid-js";
import DOMPurify from "dompurify";

interface PureDOMProps {
	html: string;
	sandbox?: string; // e.g., "allow-scripts"
	class?: string; // Tailwind or custom classes for iframe
}

// This component renders sanitized HTML inside a sandboxed iframe for isolation
export function PureDOM(props: PureDOMProps) {
	let iframeRef: HTMLIFrameElement | undefined;
	const [srcDoc, setSrcDoc] = createSignal("");

	createEffect(() => {
		// Sanitize the HTML
		const clean = DOMPurify.sanitize(props.html, { ADD_ATTR: ["target"] });
		const doc = `<!DOCTYPE html><html><head><base target='_blank'></head><body>${clean}</body></html>`;
		setSrcDoc(doc);
	});

	// Cleanup on unmount
	// onCleanup(() => {
	// 	setSrcDoc("");
	// });


	// note that the sandbox must not 'allow-same-origin' to ensure login token safety
	return (
		<iframe
			ref={iframeRef}
			srcdoc={srcDoc()}
			sandbox={props.sandbox ?? "allow-scripts"}
			class={`w-full border-0 ${props.class ?? ""}`}
			loading="lazy"
			aria-label="Sanitized HTML content"
		/>
	);
}
