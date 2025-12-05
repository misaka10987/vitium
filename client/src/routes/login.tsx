import { A } from "@solidjs/router";
import { createSignal } from "solid-js";
import { Button } from "~/components/ui/button";

export default function Login() {
  const [email, setEmail] = createSignal("");
  const [password, setPassword] = createSignal("");
  const [isLoading, setIsLoading] = createSignal(false);

  const handleLogin = async () => {
    setIsLoading(true);
    try {
      // TODO: Implement login logic here
      console.log("Login attempt with:", { email: email(), password: password() });
    } finally {
      setIsLoading(false);
    }
  };

  return (
    <main class="flex items-center justify-center min-h-screen bg-background text-foreground">
      <div class="w-full max-w-md">
        <div class="flex flex-col gap-6">
          <div class="text-center">
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
              <label for="email" class="block text-sm font-medium">
                Email
              </label>
              <input
                id="email"
                type="email"
                class="mt-1 w-full px-3 py-2 rounded-md border border-input placeholder:text-muted-foreground"
                value={email()}
                onInput={(e) => setEmail(e.currentTarget.value)}
                required
              />
            </div>

            <div>
              <label for="password" class="block text-sm font-medium">
                Password
              </label>
              <input
                id="password"
                type="password"
                class="mt-1 w-full px-3 py-2 rounded-md border border-input placeholder:text-muted-foreground"
                value={password()}
                onInput={(e) => setPassword(e.currentTarget.value)}
                required
              />
            </div>

            <Button type="submit" class="w-full py-2" disabled={isLoading()}>
              {isLoading() ? "Signing in..." : "Sign In"}
            </Button>
          </form>

          <div class="text-center">
            <A href="/" class="text-sm text-primary hover:underline">
              Back to Home
            </A>
          </div>
        </div>
      </div>
    </main>
  );
}
