export default function Footer() {
  return (
    <footer className="border-t py-8 mt-10">
      <div className="container text-center text-sm text-muted-foreground">
        © {new Date().getFullYear()} ShopSmart. All rights reserved.
      </div>
    </footer>
  )
}

