import { useForm } from "react-hook-form"
import { useState } from "react"
import { z } from "zod"
import { zodResolver } from "@hookform/resolvers/zod"
import { Input } from "@/components/ui/input"
import { Button } from "@/components/ui/button"
import { Card, CardHeader, CardTitle, CardContent, CardFooter } from "@/components/ui/card"
import { CONFIG } from "@/config"

const schema = z.object({
  email: z.string().email(),
  password: z.string().min(8),
})

export default function Login() {
  const form = useForm({ resolver: zodResolver(schema), defaultValues: { email: "", password: "" } })
  const [status, setStatus] = useState({ type: "", message: "" })
  const onSubmit = async (values) => {
    try {
      const res = await fetch(`${CONFIG.apiBaseUrl}/auth/login`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(values),
        credentials: 'include',
      })
      if (!res.ok) throw new Error(`Login failed: ${res.status}`)
      // Avoid storing tokens in localStorage; prefer httpOnly cookies.
      setStatus({ type: 'success', message: 'Login successful' })
    } catch {
      setStatus({ type: 'error', message: 'Login failed' })
    }
  }
  const { register, handleSubmit, formState } = form
  const { errors, isSubmitting } = formState
  return (
    <div className="container py-8">
      <Card className="mx-auto max-w-md">
        <CardHeader>
          <CardTitle>Login</CardTitle>
        </CardHeader>
        <CardContent className="space-y-4">
          {status.message && (
            <div className={`p-3 text-sm rounded-md border ${status.type === 'error' ? 'text-destructive bg-destructive/10 border-destructive/20' : 'text-green-600 bg-green-600/10 border-green-600/20'}`}>
              {status.message}
            </div>
          )}
          <form onSubmit={handleSubmit(onSubmit)} className="space-y-4">
            <div className="space-y-1">
              <label className="text-sm font-medium">Email</label>
              <Input type="email" autoComplete="email" {...register("email")} />
              {errors.email && <p className="text-xs text-destructive">{errors.email.message}</p>}
            </div>
            <div className="space-y-1">
              <label className="text-sm font-medium">Password</label>
              <Input type="password" autoComplete="current-password" {...register("password")} />
              {errors.password && <p className="text-xs text-destructive">{errors.password.message}</p>}
            </div>
            <Button type="submit" disabled={isSubmitting} className="w-full">Sign in</Button>
          </form>
        </CardContent>
        <CardFooter className="text-xs text-muted-foreground">We never store passwords client‑side.</CardFooter>
      </Card>
    </div>
  )
}
