import { createSignal } from "solid-js";
import { serverAddress } from "~/lib/auth";
import { Button } from "~/components/ui/button";

export default function Signup() {
  const [pass, setPass] = createSignal("");
  const [confirmPass, setConfirmPass] = createSignal("");

  return (
    <main class="flex items-center justify-center flex-1 bg-background text-foreground">
      <div class="w-full max-w-72 flex flex-col gap-3">
        <div class="text-center py-4">
          <h1 class="text-2xl font-semibold text-primary">Sign up</h1>
          <p class="text-sm text-muted-foreground">create an account</p>
        </div>

        <form
          action={new URL("/signup", serverAddress()).toString()}
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
              minLength={6}
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

          <Button type="submit" class="w-full text-sm py-4 rounded-xl" disabled={pass() === "" || pass() !== confirmPass()}>
            Sign Up
          </Button>
        </form>
        <a href="/login" class="w-full text-sm text-center">
          Sign In
        </a>
      </div>
    </main>
  );
}
