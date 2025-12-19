import { A } from "@solidjs/router";
import { createSignal } from "solid-js";
import { serverAddress, userName, setUserName } from "~/lib/auth";
import { Button } from "~/components/ui/button";

export default function Login() {
  const [pass, setPass] = createSignal("");
  const [isLoading, setIsLoading] = createSignal(false);

  const handleLogin = async () => {
    setIsLoading(true);
    try {
      const url = serverAddress();
      const res = await fetch(url + "/login", {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },
        body: JSON.stringify({
          user: userName(),
          pass: pass(),
        }),
      });
      if (res.status != 303 && !res.ok) {
        throw new Error("Login failed");
      }
      // Optionally parse response
      // const data = await res.json();
    } catch (err) {
      console.error(err);
    } finally {
      setIsLoading(false);
    }
  };

  return (
    <main class="flex items-center justify-center flex-1 bg-background text-foreground">
      <div class="w-full max-w-72 flex flex-col gap-3">
        <div class="text-center py-4">
          <h1 class="text-2xl font-semibold text-primary">Sign in</h1>
          <p class="text-sm text-muted-foreground">to continue to Vitium</p>
        </div>

        <form
          onSubmit={(e) => {
            e.preventDefault();
            handleLogin();
          }}
          class="space-y-4"
        >
          <div>
            <label for="user" class="text-sm font-medium">
              User Name
            </label>
            <input
              id="user"
              type="text"
              class="mt-1 w-full px-3 py-2 rounded-md border"
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
              class="mt-1 w-full px-3 py-2 rounded-md border"
              value={pass()}
              onInput={(e) => setPass(e.currentTarget.value)}
              required
            />
          </div>

          <Button type="submit" class="w-full py-4" disabled={isLoading()}>
            {isLoading() ? "Signing in..." : "Sign In"}
          </Button>
        </form>
        <A href="/signup">
          <Button variant="ghost" class="w-full">
            Sign Up
          </Button>
        </A>

        <div class="text-center">
          <A href="/" class="text-sm text-primary hover:underline">
            Back to Home
          </A>
        </div>
      </div>
    </main>
  );
}
