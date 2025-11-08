import React, { useState } from 'react';
import { Link } from 'react-router-dom';

function Login() {
  const [email, setEmail] = useState('');
  const [password, setPassword] = useState('');

  const handleSubmit = (e) => {
    e.preventDefault();
    // Handle login logic here - will be integrated with backend API later
    // TODO: Integrate with backend API endpoint
    // Note: Credentials should be sent securely via HTTPS to backend
    // Never log passwords in production
  };

  return (
    <div className="container mx-auto px-4 py-16">
      <div className="max-w-md mx-auto bg-white border rounded-lg">
        <div className="p-8">
          <h2 className="text-2xl font-bold mb-2">Login</h2>
          <p className="text-gray-600 mb-6">
            Enter your credentials to access your account
          </p>
          
          <form onSubmit={handleSubmit}>
            <div className="space-y-4">
              <div>
                <label htmlFor="email" className="block text-sm font-medium text-gray-700 mb-1">
                  Email
                </label>
                <input
                  id="email"
                  type="email"
                  placeholder="john.doe@example.com"
                  value={email}
                  onChange={(e) => setEmail(e.target.value)}
                  required
                  className="w-full px-3 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500"
                />
              </div>
              
              <div>
                <div className="flex items-center justify-between mb-1">
                  <label htmlFor="password" className="block text-sm font-medium text-gray-700">
                    Password
                  </label>
                  <Link to="#" className="text-sm text-blue-600 hover:underline">
                    Forgot password?
                  </Link>
                </div>
                <input
                  id="password"
                  type="password"
                  placeholder="••••••••"
                  value={password}
                  onChange={(e) => setPassword(e.target.value)}
                  required
                  className="w-full px-3 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500"
                />
              </div>
            </div>
            
            <div className="mt-6 space-y-4">
              <button
                type="submit"
                className="w-full px-6 py-3 bg-blue-600 text-white rounded-lg hover:bg-blue-700 transition-colors"
              >
                Sign In
              </button>
              
              <div className="text-sm text-center text-gray-600">
                Don&apos;t have an account?{' '}
                <Link to="/register" className="text-blue-600 hover:underline">
                  Sign up
                </Link>
              </div>
            </div>
          </form>
        </div>
      </div>
    </div>
  );
}

export default Login;
