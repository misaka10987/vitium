'use client'

import {
  NavigationMenu,
  NavigationMenuItem,
  NavigationMenuList,
} from '@/components/ui/navigation-menu'
import { DarkModeSwitch } from '@/components/dark-mode-switch'
import Link from 'next/link'
import { Button } from '@/components/ui/button'

export const NavBar = () => {
  return (
    <NavigationMenu className="p-1 gap-2 items-start justify-start">
      <NavigationMenuList>
        <NavigationMenuItem>
          <Button variant="ghost" asChild className="select-none">
            <div className="flex flex-row">
              <Link href="/">Vitium</Link>
            </div>
          </Button>
        </NavigationMenuItem>
      </NavigationMenuList>
      <NavigationMenuList>
        <NavigationMenuItem>
          <DarkModeSwitch />
        </NavigationMenuItem>
      </NavigationMenuList>
    </NavigationMenu>
  )
}
