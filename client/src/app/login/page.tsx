import { Host } from '@/components/host'
import { LoginForm } from '@/components/login-form'
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from '@/components/ui/card'
import Link from 'next/link'

export default function Page() {
  return (
    <div className="flex w-full h-full justify-center items-center">
      <div className="w-full max-w-sm">
        <Card className="select-none">
          <CardHeader>
            <CardTitle>
              Login to: <Host />
            </CardTitle>
            <CardDescription>
              Login to the above game server with your username and password
            </CardDescription>
          </CardHeader>
          <CardContent className="flex flex-col gap-4">
            <LoginForm />
            <div className="text-center text-sm">
              Don&apos;t have an account?&nbsp;
              <Link href="/signup" className="underline underline-offset-4">
                Sign up
              </Link>
            </div>
          </CardContent>
        </Card>
      </div>
    </div>
  )
}
