'use client'

import { Button } from '@/components/ui/button'
import {
    Card,
    CardContent,
    CardDescription,
    CardHeader,
    CardTitle,
} from '@/components/ui/card'
import { Input } from '@/components/ui/input'
import { Label } from '@/components/ui/label'
import { Host } from '@/components/host'
import { useUserStore } from '@/components/user'
import { useId, useState } from 'react'
import Link from 'next/link'
import { signup } from '@/lib/auth'
import { useRouter } from 'next/navigation'
import { panic } from '@/lib/util'
import { SignUp } from 'vitium-api'

/**
 * User interface for signing up to a certain game server.
 */
export const SignupForm = () => {
    const userInputId = useId()
    const passInputId = useId()
    const passInputId2 = useId()
    const [passwordMismatch, setpasswordMismatch] = useState(false)
    const router = useRouter()
    return (
        <div className="flex flex-col gap-6">
            <Card>
                <CardHeader>
                    <CardTitle>
                        Connect to: <Host />
                    </CardTitle>
                    <CardDescription>
                        Connect to the above game server
                    </CardDescription>
                </CardHeader>
                <CardContent>
                    <form
                        onSubmit={async (e) => {
                            e.preventDefault()
                            const data = new FormData(e.currentTarget)
                            const user = data.get('user')?.toString() ?? panic()
                            const pass = data.get('pass')?.toString() ?? panic()
                            const pass2 = data.get('pass2')?.toString() ?? panic()
                            if (pass !== pass2) {
                                setpasswordMismatch(true)
                                return
                            }
                            var signupInfo = {
                                user,
                                pass,
                            }
                            const res = await signup(signupInfo)
                            if (res.ok) {
                                // todo - change alert to toast or popover
                                alert('Account created successfully')
                                router.replace('/login')
                            }
                            else {
                                console.error('Failed to create account. Error: ' + res.statusText)
                            }
                        }}
                    >
                        <div className="flex flex-col gap-6">
                            <div className="grid gap-3">
                                <Label htmlFor={userInputId}>Username</Label>
                                <Input
                                    id={userInputId}
                                    name="user"
                                    required
                                />
                            </div>
                            <div className="grid gap-3">
                                <div className="flex items-center">
                                    <Label htmlFor={passInputId}>Password</Label>
                                </div>
                                <Input id={passInputId} name="pass" type="password" required />
                            </div>
                            <div className="grid gap-3">
                                <div className="flex items-center">
                                    <Label htmlFor={passInputId2}>Confirm Password</Label>
                                </div>
                                <Input id={passInputId2} name="pass2" type="password" required />
                            </div>
                            <div className="flex h-0 items-center">
                                {passwordMismatch && (
                                    <p className="text-sm text-red-600">
                                        The passwords do not match
                                    </p>
                                )}
                            </div>
                            <div className="flex flex-col gap-3">
                                <Button type="submit" className="w-full">
                                    Submit
                                </Button>
                            </div>
                        </div>
                    </form>
                </CardContent>
            </Card>
        </div>
    )
}
