'use client'

import { NavigationMenu, NavigationMenuItem } from '~/components/ui/navigation-menu'
import { DarkModeSwitch } from '~/components/DarkmodeSwitch'
import { Button } from '~/components/ui/button'

export const NavBar = () => {
  return (
    <NavigationMenu class="px-2 py-0.5 gap-2 items-start justify-start">
      <NavigationMenuItem>
        <Button variant="ghost" class="select-none">
          <a href="/">Vitium</a>
        </Button>
      </NavigationMenuItem>
      <NavigationMenuItem>
        <DarkModeSwitch />
      </NavigationMenuItem>
    </NavigationMenu >
  )
}
