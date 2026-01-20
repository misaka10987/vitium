import { Button } from '~/components/ui/button'
import LogIn from 'lucide-solid/icons/log-in'
import UserPlus from 'lucide-solid/icons/user-plus'
import Icon from '~/asset/icon.svg'

export default function Home() {
  return (
    <main class="flex flex-col-reverse items-center justify-center h-full mx-24 md:gap-16 md:flex-row bg-background text-foreground">
      <div class="flex flex-col items-center md:max-w-1/2 gap-8">
        <div class="flex flex-col items-center gap-4">
          <div class="text-5xl md:text-6xl font-medium text-center">
            Vitium: A TRPG Framework
          </div>
          <div class="text-lg">Login to start your adventure</div>
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
      <div class="flex items-center justify-center [@media(max-height:40rem)_and_(max-width:767.98px)]:hidden">
        <img
          src={Icon}
          class="w-72 h-72 md:w-96 md:h-96 min-w-64"
          alt="Vitium Logo"
        />
      </div>
    </main>
  )
}
