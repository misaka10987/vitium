import { createEffect, createSignal, onCleanup } from "solid-js";
import DOMPurify from "dompurify";

interface PureDOMProps {
	html: string;
	sandbox?: string; // e.g., "allow-scripts"
	class?: string; // Tailwind or custom classes for iframe
}

// This component renders sanitized HTML inside a sandboxed iframe for isolation
export default function PureDOM(props: PureDOMProps) {
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

	return (
		<iframe
			ref={iframeRef}
			srcdoc={srcDoc()}
			sandbox={props.sandbox ?? "allow-scripts"}
			class={`w-full border-0 min-h-[2em] ${props.class ?? ""}`}
			loading="lazy"
			aria-label="Sanitized HTML content"
		/>
	);
}
