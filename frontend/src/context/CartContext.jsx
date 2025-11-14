import { createContext, useContext, useMemo, useState, useEffect } from 'react'

const CartContext = createContext({
  items: [],
  count: 0,
  addItem: () => {},
  removeItem: () => {},
  clear: () => {},
})

export function CartProvider({ children }) {
  const [items, setItems] = useState(() => {
    try {
      const raw = localStorage.getItem('cart_items')
      return raw ? JSON.parse(raw) : []
    } catch {
      return []
    }
  })

  useEffect(() => {
    try {
      localStorage.setItem('cart_items', JSON.stringify(items))
    } catch {
      // ignore storage write errors
    }
  }, [items])

  const api = useMemo(
    () => ({
      items,
      count: items.reduce((acc, it) => acc + (it.quantity || 1), 0),
      addItem: (item) => {
        setItems((prev) => {
          const existing = prev.find((p) => p.id === item.id)
          if (existing) {
            return prev.map((p) => (p.id === item.id ? { ...p, quantity: (p.quantity || 1) + 1 } : p))
          }
          return [...prev, { ...item, quantity: 1 }]
        })
      },
      removeItem: (id) => setItems((prev) => prev.filter((p) => p.id !== id)),
      clear: () => setItems([]),
    }),
    [items]
  )

  return <CartContext.Provider value={api}>{children}</CartContext.Provider>
}

export function useCart() {
  return useContext(CartContext)
}

