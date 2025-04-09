import type { Metadata } from 'next'
import './globals.css'
import { ThemeProvider } from '@/components/theme-provider'
import { NavBar } from '@/components/navbar'

export const metadata: Metadata = {
  title: 'Vitium',
  description: 'Vitium Web Client',
}

export default function Layout({
  children,
}: Readonly<{
  children: React.ReactNode
}>) {
  return (
    <html lang="en" suppressHydrationWarning>
      <body className="antialiased">
        <ThemeProvider
          attribute="class"
          defaultTheme="system"
          enableSystem
          disableTransitionOnChange
        >
          <div className="fixed top-0 bottom-0 left-0 right-0 flex flex-col">
            <div className="relative top-0">
              <NavBar />
            </div>
            <div className="flex-1 flex-grow">{children}</div>
          </div>
        </ThemeProvider>
      </body>
    </html>
  )
}
