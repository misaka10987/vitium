import { Button } from "~/components/ui/button";
import LogIn from "lucide-solid/icons/log-in";
import UserPlus from "lucide-solid/icons/user-plus";
import Icon from "~/asset/icon.svg"

export default function Home() {
  return (
    <main class="grid grid-cols-1 items-center h-full mx-24 md:gap-16 md:grid-cols-2 bg-background text-foreground">
      <div class="flex flex-col items-center">
        <div class="text-5xl md:text-6xl font-medium mb-6 text-center">Vitium: A TRPG Framework</div>
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
      <div class="flex items-center justify-center [@media(max-height:48rem)_and_(max-width:767.98px)]:hidden select-none">
        <img src={Icon} class="w-72 h-72 md:w-96 md:h-96 min-w-64 select-none" alt="Vitium Logo" />
      </div>
    </main>
  );
}
