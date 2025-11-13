import { useForm } from "react-hook-form"
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
      alert('Login successful')
    } catch {
      alert('Login failed')
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
