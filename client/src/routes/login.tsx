import { createEffect, createSignal, onMount } from "solid-js";
import { useSearchParams } from "@solidjs/router";
import { code_challenge, code_verifier_gen } from "~/lib/auth";
import { Button } from "~/components/ui/button";

export default () => {
  const [codeChallenge, setCodeChallenge] = createSignal("");
  const [userName, setUserName] = createSignal("");

  onMount(async () => {
    const verifier = code_verifier_gen();
    sessionStorage.setItem("code_verifier", verifier);

    const challenge = await code_challenge(verifier);
    setCodeChallenge(challenge);
  });

  createEffect(() => {
    setUserName(localStorage.getItem("user_name") ?? "");
  });

  const serverAddress = () => {
    const [params] = useSearchParams();
    const url = params.server;
    if (typeof url !== "string") {
      throw new Error("server parameter is required");
    }
    return url;
  };

  const [codeChallengeGuard, setCodeChallengeGuard] = createSignal(true);

  const loginAddress = () => {
    if(typeof window === "undefined") {
      return "";
    }
    const challenge = codeChallenge();
    if (challenge === "") {
      setCodeChallengeGuard(true);
      return "";
    }
    const loginUrl = new URL("/login", serverAddress());
    loginUrl.searchParams.set("code_challenge", challenge);
    setCodeChallengeGuard(false);
    return loginUrl.toString();
  };

  return (
    <main class="flex items-center justify-center flex-1 bg-background text-foreground">
      <div class="w-full max-w-72 flex flex-col gap-3">
        <div class="text-center py-4">
          <h1 class="text-2xl font-semibold text-primary">Sign in</h1>
          <p class="text-sm text-muted-foreground">to continue to Vitium</p>
        </div>

        <form
          method="post"
          action={loginAddress()}
          onsubmit={() => {localStorage.setItem("user_name", userName())}}
          class="space-y-6"
        >
          <div>
            <label for="user" class="text-sm font-medium">
              Username
            </label>
            <input
              id="user"
              name="user"
              type="text"
              class="mt-1 w-full px-3 py-2 rounded-xl border"
              onInput={(e) => setUserName(e.currentTarget.value)}
              value={userName()}
              required
            />
          </div>

          <div>
            <label for="pass" class="text-sm font-medium">
              Password
            </label>
            <input
              id="pass"
              name="pass"
              type="password"
              class="mt-1 w-full px-3 py-2 rounded-xl border"
              required
            />
          </div>

          <Button type="submit" class="w-full text-sm py-4 rounded-xl" disabled={codeChallengeGuard()}>
            Sign In
          </Button>
        </form>
        <a href={`/signup?server=${serverAddress()}`} class="w-full text-sm text-center">
          Sign Up
        </a>
      </div>
    </main>
  );
}
