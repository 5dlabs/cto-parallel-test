import { useForm } from "react-hook-form"
import { z } from "zod"
import { zodResolver } from "@hookform/resolvers/zod"
import { Input } from "@/components/ui/input"
import { Button } from "@/components/ui/button"
import { Card, CardHeader, CardTitle, CardContent, CardFooter } from "@/components/ui/card"
import { CONFIG } from "@/config"

const schema = z.object({
  name: z.string().min(2),
  email: z.string().email(),
  password: z.string().min(8),
})

export default function Register() {
  const form = useForm({ resolver: zodResolver(schema), defaultValues: { name: "", email: "", password: "" } })
  const onSubmit = async (values) => {
    try {
      const res = await fetch(`${CONFIG.apiBaseUrl}/auth/register`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(values),
      })
      if (!res.ok) throw new Error(`Register failed: ${res.status}`)
      alert('Registration successful. Please login.')
    } catch {
      alert('Registration failed')
    }
  }
  const { register, handleSubmit, formState } = form
  const { errors, isSubmitting } = formState
  return (
    <div className="container py-8">
      <Card className="mx-auto max-w-md">
        <CardHeader>
          <CardTitle>Register</CardTitle>
        </CardHeader>
        <CardContent>
          <form onSubmit={handleSubmit(onSubmit)} className="space-y-4">
            <div className="space-y-1">
              <label className="text-sm font-medium">Name</label>
              <Input {...register("name")} />
              {errors.name && <p className="text-xs text-destructive">{errors.name.message}</p>}
            </div>
            <div className="space-y-1">
              <label className="text-sm font-medium">Email</label>
              <Input type="email" autoComplete="email" {...register("email")} />
              {errors.email && <p className="text-xs text-destructive">{errors.email.message}</p>}
            </div>
            <div className="space-y-1">
              <label className="text-sm font-medium">Password</label>
              <Input type="password" autoComplete="new-password" {...register("password")} />
              {errors.password && <p className="text-xs text-destructive">{errors.password.message}</p>}
            </div>
            <Button type="submit" disabled={isSubmitting} className="w-full">Create account</Button>
          </form>
        </CardContent>
        <CardFooter className="text-xs text-muted-foreground">We enforce client‑side validation only; never store secrets.</CardFooter>
      </Card>
    </div>
  )
}
