import { Routes, Route } from "react-router-dom"
import Header from "@/components/Header"
import Footer from "@/components/Footer"
import HomePage from "@/pages/HomePage"
import ProductList from "@/pages/ProductList"
import ProductDetail from "@/pages/ProductDetail"
import Cart from "@/pages/Cart"
import Login from "@/pages/Login"
import Register from "@/pages/Register"

export default function App() {
  return (
    <div className="flex min-h-screen flex-col">
      <Header cartCount={0} />
      <main className="flex-1">
        <Routes>
          <Route path="/" element={<HomePage />} />
          <Route path="/products" element={<ProductList />} />
          <Route path="/products/:id" element={<ProductDetail />} />
          <Route path="/cart" element={<Cart />} />
          <Route path="/login" element={<Login />} />
          <Route path="/register" element={<Register />} />
        </Routes>
      </main>
      <Footer />
    </div>
  )
}

