import { Host } from '@/components/host'
import { SignupForm } from '@/components/signup-form'
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from '@/components/ui/card'

export default function Page() {
  return (
    <div className="flex w-full h-full justify-center items-center">
      <div className="w-full max-w-sm">
        <Card className="select-none">
          <CardHeader>
            <CardTitle>
              Sign up on: <Host />
            </CardTitle>
            <CardDescription>Sign up on the above game server</CardDescription>
          </CardHeader>
          <CardContent>
            <SignupForm />
          </CardContent>
        </Card>
      </div>
    </div>
  )
}
