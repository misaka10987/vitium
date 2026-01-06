import { Button } from "~/components/ui/button";
import LogIn from "lucide-solid/icons/log-in";
import UserPlus from "lucide-solid/icons/user-plus";
import Icon from "~/asset/icon.svg"

export default function Home() {
  return (
    <main class="grid grid-cols-2 items-center flex-1 m-20 gap-12 bg-background text-foreground">
      <div class="flex flex-col items-center flex-1">
        <div class="text-6xl font-semibold mb-6 text-center">Vitium: A TRPG Framework</div>
        <div class="text-lg mb-8">Login to start your adventure</div>
        <div class="flex flex-row">
          <a href="/login" class="select-none mx-2">
            <Button size="lg" variant="default" class="text-lg select-none rounded-full">
              <LogIn class="w-6 h-6" />
              Login
            </Button>
          </a>
          <a href="/signup" class="select-none mx-2">
            <Button size="lg" variant="outline" class="text-lg select-none rounded-full">
              <UserPlus class="w-6 h-6" />
              Sign Up
            </Button>
          </a>
        </div>
      </div>
      <div class="flex items-center flex-1 select-none">
        <img src={Icon} class="w-lg h-lg min-w-64 select-none" alt="Vitium Logo" />
      </div>
    </main>
  );
}
