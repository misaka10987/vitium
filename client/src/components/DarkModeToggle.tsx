import { useColorMode } from '@kobalte/core'

import { Button } from '~/components/ui/button'
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from '~/components/ui/dropdown-menu'

import SunIcon from 'lucide-solid/icons/sun'
import MoonIcon from 'lucide-solid/icons/moon'
import LaptopIcon from 'lucide-solid/icons/laptop'

export const DarkModeToggle = () => {
  const { setColorMode } = useColorMode()
  return (
    <DropdownMenu>
      <DropdownMenuTrigger
        as={Button<'button'>}
        variant="ghost"
        size="sm"
        class="w-9 px-0"
      >
        <SunIcon class="size-6 rotate-0 scale-100 transition-all dark:-rotate-90 dark:scale-0" />
        <MoonIcon class="absolute size-6 rotate-90 scale-0 transition-all dark:rotate-0 dark:scale-100" />
        <span class="sr-only">Toggle theme</span>
      </DropdownMenuTrigger>
      <DropdownMenuContent>
        <DropdownMenuItem onSelect={() => setColorMode('light')}>
          <SunIcon class="mr-2 size-4" />
          <span>Light</span>
        </DropdownMenuItem>
        <DropdownMenuItem onSelect={() => setColorMode('dark')}>
          <MoonIcon class="mr-2 size-4" />
          <span>Dark</span>
        </DropdownMenuItem>
        <DropdownMenuItem onSelect={() => setColorMode('system')}>
          <LaptopIcon class="mr-2 size-4" />
          <span>System</span>
        </DropdownMenuItem>
      </DropdownMenuContent>
    </DropdownMenu>
  )
}
