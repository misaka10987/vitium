import { A } from "@solidjs/router";
import { Button } from "~/components/ui/button";
import { useColorMode } from "@kobalte/core";
import { createMemo, onMount } from "solid-js";
import { setServerAddress } from "~/lib/auth";
import { LogIn, UserPlus } from "lucide-solid";
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
    <main class="flex flex-row items-center flex-1 pb-24 px-20 bg-background text-foreground min-h-screen select-none">
      <div class="flex flex-col items-center flex-1 select-none">
        <div class="text-6xl font-semibold mb-6 text-center select-none">Vitium: A TRPG Framework</div>
        <div class="flex flex-row">
          <A href="/login" class="select-none mx-2">
            <Button size="lg" variant="default" class="text-lg select-none rounded-full">
              <LogIn class="w-6 h-6" />
              Login
            </Button>
          </A>
          <A href="/signin" class="select-none mx-2">
            <Button size="lg" variant="outline" class="text-lg select-none rounded-full">
              <UserPlus class="w-6 h-6" />
              Signin
            </Button>
          </A>
        </div>
      </div>
      <div class="flex flex-col items-center flex-1 select-none">
        <img src={iconSrc()} alt="Vitium" class="w-96 h-96 select-none" />
      </div>
    </main>
  );
}
