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
            <h1 class="text-6xl font-semibold">TRPG Framework</h1>
            <p class="text-lg text-secondary-foreground p-1">
              Login to start your adventure
            </p>
          </div>
          <div class="flex flex-row gap-4 p-1">
            <Button
              as="a"
              href="/login"
              size="lg"
              variant="default"
              class="text-lg rounded-full"
            >
              <LogIn class="w-6 h-6" />
              Start
            </Button>
            <Button
              as="a"
              href="/signup"
              size="lg"
              variant="outline"
              class="text-lg rounded-full"
            >
              <UserPlus class="w-6 h-6" />
              Sign Up
            </Button>
          </div>
        </div>
        <img src={Icon} class="w-64" alt="Vitium Logo" />
      </div>
    </main>
  )
}
