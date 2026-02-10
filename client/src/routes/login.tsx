import { createSignal } from "solid-js";
import { serverAddress, userName, setUserName } from "~/lib/auth";
import { Button } from "~/components/ui/button";

export default function Login() {
  const [pass, setPass] = createSignal("");
  const [isLoading, setIsLoading] = createSignal(false);
  const [error, setError] = createSignal("");

  const handleLogin = async () => {
    setError("");
    setIsLoading(true);
    console.log(`server: ${serverAddress()}`);
    try {
      const url = serverAddress();
      if(url === null) {
        throw new Error("Server address is not set");
      }
      const input = new URLSearchParams();
      input.append("user", userName());
      input.append("pass", pass());
      const res = await fetch(new URL("/login", url), {
        method: "POST",
        headers: {
            "Content-Type": "application/x-www-form-urlencoded",
        },
        body: input,
      });
      if (res.status != 303 && !res.ok) {
        throw new Error("Login failed");
      }
      // Optionally parse response
      // const data = await res.json();
    } catch (err) {
      console.error(err);
      setError("Please check your credentials");
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
              value={pass()}
              onInput={(e) => setPass(e.currentTarget.value)}
              required
            />
          </div>

          {error() && (
            <div class="text-red-500 text-sm text-center">{error()}</div>
          )}

          <Button type="submit" class="w-full text-sm py-4 rounded-xl" disabled={isLoading()}>
            {isLoading() ? "Signing in..." : "Sign In"}
          </Button>
        </form>
        <a href="/signup" class="w-full text-sm text-center">
          Sign Up
        </a>
      </div>
    </main>
  );
}
