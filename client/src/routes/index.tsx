import { A } from "@solidjs/router";
import { Button } from "~/components/ui/button";
import { useColorMode } from "@kobalte/core";
import { createMemo, onMount } from "solid-js";
import { setServerAddress } from "~/lib/auth";
  // On mount, check for ?server= in the URL and store it
  onMount(() => {
    if (typeof window !== "undefined") {
      const params = new URLSearchParams(window.location.search);
      const server = params.get("server");
      if (server) {
        setServerAddress(new URL(server));
      }
    }
  });

export default function Home() {
  const { colorMode } = useColorMode();

  const iconSrc = createMemo(() => {
    const mode = colorMode();
    // public contains `icon-white.svg` and `icon-black.svg`
    if (mode === "dark") return "/icon-white.svg";
    if (mode === "light") return "/icon-black.svg";

    // system or unknown: try to detect preference on client
    if (typeof window !== "undefined" && window.matchMedia) {
      return window.matchMedia("(prefers-color-scheme: dark)").matches ? "/icon-white.svg" : "/icon-black.svg";
    }
    return "/icon-black.svg";
  });

  return (
    <main class="flex items-center justify-center flex-1 bg-background text-foreground">
      <div class="flex flex-col items-center gap-8">
        <img src={iconSrc()} alt="Vitium" class="w-48 h-48" />
        <A href="/login">
          <Button size="lg" variant="ghost" class="text-lg px-8 py-6">
            Login
          </Button>
        </A>
      </div>
    </main>
  );
}
