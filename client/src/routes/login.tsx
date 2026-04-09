import { onMount } from "solid-js";
import { code_challenge, code_verifier_gen } from "~/lib/auth";
import { serverAddress, userName, setUserName } from "~/lib/auth";
import { Button } from "~/components/ui/button";

export default function Login() {
  onMount(() => {
    sessionStorage.setItem("code_verifier", code_verifier_gen());
  });

  return (
    <main class="flex items-center justify-center flex-1 bg-background text-foreground">
      <div class="w-full max-w-72 flex flex-col gap-3">
        <div class="text-center py-4">
          <h1 class="text-2xl font-semibold text-primary">Sign in</h1>
          <p class="text-sm text-muted-foreground">to continue to Vitium</p>
        </div>

        <form
          method="post"
          action={serverAddress()?.toString() + "/login?" + code_challenge(sessionStorage.getItem("code_verifier") ?? "")}
          class="space-y-6"
        >
          <div>
            <label for="user" class="text-sm font-medium">
              Username
            </label>
            <input
              id="user"
              type="text"
              class="mt-1 w-full px-3 py-2 rounded-xl border"
              value={userName()}
              onInput={(e) => setUserName(e.currentTarget.value)}
              required
            />
          </div>

          <div>
            <label for="pass" class="text-sm font-medium">
              Password
            </label>
            <input
              id="pass"
              type="password"
              class="mt-1 w-full px-3 py-2 rounded-xl border"
              required
            />
          </div>

          <Button type="submit" class="w-full text-sm py-4 rounded-xl">
            "Sign In"
          </Button>
        </form>
        <a href="/signup" class="w-full text-sm text-center">
          Sign Up
        </a>
      </div>
    </main>
  );
}
