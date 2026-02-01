import { createSignal } from "solid-js";
import { serverAddress } from "~/lib/auth";
import { Button } from "~/components/ui/button";

export default function Signup() {
  const [user, setUser] = createSignal("");
  const [pass, setPass] = createSignal("");
  const [confirmPass, setConfirmPass] = createSignal("");
  const [email, setEmail] = createSignal("");
  const [isLoading, setIsLoading] = createSignal(false);
  const [error, setError] = createSignal("");

  const handleSignup = async () => {
    setError("");
    if (pass() !== confirmPass()) {
      setError("Passwords do not match");
      return;
    }
    setIsLoading(true);
    try {
      const url = serverAddress();
      if (url === null) {
        throw new Error("Server address is not set");
      }
      const res = await fetch(new URL("/signup", url), {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },
        body: JSON.stringify({
          user: user(),
          pass: pass(),
          email: email(),
        }),
      });
      if (res.status != 303 && !res.ok) {
        throw new Error("Signup failed");
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
          <h1 class="text-2xl font-semibold text-primary">Sign up</h1>
          <p class="text-sm text-muted-foreground">create an account</p>
        </div>

        <form
          onSubmit={(e) => {
            e.preventDefault();
            handleSignup();
          }}
          class="space-y-4"
        >
          <div>
            <label for="user" class="text-sm font-medium">
              Username <span class="text-red-500">*</span>
            </label>
            <input
              id="user"
              type="text"
              class="mt-1 w-full px-3 py-2 rounded-xl border"
              value={user()}
              onInput={(e) => setUser(e.currentTarget.value)}
              required
            />
          </div>

          <div>
            <label for="email" class="text-sm font-medium">
              Email
            </label>
            <input
              id="email"
              type="email"
              class="mt-1 w-full px-3 py-2 rounded-xl border"
              value={email()}
              onInput={(e) => setEmail(e.currentTarget.value)}
            />
          </div>

          <div>
            <label for="pass" class="text-sm font-medium">
              Password <span class="text-red-500">*</span>
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

          <div>
            <label for="confirmPass" class="text-sm font-medium">
              Confirm Password <span class="text-red-500">*</span>
            </label>
            <input
              id="confirmPass"
              type="password"
              class="mt-1 w-full px-3 py-2 rounded-xl border"
              value={confirmPass()}
              onInput={(e) => setConfirmPass(e.currentTarget.value)}
              required
            />
          </div>

          {error() && (
            <div class="text-red-500 text-sm text-center">{error()}</div>
          )}

          <Button type="submit" class="w-full text-sm py-4 rounded-xl" disabled={isLoading()}>
            {isLoading() ? "Signing up..." : "Sign Up"}
          </Button>
        </form>
        <a href="/login" class="w-full text-sm text-center">
          Sign In
        </a>
      </div>
    </main>
  );
}
