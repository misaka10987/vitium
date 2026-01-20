import { Button } from '~/components/ui/button'
import LogIn from 'lucide-solid/icons/log-in'
import UserPlus from 'lucide-solid/icons/user-plus'
import Icon from '~/asset/icon.svg'

export default function Home() {
  return (
    <main class="w-full h-full flex items-center justify-center overflow-scroll">
      <div class="flex flex-wrap-reverse items-center justify-center gap-12 m-12">
        <div class="flex flex-col gap-8">
          <div class="flex flex-col gap-4">
            <div class="text-6xl font-semibold">TRPG Framework</div>
            <div class="text-lg text-secondary-foreground p-1">
              Login to start your adventure
            </div>
          </div>
          <div class="flex flex-row">
            <a href="/login" class="mx-2">
              <Button size="lg" variant="default" class="text-lg rounded-full">
                <LogIn class="w-6 h-6" />
                Login
              </Button>
            </a>
            <a href="/signup" class="mx-2">
              <Button size="lg" variant="outline" class="text-lg rounded-full">
                <UserPlus class="w-6 h-6" />
                Sign Up
              </Button>
            </a>
          </div>
        </div>
        <img src={Icon} class="w-64" alt="Vitium Logo" />
      </div>
    </main>
  )
}
