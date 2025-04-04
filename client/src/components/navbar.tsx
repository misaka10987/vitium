"use client"

import {
  NavigationMenu,
  NavigationMenuItem,
  NavigationMenuList,
} from "@/components/ui/navigation-menu"
import { DarkModeSwitch } from "@/components/dark-mode-switch"

export const NavBar = () => {
  return (
    <NavigationMenu className="p-1">
      <NavigationMenuList>
        <NavigationMenuItem>
          <DarkModeSwitch />
        </NavigationMenuItem>
      </NavigationMenuList>
    </NavigationMenu>
  )
}
