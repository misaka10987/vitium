import { A } from "@solidjs/router";
import { Button } from "~/components/ui/button";
import { useColorMode } from "@kobalte/core";
import { createMemo } from "solid-js";

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
    <main class="flex items-center justify-center min-h-screen bg-background text-foreground">
      <div class="flex flex-col items-center gap-8">
        {/* Vitium Icon (served from public/) - swaps based on theme */}
        <img src={iconSrc()} alt="Vitium" class="w-32 h-32" />

        {/* Login Button */}
        <A href="/login">
          <Button size="lg" variant="secondary" class="text-lg px-8 py-6">
            Login
          </Button>
        </A>
      </div>
    </main>
  );
}
