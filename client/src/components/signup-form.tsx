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
import { Host, useHostStore } from '@/components/host'
import { useId, useState, useEffect, useRef } from 'react'
import { useRouter } from 'next/navigation'

/**
 * User interface for signing up to a certain game server.
 */
export const SignupForm = () => {
    const userInputId = useId()
    const passInputId = useId()
    const passInputId2 = useId()
    const [confirmPassword, setConfirmPassword] = useState('')
    const [passwordMismatch, setpasswordMismatch] = useState(false)
    const { host } = useHostStore()
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
                        action={host ? `/api/signup` : undefined}
                        method="POST"
                        encType="application/x-www-form-urlencoded"
                        onSubmit={(e) => {
                            const formData = new FormData(e.currentTarget)
                            if (formData.get('pass') !== confirmPassword) {
                                e.preventDefault()
                                setpasswordMismatch(true)
                                return
                            }
                            router.replace('/login')
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
                                <Input
                                    id={passInputId2}
                                    type="password"
                                    required
                                    value={confirmPassword}
                                    onChange={(e) => setConfirmPassword(e.target.value)}
                                />
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
