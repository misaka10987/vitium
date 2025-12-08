import { A } from "@solidjs/router";
import { createSignal } from "solid-js";
import { Button } from "~/components/ui/button";

export default function Login() {
  const [usrName, setUsrName] = createSignal("");
  const [password, setPassword] = createSignal("");
  const [isLoading, setIsLoading] = createSignal(false);

  const handleLogin = async () => {
    setIsLoading(true);
    try {
      // TODO: Implement login logic here
      console.log("Login attempt with:", { usrName: usrName(), password: password() });
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
            <label for="usrName" class="text-sm font-medium">
              User Name
            </label>
            <input
              id="usrName"
              type="text"
              class="mt-1 w-full px-3 py-2 rounded-md border"
              value={usrName()}
              onInput={(e) => setUsrName(e.currentTarget.value)}
              required
            />
          </div>

          <div>
            <label for="password" class="text-sm font-medium">
              Password
            </label>
            <input
              id="password"
              type="password"
              class="mt-1 w-full px-3 py-2 rounded-md border"
              value={password()}
              onInput={(e) => setPassword(e.currentTarget.value)}
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
